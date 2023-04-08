import ActionButton from "./ActionButton";

const PhasePanel = ({ game }) => {
  return (
    <div className="flex flex-row gap-2">
      <ActionButton active={game.phase === "firstHalf"} label="First Half" />
      <ActionButton
        action={{ type: "switchHalf", args: null }}
        active={game.phase === "secondHalf"}
        label="Second Half"
      />
      <ActionButton
        action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } }}
        active={game.phase === "penaltyShootout"}
        label="Penalty Shots"
      />
    </div>
  );
};

export default PhasePanel;
