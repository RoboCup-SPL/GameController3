const CompetitionSettings = ({ competitions, competition, setCompetition }) => {
  return (
    <div>
      <select
        value={competition.id}
        onChange={(e) => setCompetition({ ...competition, id: e.target.value })}
      >
        {competitions.map((competition) => (
          <option key={competition.id} value={competition.id}>
            {competition.name}
          </option>
        ))}
      </select>
      <input
        type="checkbox"
        checked={competition.playOff}
        onChange={(e) => setCompetition({ ...competition, playOff: e.target.checked })}
      />
    </div>
  );
};

export default CompetitionSettings;
