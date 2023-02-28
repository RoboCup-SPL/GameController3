//! This module defines the main runtime of the GameController application.

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    net::{IpAddr, Ipv4Addr},
    path::Path,
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use serde::Serialize;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tokio::{
    fs::create_dir_all,
    select,
    sync::{broadcast, mpsc, watch, Mutex, Notify},
    task::JoinSet,
    time::{sleep_until, Instant},
};
use tokio_util::sync::CancellationToken;

use game_controller::{
    action::VAction,
    actions::TeamMessage,
    log::{LogEntry, LoggedMetadata, LoggedMonitorRequest, LoggedStatusMessage, LoggedTeamMessage},
    timer::EvaluatedRunConditions,
    types::{ActionSource, Game, GameParams, Params, Side},
    GameController,
};
use game_controller_msgs::MonitorRequest;
use game_controller_net::{
    run_control_message_sender, run_monitor_request_receiver, run_status_message_forwarder,
    run_status_message_receiver, run_team_message_receiver, Event,
};

use crate::launch::{LaunchSettings, NetworkInterface, Team};
use crate::logger::FileLogger;

/// This struct represents the state that is sent to the UI.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiState {
    /// The game state.
    game: Game,
}

/// This struct encapsulates state that must be mutated.
struct MutableState {
    /// The join set of main runtime tasks (event loop and logger).
    runtime_join_set: JoinSet<Result<()>>,
    /// The join set of network tasks.
    network_join_set: JoinSet<()>,
}

/// This struct represents the state that exposes GameController services to the UI.
pub struct RuntimeState {
    /// The sender for actions to the runtime.
    pub action_sender: mpsc::UnboundedSender<VAction>,
    /// The notify object with which the UI tells the runtime thread that it can start its loop.
    pub ui_notify: Arc<Notify>,
    /// The sender for the shutdown signal.
    shutdown_token: CancellationToken,
    /// The mutable state behind a mutex. It is a tokio mutex because it is held across await.
    mutable_state: Mutex<MutableState>,
}

/// This function starts all network services that are not tied to a specific monitor. It returns a
/// receiver for incoming network events, a sender for the game state (that will be published to the
/// players) and a join set in which all tasks were spawned.
fn start_network(
    initial_game: Game,
    params: Params,
    broadcast_address: IpAddr,
    local_address: IpAddr,
    multicast: bool,
    teams: Vec<u8>,
) -> (
    mpsc::UnboundedReceiver<Event>,
    watch::Sender<Game>,
    JoinSet<()>,
) {
    let (event_sender, event_receiver) = mpsc::unbounded_channel();
    let (control_sender, control_receiver) = watch::channel(initial_game);

    let mut join_set = JoinSet::new();

    join_set.spawn(async move {
        run_control_message_sender(broadcast_address, params, control_receiver, false)
            .await
            .unwrap()
    });

    teams.into_iter().for_each(|team| {
        let team_sender = event_sender.clone();
        join_set.spawn(async move {
            run_team_message_receiver(local_address, multicast, team, team_sender)
                .await
                .unwrap()
        });
    });

    let status_sender = event_sender.clone();
    join_set.spawn(async move {
        run_status_message_receiver(local_address, status_sender)
            .await
            .unwrap()
    });

    join_set.spawn(async move {
        run_monitor_request_receiver(local_address, event_sender)
            .await
            .unwrap()
    });

    (event_receiver, control_sender, join_set)
}

/// This function is the interfaces the GameController to external events. Each loop iteration
/// consists of three parts: First, the current state is sent to the UI and published for network
/// senders. Then, given the current state, a timestamp is calculated at which the next externally
/// visible state change happens. Finally, the next event is awaited, which can be either that the
/// previously calculated deadline was reached, an incoming network event, an action from the UI,
/// or a shutdown request.
async fn event_loop(
    mut game_controller: GameController,
    mut event_receiver: mpsc::UnboundedReceiver<Event>,
    mut action_receiver: mpsc::UnboundedReceiver<VAction>,
    ui_notify: Arc<Notify>,
    shutdown_token: CancellationToken,
    control_sender: watch::Sender<Game>,
    send_ui_state: Box<dyn Fn(UiState) -> Result<()> + Send>,
) -> Result<()> {
    let mut last = Instant::now();
    let mut monitors = HashMap::<IpAddr, JoinSet<Result<()>>>::new();
    let mut players = HashSet::<IpAddr>::new();
    let (status_forward_sender, _) = broadcast::channel(16);

    // We must wait for the main window before sending the first UI state.
    ui_notify.notified().await;

    loop {
        send_ui_state(UiState {
            game: game_controller.game.clone(),
        })?;
        control_sender.send(game_controller.game.clone())?;

        let run_conditions =
            EvaluatedRunConditions::new(&game_controller.game, &game_controller.params);
        let dt = game_controller.clip_next_timer_wrap(
            &run_conditions,
            game_controller.clip_next_timer_expiration(&run_conditions, Duration::MAX),
        );

        let deadline = last.checked_add(dt);

        select! {
            // We can't use deadline.unwrap() because it's still evaluated even if the branch is
            // disabled. Therefore we supply some Instant that we have already lying around.
            _ = sleep_until(deadline.unwrap_or(last)), if deadline.is_some() => {
                let now = Instant::now();
                game_controller.seek(now - last);
                last = now;
            },
            event = event_receiver.recv() => {
                let now = Instant::now();
                game_controller.seek(now - last);
                last = now;
                match event {
                    Some(Event::MonitorRequest { host, data, .. }) => {
                        game_controller.log_now(LogEntry::MonitorRequest(LoggedMonitorRequest {
                            host,
                            data: data.to_vec(),
                        }));
                        // Requests are ignore if they come from hosts that have previously sent
                        // status messages, the monitor request is not ill-formed, or the host is
                        // already registered as a monitor.
                        if !players.contains(&host) && MonitorRequest::try_from(data).is_ok()
                            && !monitors.contains_key(&host)
                        {
                            let mut monitor_join_set = JoinSet::new();
                            monitor_join_set.spawn(run_control_message_sender(
                                host,
                                game_controller.params.clone(),
                                // TODO: use the sender with "true" data here
                                control_sender.subscribe(),
                                true
                            ));
                            monitor_join_set.spawn(run_status_message_forwarder(
                                host,
                                status_forward_sender.subscribe()
                            ));
                            monitors.insert(host, monitor_join_set);
                        }
                    },
                    Some(Event::StatusMessage { host, data, .. }) => {
                        game_controller.log_now(LogEntry::StatusMessage(LoggedStatusMessage {
                            host,
                            data: data.to_vec(),
                        }));
                        // If the host (which is now presumed to be a player) had previously sent a
                        // monitor request, it must not get any true data anymore.
                        if let Some(mut monitor_state) = monitors.remove(&host) {
                            monitor_state.abort_all();
                        }
                        players.insert(host);
                        // Status messages are forwarded to monitors even if they are ill-formed
                        // because then the monitor can display this fact. We must ignore errors
                        // here because it is possible that nobody is subscribed at the moment.
                        let _ = status_forward_sender.send((host, data.clone()));
                    },
                    Some(Event::TeamMessage { host, team, data, too_long }) => {
                        game_controller.log_now(LogEntry::TeamMessage(LoggedTeamMessage {
                            team,
                            host,
                            data: data.to_vec(),
                        }));
                        game_controller.apply(VAction::TeamMessage(TeamMessage {
                            // We only started a team message receiver for the two playing teams,
                            // so the unwrap is justified.
                            side: game_controller.params.game.get_side(team).unwrap(),
                            illegal: too_long,
                        }), ActionSource::Network);
                    },
                    _ => {},
                }
            },
            action = action_receiver.recv() => {
                let now = Instant::now();
                game_controller.seek(now - last);
                last = now;
                if let Some(action) = action {
                    game_controller.apply(action, ActionSource::User);
                }
            },
            _ = shutdown_token.cancelled() => {
                for mut monitor_state in monitors.into_values() {
                    monitor_state.shutdown().await;
                }
                // This last seek is done so that the end timestamp in the log is more accurate
                // (the end entry is added when the GameController is dropped).
                game_controller.seek(Instant::now() - last);
                return Ok(());
            },
        };
    }
}

/// This function starts all runtime tasks.
pub async fn start_runtime(
    config_directory: &Path,
    log_directory: &Path,
    settings: &LaunchSettings,
    teams: &[Team],
    network_interfaces: &[NetworkInterface],
    send_ui_state: Box<dyn Fn(UiState) -> Result<()> + Send>,
) -> Result<RuntimeState> {
    let mut runtime_join_set = JoinSet::new();

    let params = Params {
        competition: serde_yaml::from_reader(File::open(
            config_directory
                .join(&settings.competition.id)
                .join("params.yaml"),
        )?)?,
        game: GameParams {
            teams: settings.teams.clone(),
            long: settings.competition.play_off,
        },
    };

    let team_names = settings.teams.clone().map(|_side, team| {
        teams
            .iter()
            .find(|t| team.number == t.number)
            .unwrap()
            .name
            .replace(' ', "-")
    });
    let date_time = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    create_dir_all(log_directory).await?;
    let logger = FileLogger::new(
        log_directory.join(format!(
            "log_{}_{}_{}.yaml",
            team_names[Side::Home],
            team_names[Side::Away],
            date_time.format(&Rfc3339).unwrap(),
        )),
        &mut runtime_join_set,
        false, // TODO: This should be true at actual competitions so that logs are always
               // recoverable
    )
    .await?;

    let mut game_controller = GameController::new(params.clone(), Box::new(logger));

    game_controller.log_now(LogEntry::Metadata(LoggedMetadata {
        creator: "GameController".into(),
        version: 1,
        timestamp: date_time,
        params: Box::new(params),
    }));

    let network_interface = network_interfaces
        .iter()
        .find(|network_interface| network_interface.id == settings.network.interface)
        .unwrap();

    let (event_receiver, control_sender, network_join_set) = start_network(
        game_controller.game.clone(),
        game_controller.params.clone(),
        if settings.network.broadcast {
            IpAddr::V4(Ipv4Addr::BROADCAST)
        } else {
            network_interface.broadcast
        },
        network_interface.address,
        settings.network.multicast,
        settings.teams.values().map(|team| team.number).collect(),
    );

    let (action_sender, action_receiver) = mpsc::unbounded_channel();
    let ui_notify = Arc::new(Notify::new());
    let shutdown_token = CancellationToken::new();

    runtime_join_set.spawn(event_loop(
        game_controller,
        event_receiver,
        action_receiver,
        ui_notify.clone(),
        shutdown_token.clone(),
        control_sender,
        send_ui_state,
    ));

    Ok(RuntimeState {
        action_sender,
        ui_notify,
        shutdown_token,
        mutable_state: Mutex::new(MutableState {
            runtime_join_set,
            network_join_set,
        }),
    })
}

/// This function tells the runtime to shut down and waits until all tasks have finished.
pub async fn shutdown_runtime(runtime_state: &RuntimeState) {
    runtime_state.shutdown_token.cancel();

    let mut mutable_state = runtime_state.mutable_state.lock().await;

    // There are two tasks in this join set. If there have not been any errors during logging, the
    // event loop will finish first because the logger still waits on the channel. But once the
    // event loop returns, the channel is dropped, and after writing the last log entry, the logger
    // will return, too.
    while mutable_state.runtime_join_set.join_next().await.is_some() {}

    mutable_state.network_join_set.shutdown().await;
}
