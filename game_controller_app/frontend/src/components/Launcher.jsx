import { useEffect, useState } from "react";
import CompetitionSettings from "./launcher/CompetitionSettings";
import NetworkSettings from "./launcher/NetworkSettings";
import TeamSettings from "./launcher/TeamSettings";
import WindowSettings from "./launcher/WindowSettings";
import { getLaunchData, launch } from "../api";

const Launcher = () => {
  const [competitions, setCompetitions] = useState(null);
  const [launchSettings, setLaunchSettings] = useState(null);
  const [networkInterfaces, setNetworkInterfaces] = useState(null);
  const [teams, setTeams] = useState(null);

  const launchSettingsAreLegal =
    launchSettings != null &&
    launchSettings.teams.home.number != launchSettings.teams.away.number &&
    launchSettings.teams.home.fieldPlayerColor != launchSettings.teams.away.fieldPlayerColor &&
    launchSettings.teams.home.fieldPlayerColor != launchSettings.teams.home.goalkeeperColor &&
    launchSettings.teams.home.fieldPlayerColor != launchSettings.teams.away.goalkeeperColor &&
    launchSettings.teams.away.fieldPlayerColor != launchSettings.teams.away.goalkeeperColor &&
    launchSettings.teams.away.fieldPlayerColor != launchSettings.teams.home.goalkeeperColor;

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
          <TeamSettings
            teams={teamsInThisCompetition}
            team={launchSettings.teams.home}
            setTeam={(team) =>
              setLaunchSettings({
                ...launchSettings,
                teams: { home: team, away: launchSettings.teams.away },
              })
            }
          />
          <TeamSettings
            teams={teamsInThisCompetition}
            team={launchSettings.teams.away}
            setTeam={(team) =>
              setLaunchSettings({
                ...launchSettings,
                teams: { home: launchSettings.teams.home, away: team },
              })
            }
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
