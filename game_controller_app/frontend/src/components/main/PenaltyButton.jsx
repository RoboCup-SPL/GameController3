const PenaltyButton = ({ label, onClick, selected }) => {
  return (
    <button
      className={`h-16 rounded-md border border-gray-600 ${selected ? "bg-gray-300" : ""}`}
      onClick={onClick}
    >
      {label}
    </button>
  );
};

export default PenaltyButton;
