import { formatMMSS } from "../../utils.js";

const ClockPanel = ({ game }) => {
  return (
    <div className="flex flex-col items-center">
      <p
        className={`tabular-nums text-8xl font-medium ${
          game.primaryTimer.started ? "" : "invisible"
        }`}
      >
        {formatMMSS(game.primaryTimer)}
      </p>
      <p className={`tabular-nums text-2xl ${game.secondaryTimer.started ? "" : "invisible"}`}>
        {formatMMSS(game.secondaryTimer)}
      </p>
    </div>
  );
};

export default ClockPanel;
