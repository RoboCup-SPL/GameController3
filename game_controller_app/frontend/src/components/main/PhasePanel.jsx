import ActionButton from "./ActionButton";
import * as actions from "../../actions.js";

const PhasePanel = ({ game, legalGameActions }) => {
  return (
    <div className="flex flex-row gap-2">
      <ActionButton active={game.phase === "firstHalf"} label="First Half" legal={false} />
      <ActionButton
        action={{ type: "switchHalf", args: null }}
        active={game.phase === "secondHalf"}
        label="Second Half"
        legal={legalGameActions[actions.SWITCH_HALF]}
      />
      <ActionButton
        action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } }}
        active={game.phase === "penaltyShootout"}
        label="Penalty Shots"
        legal={legalGameActions[actions.START_PENALTY_SHOOTOUT]}
      />
    </div>
  );
};

export default PhasePanel;
