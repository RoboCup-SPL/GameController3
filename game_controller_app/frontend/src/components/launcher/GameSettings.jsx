import TeamSettings from "./TeamSettings";

const GameSettings = ({ teams, game, setGame }) => {
  return (
    <div>
      <label htmlFor="play-off">Play-off</label>
      <input
        type="checkbox"
        checked={game.long}
        id="play-off"
        onChange={(e) => setGame({ ...game, long: e.target.checked })}
      />
      {["home", "away"].map((side) => {
        return (
          <div key={side}>
            <TeamSettings
              teams={teams}
              team={game.teams[side]}
              setTeam={(team) =>
                setGame({
                  ...game,
                  teams: { ...game.teams, [side]: team },
                })
              }
            />
            <input
              type="radio"
              value={side}
              checked={game.kickOffSide === side}
              onChange={(e) => {
                setGame({ ...game, kickOffSide: e.target.value });
              }}
            />
          </div>
        );
      })}
      <label htmlFor="mirror">Mirror</label>
      <input
        type="checkbox"
        checked={game.sideMapping === "homeDefendsRightGoal"}
        id="mirror"
        onChange={(e) =>
          setGame({
            ...game,
            sideMapping: e.target.checked ? "homeDefendsRightGoal" : "homeDefendsLeftGoal",
          })
        }
      />
    </div>
  );
};

export default GameSettings;
