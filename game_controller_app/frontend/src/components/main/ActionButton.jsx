import { applyAction } from "../../api.js";

const ActionButton = ({ action, active, label, legal }) => {
  return (
    <button
      className={`w-full h-16 rounded-md border border-gray-600 ${
        active ? "bg-gray-300" : legal ? "" : "text-gray-300 bg-gray-100"
      }`}
      disabled={!legal}
      onClick={action ? () => applyAction(action) : () => {}}
    >
      {label}
    </button>
  );
};

export default ActionButton;
