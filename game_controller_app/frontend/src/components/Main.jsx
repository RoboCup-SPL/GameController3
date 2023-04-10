import { useEffect, useState } from "react";
import CenterPanel from "./main/CenterPanel";
import TeamPanel from "./main/TeamPanel";
import { getLaunchData, listenForState, syncWithBackend } from "../api.js";

const Main = () => {
  const [connectionStatus, setConnectionStatus] = useState(null);
  const [game, setGame] = useState(null);
  const [legalActions, setLegalActions] = useState(null);
  const [params, setParams] = useState(null);
  const [selectedPenaltyCall, setSelectedPenaltyCall] = useState(null);
  const [teamNames, setTeamNames] = useState(null);

  useEffect(() => {
    const thePromise = (async () => {
      const unlisten = await listenForState((state) => {
        setConnectionStatus(state.connectionStatus);
        setLegalActions(state.legalActions);
        setGame(state.game);
      });
      // listen must have completed before starting the next call because the core may send a state
      // event once syncWithBackend is called that must not be missed.
      const params = await syncWithBackend();
      setParams(params);
      const teams = (await getLaunchData()).teams;
      setTeamNames(
        Object.fromEntries(
          Object.entries(params.game.teams).map(([side, teamParams]) => [
            side,
            teams.find((team) => team.number === teamParams.number).name,
          ])
        )
      );
      return unlisten;
    })();
    return () => {
      thePromise.then((unlisten) => unlisten());
    };
  }, []);

  if (
    connectionStatus != null &&
    game != null &&
    params != null &&
    teamNames != null
  ) {
    const mirror = game.sides === "homeDefendsRightGoal";
    return (
      <div
        className={`flex ${
          mirror ? "flex-row-reverse" : "flex-row"
        } h-full gap-4 p-2 overscroll-none`}
      >
        <div className="w-80">
          <TeamPanel
            connectionStatus={connectionStatus}
            game={game}
            params={params}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
            side="home"
            sign={mirror ? -1 : 1}
            teamNames={teamNames}
          />
        </div>
        <div className="grow">
          <CenterPanel
            game={game}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
          />
        </div>
        <div className="w-80">
          <TeamPanel
            connectionStatus={connectionStatus}
            game={game}
            params={params}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
            side="away"
            sign={mirror ? 1 : -1}
            teamNames={teamNames}
          />
        </div>
      </div>
    );
  } else {
    return <></>;
  }
};

export default Main;
