//! This crate defines the main program of the GameController application.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{async_runtime, generate_context, Manager, RunEvent, WindowBuilder, WindowUrl};

mod handlers;
mod launch;
mod logger;
mod runtime;

use handlers::get_invoke_handler;
use launch::make_launch_data;
use runtime::{shutdown_runtime, RuntimeState};

/// This function runs the tauri app. It first parses command line arguments and displays a
/// launcher in which the user can configure the settings for the game. When the user is done with
/// that, the main window and network services are started and shut down when the app is quit.
fn main() {
    // We want to manage an external tokio runtime, mainly to keep dependencies to tauri minimal,
    // but also because I don't know how to do the shutdown correctly otherwise.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    async_runtime::set(runtime.handle().clone());

    let app = tauri::Builder::default()
        .setup(|app| {
            // We don't want to pass errors from command line argument parsing out of this closure
            // (although that would be possible), because that would result in a panic! and we want
            // to gracefully quit (but with nonzero error code) in most cases.
            match app.get_cli_matches() {
                Ok(matches) => {
                    if matches.args.contains_key("help") {
                        println!("{}", matches.args["help"].value.as_str().unwrap());
                        app.handle().exit(0);
                    } else if matches.args.contains_key("version") {
                        println!("{} {}", app.package_info().name, app.package_info().version);
                        app.handle().exit(0);
                    }
                    // TODO: This will probably not work in production.
                    let config_directory = app
                        .path_resolver()
                        .resource_dir()
                        .unwrap()
                        .join("..")
                        .join("..")
                        .join("config");
                    match make_launch_data(&config_directory, matches.args) {
                        Ok(launch_data) => {
                            app.manage(launch_data);
                        }
                        Err(error) => {
                            eprintln!("{error:?}");
                            app.handle().exit(1);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{error:?}");
                    app.handle().exit(1);
                }
            };

            let _launcher =
                WindowBuilder::new(app, "launcher", WindowUrl::App("launcher.html".into()))
                    .center()
                    .inner_size(600.0, 450.0)
                    .resizable(false)
                    .title("GameController")
                    .build()?;
            Ok(())
        })
        .invoke_handler(get_invoke_handler())
        .build(generate_context!())
        .expect("error while running tauri application");

    app.run(move |handle, event| {
        if let RunEvent::Exit = event {
            if let Some(runtime_state) = handle.try_state::<RuntimeState>() {
                runtime.block_on(shutdown_runtime(&runtime_state));
            }
        }
    });
}
