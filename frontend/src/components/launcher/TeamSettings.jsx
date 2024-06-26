import TeamColorSelector from "./TeamColorSelector";
import TeamSelector from "./TeamSelector";

const TeamSettings = ({
  teams,
  team,
  setTeam,
  isTeamLegal,
  isFieldPlayerColorLegal,
  isGoalkeeperColorLegal,
}) => {
  const setNumber = (number) => {
    const teamOptions = teams.find((t) => t.number === number);
    setTeam({
      ...team,
      number: number,
      fieldPlayerColor: teamOptions.fieldPlayerColors[0],
      goalkeeperColor: teamOptions.goalkeeperColors[0],
    });
  };
  const teamOptions = teams.find((t) => t.number === team.number);
  return (
    <div className="flex flex-col gap-2">
      <TeamSelector
        teams={teams}
        number={team.number}
        setNumber={setNumber}
        isTeamLegal={isTeamLegal}
      />
      <div className="flex flex-row gap-2">
        <div className="flex flex-col gap-1">
          <label>Field Player Color</label>
          <TeamColorSelector
            colors={teamOptions.fieldPlayerColors}
            color={team.fieldPlayerColor}
            setColor={(color) => setTeam({ ...team, fieldPlayerColor: color })}
            isColorLegal={isFieldPlayerColorLegal}
          />
        </div>
        <div className="flex flex-col gap-1">
          <label>Goalkeeper Color</label>
          <TeamColorSelector
            colors={teamOptions.goalkeeperColors}
            color={team.goalkeeperColor}
            setColor={(color) => setTeam({ ...team, goalkeeperColor: color })}
            isColorLegal={isGoalkeeperColorLegal}
          />
        </div>
      </div>
    </div>
  );
};

export default TeamSettings;
