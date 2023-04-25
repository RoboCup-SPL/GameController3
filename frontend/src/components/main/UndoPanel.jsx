import ActionButton from "./ActionButton";

const getActionName = (action) => {
  switch (action.type) {
    case "addExtraTime":
      return "Add Extra Time";
    case "finishHalf":
    case "finishPenaltyShot":
      return "Finish";
    case "finishSetPlay":
      return "Set Play Complete";
    case "freePenaltyShot":
    case "freeSetPlay":
      return "Playing";
    case "globalGameStuck":
      return "Global Game Stuck";
    case "goal":
      return "Goal";
    case "penalize":
      return "Penalize";
    case "selectPenaltyShotPlayer":
      return "Select";
    case "startPenaltyShootout":
      return "Penalty Shoot-out";
    case "startSetPlay":
      switch (action.args.setPlay) {
        case "kickOff":
          return "Ready";
        case "kickIn":
          return "Kick-in";
        case "goalKick":
          return "Goal Kick";
        case "cornerKick":
          return "Corner Kick";
      }
      break;
    case "substitute":
      return "Substitute";
    case "switchHalf":
      return "Second Half";
    case "timeout":
      return action.args.side ? "Timeout" : "Referee Timeout";
    case "unpenalize":
      return "Unpenalize";
    case "waitForPenaltyShot":
    case "waitForSetPlay":
      return "Set";
  }
  return action.type;
};

const UndoPanel = ({ undoActions, legalUndoActions }) => {
  return (
    <div className="flex flex-row-reverse gap-2 h-10">
      {legalUndoActions.map((legal, index) => (
        <div className={`w-1/5`} key={index}>
          <ActionButton
            action={{ type: "undo", args: { states: index + 1 } }}
            label={index < undoActions.length ? getActionName(undoActions[index]) : "Undo"}
            legal={legal}
          />
        </div>
      ))}
    </div>
  );
};

export default UndoPanel;
