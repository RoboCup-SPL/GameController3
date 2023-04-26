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

const PlayerButton = ({ color, legal, sign, onClick, player }) => {
  return (
    <button
      className={`grow rounded-md border border-gray-600 ${bgClasses[color]} ${
        legal ? "" : "text-gray-500"
      } ${
        player.penaltyTimer.started && player.penaltyTimer.started.remaining[0] < 10
          ? "animate-flash-bg"
          : ""
      }`}
      disabled={!legal}
      onClick={onClick}
    >
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} items-center gap-4 px-4`}>
        <div className="grow flex flex-col">
          <p>{color.charAt(0).toUpperCase() + color.slice(1)}</p>
          <p
            className={player.penaltyTimer.started || player.penalty != "noPenalty" ? "" : "invisible"}
          >
            {player.penaltyTimer.started ? formatMMSS(player.penaltyTimer) : "P"}
          </p>
        </div>
        <svg
          className={
            player.connectionStatus >= 2
              ? "text-green-600"
              : player.connectionStatus >= 1
              ? "text-yellow-400"
              : "text-red-600"
          }
          fill="currentColor"
          height="14"
          width="14"
        >
          <circle cx="7" cy="7" r="7" />
        </svg>
        <p className="text-3xl tabular-nums">{player.number}</p>
      </div>
    </button>
  );
};

export default PlayerButton;
