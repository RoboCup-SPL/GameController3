import ClockPanel from "./ClockPanel";
import PenaltyPanel from "./PenaltyPanel";
import PhasePanel from "./PhasePanel";
import StatePanel from "./StatePanel";

const CenterPanel = ({ game, selectedPenaltyCall, setSelectedPenaltyCall }) => {
  return (
    <div className="flex flex-col gap-4">
      <ClockPanel game={game} />
      <PhasePanel game={game} />
      <StatePanel game={game} />
      <PenaltyPanel
        selectedPenaltyCall={selectedPenaltyCall}
        setSelectedPenaltyCall={setSelectedPenaltyCall}
      />
    </div>
  );
};

export default CenterPanel;
