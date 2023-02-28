import { formatMMSS } from "../../utils.js";

const PlayerButton = ({ player, onClick }) => {
  return (
    <button
      className={`h-16 rounded-md border border-gray-600 ${
        player.penalty === "noPenalty" ? "" : "text-gray-300"
      }`}
      onClick={onClick}
    >
      <div>{player.number}</div>
      <div className={player.penaltyTimer.started ? "" : "invisible"}>
        {formatMMSS(player.penaltyTimer)}
      </div>
    </button>
  );
};

export default PlayerButton;
