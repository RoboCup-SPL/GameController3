import ActionButton from "./ActionButton";

const PhasePanel = ({ game }) => {
  return (
    <div className="flex flex-row gap-2">
      <ActionButton active={game.phase === "firstHalf"} label="First Half" />
      <ActionButton
        action={{
          type: "switchHalf",
          args: null,
        }}
        active={game.phase === "secondHalf"}
        label="Second Half"
      />
    </div>
  );
};

export default PhasePanel;
