const TeamSelector = ({ teams, number, setNumber, isTeamLegal }) => {
  return (
    <select
      className={`w-full border-2 ${isTeamLegal ? "" : "border-red-600"}`}
      value={number}
      onChange={(e) => setNumber(parseInt(e.target.value))}
    >
      {teams
        .sort((a, b) => a.name.localeCompare(b.name))
        .map((team) => (
          <option key={team.number} value={team.number}>
            {team.name} ({team.number})
          </option>
        ))}
    </select>
  );
};

export default TeamSelector;
