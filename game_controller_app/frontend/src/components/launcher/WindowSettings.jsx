const WindowSettings = ({ window, setWindow }) => {
  return (
    <div>
      <input
        type="checkbox"
        checked={window.fullscreen}
        onChange={(e) => setWindow({ ...window, fullscreen: e.target.checked })}
      />
    </div>
  );
};

export default WindowSettings;
