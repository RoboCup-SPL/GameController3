import { formatMMSS } from "../../utils.js";

const bgClasses = {
  red: "bg-red-100",
  blue: "bg-blue-100",
  yellow: "bg-yellow-100",
  black: "bg-white",
  white: "bg-white",
  green: "bg-green-100",
  orange: "bg-orange-100",
  purple: "bg-purple-100",
  brown: "bg-amber-100",
  gray: "bg-gray-200",
};

const PlayerButton = ({ color, onClick, player }) => {
  return (
    <button
      className={`h-16 rounded-md border border-gray-600 ${bgClasses[color]} ${
        player.penalty === "noPenalty" ? "" : "text-gray-300"
      }`}
      onClick={onClick}
    >
      <div>
        {color.charAt(0).toUpperCase() + color.slice(1)} {player.number}
      </div>
      <div className={player.penaltyTimer.started ? "" : "invisible"}>
        {formatMMSS(player.penaltyTimer)}
      </div>
    </button>
  );
};

export default PlayerButton;
