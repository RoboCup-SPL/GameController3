const WindowSettings = ({ window, setWindow }) => {
  return (
    <div>
      <label htmlFor="fullscreen">Fullscreen</label>
      <input
        type="checkbox"
        checked={window.fullscreen}
        id="fullscreen"
        onChange={(e) => setWindow({ ...window, fullscreen: e.target.checked })}
      />
    </div>
  );
};

export default WindowSettings;
