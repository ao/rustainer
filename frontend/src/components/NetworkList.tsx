import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Network, CreateNetworkRequest } from '../types';
import { networkApi } from '../services/api';

const NetworkList: React.FC = () => {
  const [networks, setNetworks] = useState<Network[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateModal, setShowCreateModal] = useState<boolean>(false);
  const [pruning, setPruning] = useState<boolean>(false);
  const [filter, setFilter] = useState<string>('');

  // New network form state
  const [newNetwork, setNewNetwork] = useState<CreateNetworkRequest>({
    name: '',
    driver: 'bridge',
    internal: false,
    enable_ipv6: false,
    options: {},
    labels: {},
    ipam: {
      driver: 'default',
      config: [{ subnet: '', gateway: '', ip_range: '' }]
    }
  });

  const fetchNetworks = async () => {
    try {
      setLoading(true);
      const data = await networkApi.getNetworks();
      setNetworks(data);
      setError(null);
    } catch (err) {
      setError('Failed to fetch networks');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchNetworks();
  }, []);

  const handleCreateNetwork = async () => {
    try {
      // Validate form
      if (!newNetwork.name) {
        setError('Network name is required');
        return;
      }

      // Clean up empty IPAM config
      const cleanedIpam = newNetwork.ipam ? {
        ...newNetwork.ipam,
        config: newNetwork.ipam.config.filter(config => 
          config.subnet || config.gateway || config.ip_range
        )
      } : undefined;

      // Create network
      await networkApi.createNetwork({
        ...newNetwork,
        ipam: cleanedIpam && cleanedIpam.config.length > 0 ? cleanedIpam : undefined
      });
      
      // Reset form and close modal
      setNewNetwork({
        name: '',
        driver: 'bridge',
        internal: false,
        enable_ipv6: false,
        options: {},
        labels: {},
        ipam: {
          driver: 'default',
          config: [{ subnet: '', gateway: '', ip_range: '' }]
        }
      });
      setShowCreateModal(false);
      
      // Refresh networks
      fetchNetworks();
    } catch (err) {
      setError('Failed to create network');
      console.error(err);
    }
  };

  const handleDeleteNetwork = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this network?')) {
      try {
        await networkApi.deleteNetwork(id);
        fetchNetworks();
      } catch (err) {
        setError('Failed to delete network');
        console.error(err);
      }
    }
  };

  const handlePruneNetworks = async () => {
    if (window.confirm('Are you sure you want to prune unused networks?')) {
      try {
        setPruning(true);
        const prunedNetworks = await networkApi.pruneNetworks();
        setPruning(false);
        alert(`Pruned ${prunedNetworks.length} networks`);
        fetchNetworks();
      } catch (err) {
        setError('Failed to prune networks');
        console.error(err);
        setPruning(false);
      }
    }
  };

  // Add IPAM config field
  const addIpamConfig = () => {
    if (newNetwork.ipam) {
      setNewNetwork({
        ...newNetwork,
        ipam: {
          ...newNetwork.ipam,
          config: [...newNetwork.ipam.config, { subnet: '', gateway: '', ip_range: '' }]
        }
      });
    }
  };

  // Remove IPAM config field
  const removeIpamConfig = (index: number) => {
    if (newNetwork.ipam) {
      setNewNetwork({
        ...newNetwork,
        ipam: {
          ...newNetwork.ipam,
          config: newNetwork.ipam.config.filter((_, i) => i !== index)
        }
      });
    }
  };

  // Update IPAM config field
  const updateIpamConfig = (index: number, field: string, value: string) => {
    if (newNetwork.ipam) {
      const newConfig = [...newNetwork.ipam.config];
      newConfig[index] = { ...newConfig[index], [field]: value };
      setNewNetwork({
        ...newNetwork,
        ipam: {
          ...newNetwork.ipam,
          config: newConfig
        }
      });
    }
  };

  // Filter networks
  const filteredNetworks = networks.filter(network => 
    network.name.toLowerCase().includes(filter.toLowerCase()) ||
    network.id.toLowerCase().includes(filter.toLowerCase()) ||
    network.driver.toLowerCase().includes(filter.toLowerCase())
  );

  if (loading && networks.length === 0) {
    return <div className="card">Loading networks...</div>;
  }

  if (error && networks.length === 0) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div>
      <div className="header-actions">
        <h2>Networks</h2>
        <div>
          <button className="button" onClick={() => setShowCreateModal(true)}>
            Create Network
          </button>
          <button 
            className="button secondary" 
            onClick={handlePruneNetworks}
            disabled={pruning}
          >
            {pruning ? 'Pruning...' : 'Prune Networks'}
          </button>
          <button className="button" onClick={fetchNetworks}>
            Refresh
          </button>
        </div>
      </div>

      {error && <div className="error-message">{error}</div>}

      <div className="search-box">
        <input
          type="text"
          placeholder="Filter networks..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
        />
      </div>

      <div className="card">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>ID</th>
              <th>Driver</th>
              <th>Scope</th>
              <th>IPAM</th>
              <th>Containers</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {filteredNetworks.length === 0 ? (
              <tr>
                <td colSpan={7}>No networks found</td>
              </tr>
            ) : (
              filteredNetworks.map((network) => (
                <tr key={network.id}>
                  <td>{network.name}</td>
                  <td>{network.id.substring(0, 12)}</td>
                  <td>
                    {network.driver}
                    {network.internal && <span className="badge">internal</span>}
                    {network.enable_ipv6 && <span className="badge">ipv6</span>}
                  </td>
                  <td>{network.scope}</td>
                  <td>
                    {network.ipam && network.ipam.config.length > 0
                      ? network.ipam.config.map((config, index) => (
                          <div key={index} className="ipam-config">
                            {config.subnet && <div>Subnet: {config.subnet}</div>}
                            {config.gateway && <div>Gateway: {config.gateway}</div>}
                            {config.ip_range && <div>IP Range: {config.ip_range}</div>}
                          </div>
                        ))
                      : 'No IPAM config'}
                  </td>
                  <td>
                    {network.containers && network.containers.length > 0 ? (
                      <div className="container-count">
                        {network.containers.length} container(s)
                      </div>
                    ) : (
                      'None'
                    )}
                  </td>
                  <td>
                    <div className="actions">
                      <Link to={`/networks/${network.id}`} className="button">
                        Details
                      </Link>
                      <button
                        className="button danger"
                        onClick={() => handleDeleteNetwork(network.id)}
                        disabled={network.name === 'bridge' || network.name === 'host' || network.name === 'none'}
                      >
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Create Network Modal */}
      {showCreateModal && (
        <div className="modal">
          <div className="modal-content">
            <div className="modal-header">
              <h3>Create Network</h3>
              <button onClick={() => setShowCreateModal(false)} className="close-button">&times;</button>
            </div>
            <div className="modal-body">
              <div className="form-group">
                <label htmlFor="name">Name:</label>
                <input
                  type="text"
                  id="name"
                  value={newNetwork.name}
                  onChange={(e) => setNewNetwork({ ...newNetwork, name: e.target.value })}
                  required
                />
              </div>

              <div className="form-group">
                <label htmlFor="driver">Driver:</label>
                <select
                  id="driver"
                  value={newNetwork.driver}
                  onChange={(e) => setNewNetwork({ ...newNetwork, driver: e.target.value })}
                >
                  <option value="bridge">bridge</option>
                  <option value="host">host</option>
                  <option value="overlay">overlay</option>
                  <option value="macvlan">macvlan</option>
                  <option value="ipvlan">ipvlan</option>
                  <option value="none">none</option>
                </select>
              </div>

              <div className="form-group">
                <label>
                  <input
                    type="checkbox"
                    checked={newNetwork.internal || false}
                    onChange={(e) => setNewNetwork({ ...newNetwork, internal: e.target.checked })}
                  />
                  Internal Network
                </label>
              </div>

              <div className="form-group">
                <label>
                  <input
                    type="checkbox"
                    checked={newNetwork.enable_ipv6 || false}
                    onChange={(e) => setNewNetwork({ ...newNetwork, enable_ipv6: e.target.checked })}
                  />
                  Enable IPv6
                </label>
              </div>

              <div className="form-section">
                <h4>IPAM Configuration</h4>
                <div className="form-group">
                  <label htmlFor="ipam-driver">IPAM Driver:</label>
                  <input
                    type="text"
                    id="ipam-driver"
                    value={newNetwork.ipam?.driver || 'default'}
                    onChange={(e) => setNewNetwork({
                      ...newNetwork,
                      ipam: {
                        ...newNetwork.ipam!,
                        driver: e.target.value
                      }
                    })}
                  />
                </div>

                {newNetwork.ipam?.config.map((config, index) => (
                  <div key={index} className="ipam-config-form">
                    <h5>Config #{index + 1}</h5>
                    <div className="form-group">
                      <label htmlFor={`subnet-${index}`}>Subnet:</label>
                      <input
                        type="text"
                        id={`subnet-${index}`}
                        value={config.subnet || ''}
                        onChange={(e) => updateIpamConfig(index, 'subnet', e.target.value)}
                        placeholder="e.g., 172.20.0.0/16"
                      />
                    </div>
                    <div className="form-group">
                      <label htmlFor={`gateway-${index}`}>Gateway:</label>
                      <input
                        type="text"
                        id={`gateway-${index}`}
                        value={config.gateway || ''}
                        onChange={(e) => updateIpamConfig(index, 'gateway', e.target.value)}
                        placeholder="e.g., 172.20.0.1"
                      />
                    </div>
                    <div className="form-group">
                      <label htmlFor={`ip-range-${index}`}>IP Range:</label>
                      <input
                        type="text"
                        id={`ip-range-${index}`}
                        value={config.ip_range || ''}
                        onChange={(e) => updateIpamConfig(index, 'ip_range', e.target.value)}
                        placeholder="e.g., 172.20.10.0/24"
                      />
                    </div>
                    <button
                      type="button"
                      className="button danger"
                      onClick={() => removeIpamConfig(index)}
                    >
                      Remove Config
                    </button>
                  </div>
                ))}

                <button
                  type="button"
                  className="button"
                  onClick={addIpamConfig}
                >
                  Add IPAM Config
                </button>
              </div>

              <div className="form-actions">
                <button
                  type="button"
                  className="button"
                  onClick={handleCreateNetwork}
                >
                  Create Network
                </button>
                <button
                  type="button"
                  className="button secondary"
                  onClick={() => setShowCreateModal(false)}
                >
                  Cancel
                </button>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default NetworkList;