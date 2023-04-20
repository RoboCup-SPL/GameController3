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

const PlayerButton = ({ color, legal, onClick, player }) => {
  return (
    <button
      className={`grow rounded-md border border-gray-600 ${bgClasses[color]} ${
        player.penalty === "noPenalty" ? "" : "text-gray-300"
      }`}
      disabled={!legal}
      onClick={onClick}
    >
      <div className="flex items-center justify-center gap-2">
        <svg
          className={
            player.connectionStatus >= 2
              ? "text-green-600"
              : player.connectionStatus >= 1
              ? "text-yellow-400"
              : "text-red-600"
          }
          fill="currentColor"
          height="10"
          width="10"
        >
          <circle cx="5" cy="5" r="5" />
        </svg>
        {color.charAt(0).toUpperCase() + color.slice(1)} {player.number}
      </div>
      <div className={player.penaltyTimer.started ? "" : "invisible"}>
        {formatMMSS(player.penaltyTimer)}
      </div>
    </button>
  );
};

export default PlayerButton;
