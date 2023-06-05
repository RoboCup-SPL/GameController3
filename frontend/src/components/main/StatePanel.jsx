import ActionButton from "./ActionButton";
import * as actions from "../../actions.js";

const StatePanel = ({ game, legalGameActions }) => {
  let readyButton =
    game.phase != "penaltyShootout" &&
    (game.state === "initial" ||
      game.state === "timeout" ||
      (game.phase === "firstHalf" && game.state === "finished")) ? (
      <ActionButton
        action={{ type: "startSetPlay", args: { side: game.kickingSide, setPlay: "kickOff" } }}
        label="Ready"
        legal={
          legalGameActions[
            game.kickingSide === "home" ? actions.START_KICK_OFF_HOME : actions.START_KICK_OFF_AWAY
          ]
        }
      />
    ) : (
      <></>
    );

  let setButton =
    game.phase === "penaltyShootout" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "waitForPenaltyShot", args: null }
            : { type: "waitForSetPlay", args: null }
        }
        label="Set"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout"
              ? actions.WAIT_FOR_PENALTY_SHOT
              : actions.WAIT_FOR_SET_PLAY
          ]
        }
      />
    ) : (
      <></>
    );

  let playingButton =
    game.phase === "penaltyShootout" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "freePenaltyShot", args: null }
            : { type: "freeSetPlay", args: null }
        }
        label={"Playing"}
        legal={
          legalGameActions[
            game.phase === "penaltyShootout" ? actions.FREE_PENALTY_SHOT : actions.FREE_SET_PLAY
          ]
        }
      />
    ) : (
      <></>
    );

  let ballFreeButton =
    game.phase != "penaltyShootout" &&
    (game.state === "ready" || game.state === "set" || game.state === "playing") ? (
      <ActionButton
        action={{ type: "finishSetPlay", args: null }}
        label={"Ball Free"}
        legal={legalGameActions[actions.FINISH_SET_PLAY]}
      />
    ) : (
      <></>
    );

  let finishButton =
    game.phase === "penaltyShootout" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "finishPenaltyShot", args: null }
            : { type: "finishHalf", args: null }
        }
        label="Finish"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout" ? actions.FINISH_PENALTY_SHOT : actions.FINISH_HALF
          ]
        }
      />
    ) : (
      <></>
    );

  // This button is still displayed when we are already in the Initial state of the second half.
  // This is because the state can switch automatically to the second half (actually not yet) and
  // it would be bad if the operator clicked the button exactly at that time, but the button
  // switches its meaning to Ready before the button is actually clicked. Therefore, both buttons
  // (Ready and Second Half) are displayed during the entire half-time break, even though only one
  // of them can be legal.
  let secondHalfButton =
    (game.phase === "firstHalf" && game.state === "finished") ||
    (game.phase === "secondHalf" && game.state === "initial") ? (
      <ActionButton
        action={{ type: "switchHalf", args: null }}
        label="Second Half"
        legal={legalGameActions[actions.SWITCH_HALF]}
      />
    ) : (
      <></>
    );

  let penaltyShootoutButton =
    game.phase === "secondHalf" && game.state === "finished" ? (
      <ActionButton
        action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } }}
        label="Penalty Shoot-out"
        legal={legalGameActions[actions.START_PENALTY_SHOOTOUT]}
      />
    ) : (
      <></>
    );

  let refereeTimeoutButton = (
    <ActionButton
      action={{ type: "timeout", args: { side: null } }}
      label="Referee Timeout"
      legal={legalGameActions[actions.REFEREE_TIMEOUT]}
    />
  );

  return (
    <div className="flex flex-row gap-2">
      {secondHalfButton}
      {penaltyShootoutButton}
      {readyButton}
      {setButton}
      {playingButton}
      {ballFreeButton}
      {finishButton}
      {refereeTimeoutButton}
    </div>
  );
};

export default StatePanel;
