const NetworkSettings = ({ interfaces, network, setNetwork }) => {
  return (
    <div>
      <select
        value={network.interface}
        onChange={(e) => setNetwork({ ...network, interface: e.target.value })}
      >
        {interfaces.map((interphase) => (
          <option key={interphase.id} value={interphase.id}>
            {interphase.id}
          </option>
        ))}
      </select>
      <input
        type="checkbox"
        checked={network.broadcast}
        onChange={(e) => setNetwork({ ...network, broadcast: e.target.checked })}
      />
      <input
        type="checkbox"
        checked={network.multicast}
        onChange={(e) => setNetwork({ ...network, multicast: e.target.checked })}
      />
    </div>
  );
};

export default NetworkSettings;
