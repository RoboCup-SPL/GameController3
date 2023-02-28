import { applyAction } from "../../api.js";

const ActionButton = ({ action, active, label }) => {
  return (
    <button
      onClick={action ? () => applyAction(action) : () => {}}
      className={`w-full h-16 rounded-md border border-gray-600 ${active ? "bg-gray-300" : ""}`}
    >
      {label}
    </button>
  );
};

export default ActionButton;
