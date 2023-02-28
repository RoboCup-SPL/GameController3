import PenaltyButton from "./PenaltyButton";

const penalties = [
  ["Pushing", "pushing"],
  ["Foul", "foul"],
  ["Fallen / Inactive", "fallenInactive"],
  ["Leaving the Field", "leavingTheField"],
  ["Motion in Set", "motionInSet"],
  ["Illegal Position", "illegalPosition"],
  ["Ball Holding", "ballHolding"],
  ["Penalty Kick", "penaltyKick"],
  ["Local Game Stuck", "localGameStuck"],
  ["Pick-Up", "requestForPickUp"],
  ["Player Stance", "playerStance"],
  ["Arms / Hands", "playingWithArmsHands"],
];

const PenaltyPanel = ({ selectedPenaltyCall, setSelectedPenaltyCall }) => {
  return (
    <div className="grid grid-cols-2 gap-2">
      {penalties.map((penalty) => (
        <PenaltyButton
          key={penalty[1]}
          label={penalty[0]}
          onClick={() =>
            setSelectedPenaltyCall(selectedPenaltyCall === penalty[1] ? null : penalty[1])
          }
          selected={selectedPenaltyCall === penalty[1]}
        />
      ))}
    </div>
  );
};

export default PenaltyPanel;
