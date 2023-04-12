const CompetitionSettings = ({ competitions, competition, setCompetition }) => {
  return (
    <div>
      <label htmlFor="competition">Competition</label>
      <select
        value={competition.id}
        id="competition"
        onChange={(e) => setCompetition({ ...competition, id: e.target.value })}
      >
        {competitions.map((competition) => (
          <option key={competition.id} value={competition.id}>
            {competition.name}
          </option>
        ))}
      </select>
    </div>
  );
};

export default CompetitionSettings;
