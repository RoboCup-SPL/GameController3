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
      <label htmlFor="play-off">Play-off</label>
      <input
        type="checkbox"
        checked={competition.playOff}
        id="play-off"
        onChange={(e) => setCompetition({ ...competition, playOff: e.target.checked })}
      />
    </div>
  );
};

export default CompetitionSettings;
