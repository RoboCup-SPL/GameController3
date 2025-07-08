import { CheckIcon, ExclamationTriangleIcon, XMarkIcon } from "@heroicons/react/24/solid";
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

const penaltyDescriptions = {
  noPenalty: "No Penalty",
  substitute: "Substitute",
  pickedUp: "Picked Up",
  illegalPositionInSet: "Illegal Position",
  illegalPosition: "Illegal Position",
  motionInSet: "Motion in Set",
  fallenInactive: "Fallen / Inactive",
  localGameStuck: "Local Game Stuck",
  ballHolding: "Ball Holding",
  playerStance: "Player Stance",
  playerPushing: "Pushing",
  playingWithArmsHands: "Arms / Hands",
  leavingTheField: "Leaving the Field",
};

const ConnectionStatusIndicatorIcon = ({ connectionStatus }) => {
  if (connectionStatus >= 2) {
    return <CheckIcon className="w-6 h-6 text-green-600" />;
  }

  if (connectionStatus >= 1) {
    return <ExclamationTriangleIcon className="w-6 h-6 text-yellow-400" />;
  }

  return <XMarkIcon className="w-6 h-6 text-red-600" />;
};

const PlayerButton = ({ color, legal, sign, onClick, player }) => {
  const shouldFlash =
    player &&
    player.penalty != "noPenalty" &&
    player.penalty != "substitute" &&
    player.penalty != "motionInSet" &&
    (player.penaltyTimer.started
      ? player.penaltyTimer.started.remaining[0] < 10
      : player.penalty != "pickedUp");

  return (
    <button
      className={`grow rounded-md border border-gray-600 ${bgClasses[color]} ${
        shouldFlash ? "animate-flash-bg" : ""
      } ${legal ? "" : "text-gray-500"}`}
      disabled={!legal}
      onClick={onClick}
    >
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} items-center gap-4 px-4`}>
        <div className="grow flex flex-col">
          <p>{color.charAt(0).toUpperCase() + color.slice(1)}</p>
          {player ? (
            <p
              className={
                player.penaltyTimer.started
                  ? "tabular-nums"
                  : player.penalty === "noPenalty"
                  ? "invisible"
                  : ""
              }
            >
              {player.penaltyTimer.started
                ? formatMMSS(player.penaltyTimer)
                : penaltyDescriptions[player.penalty]}
            </p>
          ) : (
            <></>
          )}
        </div>
        {player ? (
          <>
            <ConnectionStatusIndicatorIcon connectionStatus={player.connectionStatus} />
            <p className="text-3xl tabular-nums">{player.number}</p>
          </>
        ) : (
          <></>
        )}
      </div>
    </button>
  );
};

export default PlayerButton;
