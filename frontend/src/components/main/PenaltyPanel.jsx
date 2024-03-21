import PenaltyButton from "./PenaltyButton";
import { isPenaltyCallLegal, PENALTIES } from "../../actions.js";

const PenaltyPanel = ({
  game,
  legalPenaltyActions,
  selectedPenaltyCall,
  setSelectedPenaltyCall,
}) => {
  return (
    <div className="grow grid grid-cols-2 gap-2">
      {PENALTIES.map((penalty, index) => [penalty, index])
        .filter(
          (penaltyWithIndex) => penaltyWithIndex[0].length < 3 || penaltyWithIndex[0][2](game)
        )
        .map((penaltyWithIndex) => (
          <PenaltyButton
            key={penaltyWithIndex[0][1]}
            label={penaltyWithIndex[0][0]}
            legal={isPenaltyCallLegal(legalPenaltyActions, penaltyWithIndex[1])}
            onClick={() =>
              setSelectedPenaltyCall(
                selectedPenaltyCall === penaltyWithIndex[1] ? null : penaltyWithIndex[1]
              )
            }
            selected={selectedPenaltyCall === penaltyWithIndex[1]}
          />
        ))}
    </div>
  );
};

export default PenaltyPanel;
