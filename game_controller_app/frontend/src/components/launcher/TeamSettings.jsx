import TeamColorSelector from "./TeamColorSelector";
import TeamSelector from "./TeamSelector";

const TeamSettings = ({ teams, team, setTeam }) => {
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
    <div>
      <TeamSelector teams={teams} number={team.number} setNumber={setNumber} />
      <TeamColorSelector
        colors={teamOptions.fieldPlayerColors}
        color={team.fieldPlayerColor}
        setColor={(color) => setTeam({ ...team, fieldPlayerColor: color })}
      />
      <TeamColorSelector
        colors={teamOptions.goalkeeperColors}
        color={team.goalkeeperColor}
        setColor={(color) => setTeam({ ...team, goalkeeperColor: color })}
      />
    </div>
  );
};

export default TeamSettings;
