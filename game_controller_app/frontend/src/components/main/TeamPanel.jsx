import ActionButton from "./ActionButton";
import PlayerButton from "./PlayerButton";
import { applyAction } from "../../api.js";

const TeamPanel = ({
  connectionStatus,
  game,
  params,
  selectedPenaltyCall,
  setSelectedPenaltyCall,
  side,
  sign,
}) => {
  const team = game.teams[side];
  const teamConnectionStatus = connectionStatus[side];
  const teamParams = params.game.teams[side];
  const handlePlayerClick = (player) => {
    if (selectedPenaltyCall) {
      applyAction({
        type: "penalize",
        args: { side: side, player: player.number, call: selectedPenaltyCall },
      });
      if (selectedPenaltyCall != "motionInSet") {
        setSelectedPenaltyCall(null);
      }
    } else {
      applyAction({
        type: "unpenalize",
        args: { side: side, player: player.number },
      });
    }
  };
  return (
    <div className="flex flex-col gap-2">
      <div className="flex items-center justify-center gap-2">
        <svg
          className={`${game.kickingSide === side ? "" : "invisible"} text-black`}
          fill="currentColor"
          height="14"
          width="14"
        >
          <circle cx="7" cy="7" r="7" />
        </svg>
        <h1 className="text-center text-2xl font-semibold">{side}</h1>
      </div>
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} gap-2`}>
        <div className="flex flex-col gap-2 flex-1">
          <ActionButton
            action={{ type: "timeout", args: { side: side } }}
            active={game.state === "timeout" && game.kickingSide != side}
            label="Timeout"
          />
          <ActionButton
            action={{ type: "globalGameStuck", args: { side: side } }}
            label="Global Game Stuck"
          />
        </div>
        <div className="flex-1">
          <ActionButton action={{ type: "goal", args: { side: side } }} label="+" />
        </div>
        <dl className="flex-1">
          <dt className="sr-only">Score</dt>
          <dd
            className={`font-bold text-4xl ${sign > 0 ? "text-right" : "text-left"} tabular-nums`}
          >
            {team.score}
          </dd>

          <dt>Penalties</dt>
          <dd className="tabular-nums">{team.penaltyCounter}</dd>

          {game.phase === "penaltyShootout" ? (
            <>
              <dt>Penalty Shot{game.kickingSide === side ? "" : "s"}</dt>
              <dd className="tabular-nums">{team.penaltyShot}</dd>
            </>
          ) : (
            <>
              <dt>Messages</dt>
              <dd className="tabular-nums">{team.messageBudget}</dd>
            </>
          )}
        </dl>
      </div>
      {team.players
        .map((player, index) => {
          return {
            ...player,
            connectionStatus: teamConnectionStatus[index],
            number: index + 1,
          };
        })
        .filter((player) => player.penalty != "substitute")
        .map((player) => (
          <PlayerButton
            key={player.number}
            color={
              player.number == team.goalkeeper
                ? teamParams.goalkeeperColor
                : teamParams.fieldPlayerColor
            }
            onClick={() => handlePlayerClick(player)}
            player={player}
          />
        ))}
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} gap-2`}>
        <ActionButton
          action={{ type: "startSetPlay", args: { side: side, setPlay: "goalKick" } }}
          active={game.setPlay === "goalKick" && game.kickingSide === side}
          label="Goal Kick"
        />
        <ActionButton
          action={{ type: "startSetPlay", args: { side: side, setPlay: "kickIn" } }}
          active={game.setPlay === "kickIn" && game.kickingSide === side}
          label="Kick-in"
        />
        <ActionButton
          action={{ type: "startSetPlay", args: { side: side, setPlay: "cornerKick" } }}
          active={game.setPlay === "cornerKick" && game.kickingSide === side}
          label="Corner Kick"
        />
      </div>
    </div>
  );
};

export default TeamPanel;
