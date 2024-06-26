const TeamColorSelector = ({ colors, color, setColor, isColorLegal }) => {
  return (
    <select
      className={`border-2 ${isColorLegal ? "" : "border-red-600"}`}
      value={color}
      onChange={(e) => setColor(e.target.value)}
    >
      {colors.map((color) => (
        <option key={color} value={color}>
          {color}
        </option>
      ))}
    </select>
  );
};

export default TeamColorSelector;
