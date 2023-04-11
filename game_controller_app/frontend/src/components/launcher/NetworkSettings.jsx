const NetworkSettings = ({ interfaces, network, setNetwork }) => {
  return (
    <div>
      <label htmlFor="interface">Interface</label>
      <select
        id="interface"
        value={network.interface}
        onChange={(e) => setNetwork({ ...network, interface: e.target.value })}
      >
        {interfaces.map((interphase) => (
          <option key={interphase.id} value={interphase.id}>
            {interphase.id}
          </option>
        ))}
      </select>
      <label htmlFor="broadcast">Broadcast</label>
      <input
        type="checkbox"
        checked={network.broadcast}
        id="broadcast"
        onChange={(e) => setNetwork({ ...network, broadcast: e.target.checked })}
      />
      <label htmlFor="multicast">Multicast</label>
      <input
        type="checkbox"
        checked={network.multicast}
        id="multicast"
        onChange={(e) => setNetwork({ ...network, multicast: e.target.checked })}
      />
    </div>
  );
};

export default NetworkSettings;
