import ClockPanel from "./ClockPanel";
import PenaltyPanel from "./PenaltyPanel";
import StatePanel from "./StatePanel";

const CenterPanel = ({
  game,
  legalGameActions,
  legalPenaltyActions,
  params,
  selectedPenaltyCall,
  setSelectedPenaltyCall,
}) => {
  return (
    <div className="grow flex flex-col gap-4">
      <ClockPanel game={game} legalGameActions={legalGameActions} />
      <StatePanel game={game} params={params} legalGameActions={legalGameActions} />
      <PenaltyPanel
        game={game}
        legalPenaltyActions={legalPenaltyActions}
        selectedPenaltyCall={selectedPenaltyCall}
        setSelectedPenaltyCall={setSelectedPenaltyCall}
      />
    </div>
  );
};

export default CenterPanel;
