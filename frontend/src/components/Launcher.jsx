import { useEffect, useState } from "react";
import CompetitionSettings from "./launcher/CompetitionSettings";
import GameSettings from "./launcher/GameSettings";
import NetworkSettings from "./launcher/NetworkSettings";
import WindowSettings from "./launcher/WindowSettings";
import { getLaunchData, launch } from "../api";

const getLegal = (launchSettings) => {
  if (launchSettings == null) {
    return { isLegal: false, reason: "Launch settings are null." };
  } else {
    let isLegal = true;
    let reason = "";
    if (launchSettings.game.teams.home.number === launchSettings.game.teams.away.number) {
      isLegal = false;
      reason += "Home and away teams must be different.\n";
    }
    if (launchSettings.game.teams.home.fieldPlayerColor === launchSettings.game.teams.away.fieldPlayerColor) {
      isLegal = false;
      reason += "Home and away field player colors must be different.\n";
    }
    if (launchSettings.game.teams.home.fieldPlayerColor === launchSettings.game.teams.home.goalkeeperColor) {
      isLegal = false;
      reason += "Home field player and goalkeeper colors must be different.\n";
    }
    if (launchSettings.game.teams.home.fieldPlayerColor === launchSettings.game.teams.away.goalkeeperColor) {
      isLegal = false;
      reason += "Home field player and away goalkeeper colors must be different.\n";
    }
    if (launchSettings.game.teams.away.fieldPlayerColor === launchSettings.game.teams.away.goalkeeperColor) {
      isLegal = false;
      reason += "Away field player and goalkeeper colors must be different.\n";
    }
    if (launchSettings.game.teams.away.fieldPlayerColor === launchSettings.game.teams.home.goalkeeperColor) {
      isLegal = false;
      reason += "Away field player and home goalkeeper colors must be different.\n";
    }
    return { isLegal: isLegal, reason: reason };
  }
};

const Launcher = ({ setLaunched }) => {
  const [competitions, setCompetitions] = useState(null);
  const [launchSettings, setLaunchSettings] = useState(null);
  const [networkInterfaces, setNetworkInterfaces] = useState(null);
  const [teams, setTeams] = useState(null);

  const { isLegal: launchSettingsAreLegal, reason } = getLegal(launchSettings);

  useEffect(() => {
    getLaunchData().then((data) => {
      setCompetitions(data.competitions);
      setLaunchSettings(data.defaultSettings);
      setNetworkInterfaces(data.networkInterfaces);
      setTeams(data.teams);
    });
  }, []);

  if (
    competitions != null &&
    launchSettings != null &&
    networkInterfaces != null &&
    teams != null
  ) {
    const setCompetition = (competition) => {
      const INVISIBLES_NUMBER = 0;
      const defaultTeam = teams.find((team) => team.number === INVISIBLES_NUMBER); // Assuming that the Invisibles are part of every competition.
      setLaunchSettings({
        ...launchSettings,
        competition: competition,
        game: {
          ...launchSettings.game,
          teams: {
            home: {
              number: defaultTeam.number,
              fieldPlayerColor: defaultTeam.fieldPlayerColors[0],
              goalkeeperColor: defaultTeam.goalkeeperColors[0],
            },
            away: {
              number: defaultTeam.number,
              fieldPlayerColor: defaultTeam.fieldPlayerColors[1],
              goalkeeperColor: defaultTeam.goalkeeperColors[1],
            },
          },
        },
      });
    };
    const thisCompetition = competitions.find(
      (competition) => competition.id === launchSettings.competition.id
    );
    const teamsInThisCompetition = teams.filter((team) =>
      thisCompetition.teams.includes(team.number)
    );
    return (
      <div className="flex flex-col items-center p-4 gap-2">
        <CompetitionSettings
          competitions={competitions}
          competition={launchSettings.competition}
          setCompetition={setCompetition}
        />
        <GameSettings
          teams={teamsInThisCompetition}
          game={launchSettings.game}
          setGame={(game) => setLaunchSettings({ ...launchSettings, game: game })}
        />
        <WindowSettings
          window={launchSettings.window}
          setWindow={(window) => setLaunchSettings({ ...launchSettings, window: window })}
        />
        <NetworkSettings
          interfaces={networkInterfaces}
          network={launchSettings.network}
          setNetwork={(network) => setLaunchSettings({ ...launchSettings, network: network })}
        />
        <button
          className="px-8 py-2 rounded-md border border-black disabled:bg-slate-400"
          disabled={!launchSettingsAreLegal}
          onClick={() => launch(launchSettings).then(() => setLaunched(true))}
        >
          Start
        </button>
        {!launchSettingsAreLegal && (
          <p className="text-red-500 whitespace-pre-wrap">{reason}</p>
        )}
      </div>
    );
  } else {
    return <></>;
  }
};

export default Launcher;
