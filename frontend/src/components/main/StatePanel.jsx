import ActionButton from "./ActionButton";
import * as actions from "../../actions.js";

const StatePanel = ({ game, legalGameActions }) => {
  return (
    <div className="flex flex-row gap-2">
      <ActionButton
        active={game.state === "initial" || game.state === "timeout"}
        label="Initial"
        legal={false}
      />
      <ActionButton
        action={{ type: "startSetPlay", args: { side: game.kickingSide, setPlay: "kickOff" } }}
        active={game.state === "ready"}
        label="Ready"
        legal={
          legalGameActions[
            game.kickingSide === "home" ? actions.START_KICK_OFF_HOME : actions.START_KICK_OFF_AWAY
          ]
        }
      />
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "waitForPenaltyShot", args: null }
            : { type: "waitForSetPlay", args: null }
        }
        active={game.state === "set"}
        label="Set"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout"
              ? actions.WAIT_FOR_PENALTY_SHOT
              : actions.WAIT_FOR_SET_PLAY
          ]
        }
      />
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "freePenaltyShot", args: null }
            : game.state === "playing"
            ? { type: "finishSetPlay", args: null }
            : { type: "freeSetPlay", args: null }
        }
        active={game.state === "playing"}
        label="Playing"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout"
              ? actions.FREE_PENALTY_SHOT
              : game.state === "playing"
              ? actions.FINISH_SET_PLAY
              : actions.FREE_SET_PLAY
          ]
        }
      />
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "finishPenaltyShot", args: null }
            : { type: "finishHalf", args: null }
        }
        active={game.state === "finished"}
        label="Finish"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout" ? actions.FINISH_PENALTY_SHOT : actions.FINISH_HALF
          ]
        }
      />
    </div>
  );
};

export default StatePanel;
