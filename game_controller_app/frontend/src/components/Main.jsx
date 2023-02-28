import { useEffect, useState } from "react";
import CenterPanel from "./main/CenterPanel";
import TeamPanel from "./main/TeamPanel";
import { listenForState, syncWithBackend } from "../api.js";

const Main = () => {
  const [game, setGame] = useState(null);
  const [selectedPenaltyCall, setSelectedPenaltyCall] = useState(null);

  useEffect(() => {
    const thePromise = (async () => {
      const unlisten = await listenForState((state) => {
        setGame(state.game);
      });
      // listen must have completed before starting the next call because the core may send a state
      // event once syncWithBackend is called that must not be missed.
      await syncWithBackend();
      return unlisten;
    })();
    return () => {
      thePromise.then((unlisten) => unlisten());
    };
  }, []);

  if (game != null) {
    const mirror = game.sides === "homeDefendsRightGoal";
    return (
      <div
        className={`flex ${
          mirror ? "flex-row-reverse" : "flex-row"
        } h-full gap-4 p-2 overscroll-none`}
      >
        <div className="w-80">
          <TeamPanel
            game={game}
            team={game.teams.home}
            side="home"
            sign={mirror ? -1 : 1}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
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
            game={game}
            team={game.teams.away}
            side="away"
            sign={mirror ? 1 : -1}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
          />
        </div>
      </div>
    );
  } else {
    return <></>;
  }
};

export default Main;
