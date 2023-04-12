import { useEffect, useState } from "react";
import CompetitionSettings from "./launcher/CompetitionSettings";
import GameSettings from "./launcher/GameSettings";
import NetworkSettings from "./launcher/NetworkSettings";
import WindowSettings from "./launcher/WindowSettings";
import { getLaunchData, launch } from "../api";

const Launcher = () => {
  const [competitions, setCompetitions] = useState(null);
  const [launchSettings, setLaunchSettings] = useState(null);
  const [networkInterfaces, setNetworkInterfaces] = useState(null);
  const [teams, setTeams] = useState(null);

  const launchSettingsAreLegal =
    launchSettings != null &&
    launchSettings.game.teams.home.number != launchSettings.game.teams.away.number &&
    launchSettings.game.teams.home.fieldPlayerColor !=
      launchSettings.game.teams.away.fieldPlayerColor &&
    launchSettings.game.teams.home.fieldPlayerColor !=
      launchSettings.game.teams.home.goalkeeperColor &&
    launchSettings.game.teams.home.fieldPlayerColor !=
      launchSettings.game.teams.away.goalkeeperColor &&
    launchSettings.game.teams.away.fieldPlayerColor !=
      launchSettings.game.teams.away.goalkeeperColor &&
    launchSettings.game.teams.away.fieldPlayerColor !=
      launchSettings.game.teams.home.goalkeeperColor;

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
      });
    };
    const thisCompetition = competitions.find(
      (competition) => competition.id === launchSettings.competition.id
    );
    const teamsInThisCompetition = teams.filter((team) =>
      thisCompetition.teams.includes(team.number)
    );
    return (
      <>
        <div>
          <CompetitionSettings
            competitions={competitions}
            competition={launchSettings.competition}
            setCompetition={setCompetition}
          />
        </div>
        <div>
          <GameSettings
            teams={teamsInThisCompetition}
            game={launchSettings.game}
            setGame={(game) => setLaunchSettings({ ...launchSettings, game: game })}
          />
        </div>
        <div>
          <WindowSettings
            window={launchSettings.window}
            setWindow={(window) => setLaunchSettings({ ...launchSettings, window: window })}
          />
        </div>
        <div>
          <NetworkSettings
            interfaces={networkInterfaces}
            network={launchSettings.network}
            setNetwork={(network) => setLaunchSettings({ ...launchSettings, network: network })}
          />
        </div>
        <div>
          <button disabled={!launchSettingsAreLegal} onClick={() => launch(launchSettings)}>
            Start
          </button>
        </div>
      </>
    );
  } else {
    return <></>;
  }
};

export default Launcher;
