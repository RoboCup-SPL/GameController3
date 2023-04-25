import { formatMMSS } from "../../utils.js";

const getStateDescription = (game) => {
  if (game.state === "timeout") {
    return "Timeout";
  } else if (
    (game.phase === "firstHalf" && game.state === "finished") ||
    (game.phase === "secondHalf" && game.state === "initial")
  ) {
    return "Half-Time Break";
  }
  switch (game.setPlay) {
    case "kickOff":
      return "Kick-off";
    case "kickIn":
      return "Kick-in";
    case "goalKick":
      return "Goal Kick";
    case "cornerKick":
      return "Corner Kick";
    case "pushingFreeKick":
      return "Pushing Free Kick";
    case "penaltyKick":
      return "Penalty Kick";
  }
  return "";
};

const ClockPanel = ({ game }) => {
  return (
    <div className="flex flex-col items-center">
      <p
        className={`tabular-nums text-8xl font-medium ${
          game.primaryTimer.started
            ? game.primaryTimer.started.remaining[0] < 10
              ? "animate-flash-text"
              : ""
            : "invisible"
        }`}
      >
        {formatMMSS(game.primaryTimer)}
      </p>
      <p className={`tabular-nums text-2xl ${game.secondaryTimer.started ? "" : "invisible"}`}>
        {formatMMSS(game.secondaryTimer)}
      </p>
      <p className="h-6">{getStateDescription(game)}</p>
    </div>
  );
};

export default ClockPanel;
