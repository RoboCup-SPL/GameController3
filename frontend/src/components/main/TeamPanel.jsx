import { useState } from "react";
import ActionButton from "./ActionButton";
import PlayerButton from "./PlayerButton";
import * as actions from "../../actions.js";
import { applyAction } from "../../api.js";

const textClasses = {
  red: "text-red-600",
  blue: "text-blue-600",
  yellow: "text-yellow-400",
  black: "text-black",
  white: "text-black",
  green: "text-green-600",
  orange: "text-orange-400",
  purple: "text-purple-600",
  brown: "text-amber-800",
  gray: "text-gray-600",
};

const TeamPanel = ({
  connectionStatus,
  game,
  legalPenaltyActions,
  legalTeamActions,
  params,
  selectedPenaltyCall,
  setSelectedPenaltyCall,
  side,
  sign,
  teamNames,
}) => {
  const [substitute, setSubstitute] = useState(false);
  const [substitutedPlayer, setSubstitutedPlayer] = useState(null);

  const selectingPlayerIn = substitute && substitutedPlayer != null;
  const selectingPlayerInPSO = substitute && game.phase === "penaltyShootout";

  const team = game.teams[side];
  const teamConnectionStatus = connectionStatus[side];
  const teamParams = params.game.teams[side];
  const handlePlayerClick = (player) => {
    if (selectingPlayerInPSO) {
      applyAction({
        type: "selectPenaltyShotPlayer",
        args: { side: side, player: player.number, goalkeeper: side != game.kickingSide /*TODO*/ },
      });
      setSubstitute(false);
      setSubstitutedPlayer(null);
    } else if (selectingPlayerIn) {
      applyAction({
        type: "substitute",
        args: { side: side, playerOut: substitutedPlayer, playerIn: player.number },
      });
      setSubstitute(false);
      setSubstitutedPlayer(null);
    } else if (substitute) {
      setSubstitutedPlayer(player.number);
    } else if (selectedPenaltyCall != null) {
      applyAction({
        type: "penalize",
        args: {
          side: side,
          player: player.number,
          call: actions.PENALTIES[selectedPenaltyCall][1],
        },
      });
      if (actions.PENALTIES[selectedPenaltyCall][1] != "motionInSet") {
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
    <div className="min-w-80 flex flex-col gap-2">
      <div className="flex items-center justify-center gap-2">
        <svg
          className={`${game.kickingSide === side ? "" : "invisible"} text-black`}
          fill="currentColor"
          height="14"
          width="14"
        >
          <circle cx="7" cy="7" r="7" />
        </svg>
        <h1
          className={`text-center text-2xl font-semibold ${
            textClasses[teamParams.fieldPlayerColor]
          }`}
        >
          {teamNames[side]}
        </h1>
      </div>
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} gap-2`}>
        <div className="flex-1 flex flex-col gap-2">
          <ActionButton
            action={() => {
              setSubstitute(!substitute);
              setSubstitutedPlayer(null);
            }}
            active={substitute}
            label={game.phase === "penaltyShootout" ? "Select" : "Substitute"}
            legal={true}
          />
          {game.state != "playing" ? (
            <ActionButton
              action={{ type: "timeout", args: { side: side } }}
              label="Timeout"
              legal={legalTeamActions[actions.TIMEOUT]}
            />
          ) : (
            <ActionButton
              action={{ type: "globalGameStuck", args: { side: side } }}
              label="Global Game Stuck"
              legal={legalTeamActions[actions.GLOBAL_GAME_STUCK]}
            />
          )}
        </div>
        <div className="flex-1">
          <ActionButton
            action={{ type: "goal", args: { side: side } }}
            label="Goal"
            legal={legalTeamActions[actions.GOAL]}
          />
        </div>
        <dl className="flex-1">
          <dt className="sr-only">Score</dt>
          <dd
            className={`font-bold text-4xl ${sign > 0 ? "text-right" : "text-left"} tabular-nums ${
              team.illegalCommunication ? "text-fuchsia-400" : ""
            }`}
          >
            {team.score}
          </dd>

          {game.phase === "penaltyShootout" ? (
            <>
              <dt>Shot{game.kickingSide === side ? "" : "s"}:</dt>
              <dd className="tabular-nums text-right">{team.penaltyShot}</dd>
            </>
          ) : (
            <>
              <dt className={team.illegalCommunication ? "text-fuchsia-400" : ""}>Messages:</dt>
              <dd
                className={`tabular-nums text-right ${
                  team.illegalCommunication ? "text-fuchsia-400" : ""
                }`}
              >
                {team.messageBudget}
              </dd>
            </>
          )}

          <dt>Penalties:</dt>
          <dd className="tabular-nums text-right">{team.penaltyCounter}</dd>
        </dl>
      </div>
      <div className="grow flex flex-col gap-2 overflow-auto">
        {team.players
          .map((player, index) => {
            return {
              ...player,
              connectionStatus: teamConnectionStatus[index],
              number: index + 1,
            };
          })
          .filter(
            selectingPlayerInPSO
              ? () => true
              : selectingPlayerIn
              ? (player) => player.penalty === "substitute"
              : (player) => player.penalty != "substitute"
          )
          .map((player) => (
            <PlayerButton
              key={player.number}
              color={
                (
                  selectingPlayerInPSO
                    ? side != game.kickingSide /*TODO*/
                    : (selectingPlayerIn ? substitutedPlayer : player.number) === team.goalkeeper
                )
                  ? teamParams.goalkeeperColor
                  : teamParams.fieldPlayerColor
              }
              legal={
                substitute ||
                actions.isPenaltyCallLegalForPlayer(
                  legalPenaltyActions,
                  side,
                  player.number,
                  selectedPenaltyCall
                )
              }
              sign={sign}
              onClick={() => handlePlayerClick(player)}
              player={player}
            />
          ))}
      </div>
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} gap-2`}>
        <ActionButton
          action={{ type: "startSetPlay", args: { side: side, setPlay: "goalKick" } }}
          active={game.setPlay === "goalKick" && game.kickingSide === side}
          label="Goal Kick"
          legal={legalTeamActions[actions.GOAL_KICK]}
        />
        <ActionButton
          action={{ type: "startSetPlay", args: { side: side, setPlay: "kickIn" } }}
          active={game.setPlay === "kickIn" && game.kickingSide === side}
          label="Kick-in"
          legal={legalTeamActions[actions.KICK_IN]}
        />
        <ActionButton
          action={{ type: "startSetPlay", args: { side: side, setPlay: "cornerKick" } }}
          active={game.setPlay === "cornerKick" && game.kickingSide === side}
          label="Corner Kick"
          legal={legalTeamActions[actions.CORNER_KICK]}
        />
      </div>
    </div>
  );
};

export default TeamPanel;
