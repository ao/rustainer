import React, { useEffect, useState } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { 
  Network, 
  NetworkDiagnostics, 
  ConnectContainerRequest, 
  DisconnectContainerRequest,
  Container
} from '../types';
import api from '../services/api';

const NetworkDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  
  const [network, setNetwork] = useState<Network | null>(null);
  const [diagnostics, setDiagnostics] = useState<NetworkDiagnostics | null>(null);
  const [availableContainers, setAvailableContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [showConnectModal, setShowConnectModal] = useState<boolean>(false);
  
  // Connect container form state
  const [connectRequest, setConnectRequest] = useState<ConnectContainerRequest>({
    container_id: '',
    ipv4_address: '',
    ipv6_address: '',
    aliases: []
  });
  
  // New alias state
  const [newAlias, setNewAlias] = useState<string>('');
  
  useEffect(() => {
    if (id) {
      fetchNetworkData(id);
    }
  }, [id]);
  
  const fetchNetworkData = async (networkId: string) => {
    try {
      setLoading(true);
      
      // Fetch network details
      const networkData = await api.networks.getNetwork(networkId);
      setNetwork(networkData);
      
      // Fetch network diagnostics
      try {
        const diagnosticsData = await api.networks.getNetworkDiagnostics(networkId);
        setDiagnostics(diagnosticsData);
      } catch (err) {
        console.error('Failed to fetch network diagnostics:', err);
        // Don't set error here, as we still have the network data
      }
      
      // Fetch available containers
      const containers = await api.containers.listContainers();
      
      // Filter out containers already in this network
      const networkContainerIds = new Set(networkData.containers.map(c => c.id));
      const availableContainers = containers.filter(c => !networkContainerIds.has(c.id));
      
      setAvailableContainers(availableContainers);
      setError(null);
    } catch (err) {
      setError('Failed to fetch network details');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };
  
  const handleDeleteNetwork = async () => {
    if (!network || !id) return;
    
    if (window.confirm(`Are you sure you want to delete network "${network.name}"?`)) {
      try {
        await api.networks.deleteNetwork(id);
        navigate('/networks');
      } catch (err) {
        setError('Failed to delete network');
        console.error(err);
      }
    }
  };
  
  const handleConnectContainer = async () => {
    if (!id || !connectRequest.container_id) return;
    
    try {
      await api.networks.connectContainer(id, connectRequest);
      setShowConnectModal(false);
      setConnectRequest({
        container_id: '',
        ipv4_address: '',
        ipv6_address: '',
        aliases: []
      });
      fetchNetworkData(id);
    } catch (err) {
      setError('Failed to connect container to network');
      console.error(err);
    }
  };
  
  const handleDisconnectContainer = async (containerId: string) => {
    if (!id) return;
    
    if (window.confirm('Are you sure you want to disconnect this container from the network?')) {
      try {
        const request: DisconnectContainerRequest = {
          container_id: containerId,
          force: false
        };
        await api.networks.disconnectContainer(id, request);
        fetchNetworkData(id);
        fetchNetworkData(id);
      } catch (err) {
        setError('Failed to disconnect container from network');
        console.error(err);
      }
    }
  };
  
  const addAlias = () => {
    if (newAlias.trim() === '') return;
    
    setConnectRequest({
      ...connectRequest,
      aliases: [...(connectRequest.aliases || []), newAlias.trim()]
    });
    
    setNewAlias('');
  };
  
  const removeAlias = (index: number) => {
    if (!connectRequest.aliases) return;
    
    setConnectRequest({
      ...connectRequest,
      aliases: connectRequest.aliases.filter((_, i) => i !== index)
    });
  };
  
  if (loading && !network) {
    return <div className="loading">Loading network details...</div>;
  }
  
  if (error && !network) {
    return <div className="error-message">{error}</div>;
  }
  
  if (!network) {
    return <div className="error-message">Network not found</div>;
  }
  
  return (
    <div className="network-detail">
      <div className="header-actions">
        <h2>{network.name}</h2>
        <div>
          <button 
            className="button" 
            onClick={() => setShowConnectModal(true)}
            disabled={availableContainers.length === 0}
          >
            Connect Container
          </button>
          <button 
            className="button danger" 
            onClick={handleDeleteNetwork}
            disabled={network.name === 'bridge' || network.name === 'host' || network.name === 'none'}
          >
            Delete Network
          </button>
          <Link to="/networks" className="button secondary">
            Back to Networks
          </Link>
        </div>
      </div>
      
      {error && <div className="error-message">{error}</div>}
      
      <div className="card">
        <h3>Network Information</h3>
        <div className="network-info">
          <div className="info-group">
            <div className="info-label">ID:</div>
            <div className="info-value">{network.id}</div>
          </div>
          <div className="info-group">
            <div className="info-label">Driver:</div>
            <div className="info-value">{network.driver}</div>
          </div>
          <div className="info-group">
            <div className="info-label">Scope:</div>
            <div className="info-value">{network.scope}</div>
          </div>
          <div className="info-group">
            <div className="info-label">Internal:</div>
            <div className="info-value">{network.internal ? 'Yes' : 'No'}</div>
          </div>
          <div className="info-group">
            <div className="info-label">IPv6 Enabled:</div>
            <div className="info-value">{network.enable_ipv6 ? 'Yes' : 'No'}</div>
          </div>
          {network.created && (
            <div className="info-group">
              <div className="info-label">Created:</div>
              <div className="info-value">{network.created}</div>
            </div>
          )}
        </div>
      </div>
      
      <div className="card">
        <h3>IPAM Configuration</h3>
        {network.ipam && network.ipam.config.length > 0 ? (
          <div className="ipam-info">
            <div className="info-group">
              <div className="info-label">Driver:</div>
              <div className="info-value">{network.ipam.driver}</div>
            </div>
            
            <h4>Configurations</h4>
            <table>
              <thead>
                <tr>
                  <th>Subnet</th>
                  <th>Gateway</th>
                  <th>IP Range</th>
                </tr>
              </thead>
              <tbody>
                {network.ipam.config.map((config, index) => (
                  <tr key={index}>
                    <td>{config.subnet || '-'}</td>
                    <td>{config.gateway || '-'}</td>
                    <td>{config.ip_range || '-'}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="no-data">No IPAM configuration</div>
        )}
      </div>
      
      {Object.keys(network.options).length > 0 && (
        <div className="card">
          <h3>Options</h3>
          <table>
            <thead>
              <tr>
                <th>Key</th>
                <th>Value</th>
              </tr>
            </thead>
            <tbody>
              {Object.entries(network.options).map(([key, value]) => (
                <tr key={key}>
                  <td>{key}</td>
                  <td>{value}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
      
      {Object.keys(network.labels).length > 0 && (
        <div className="card">
          <h3>Labels</h3>
          <table>
            <thead>
              <tr>
                <th>Key</th>
                <th>Value</th>
              </tr>
            </thead>
            <tbody>
              {Object.entries(network.labels).map(([key, value]) => (
                <tr key={key}>
                  <td>{key}</td>
                  <td>{value}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
      
      <div className="card">
        <h3>Connected Containers</h3>
        {network.containers && network.containers.length > 0 ? (
          <table>
            <thead>
              <tr>
                <th>Name</th>
                <th>ID</th>
                <th>IPv4 Address</th>
                <th>IPv6 Address</th>
                <th>MAC Address</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {network.containers.map((container) => (
                <tr key={container.id}>
                  <td>{container.name}</td>
                  <td>{container.id.substring(0, 12)}</td>
                  <td>{container.ipv4_address || '-'}</td>
                  <td>{container.ipv6_address || '-'}</td>
                  <td>{container.mac_address || '-'}</td>
                  <td>
                    <div className="actions">
                      <Link to={`/containers/${container.id}`} className="button">
                        View
                      </Link>
                      <button
                        className="button danger"
                        onClick={() => handleDisconnectContainer(container.id)}
                      >
                        Disconnect
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <div className="no-data">No containers connected to this network</div>
        )}
      </div>
      
      {diagnostics && (
        <div className="card">
          <h3>Network Diagnostics</h3>
          <div className="network-diagnostics">
            <div className="info-group">
              <div className="info-label">Status:</div>
              <div className={`info-value status-${diagnostics.status.operational ? 'up' : 'down'}`}>
                {diagnostics.status.operational ? 'Operational' : 'Not Operational'}
              </div>
            </div>
            <div className="info-group">
              <div className="info-label">Message:</div>
              <div className="info-value">{diagnostics.status.message}</div>
            </div>
            <div className="info-group">
              <div className="info-label">Container Count:</div>
              <div className="info-value">{diagnostics.status.container_count}</div>
            </div>
            
            <h4>Network Metrics</h4>
            <div className="metrics-grid">
              <div className="metric-card">
                <div className="metric-title">Received</div>
                <div className="metric-value">{formatBytes(diagnostics.metrics.rx_bytes)}</div>
                <div className="metric-subtitle">{diagnostics.metrics.rx_packets} packets</div>
                <div className="metric-subtitle">{diagnostics.metrics.rx_errors} errors</div>
                <div className="metric-subtitle">{diagnostics.metrics.rx_dropped} dropped</div>
              </div>
              <div className="metric-card">
                <div className="metric-title">Transmitted</div>
                <div className="metric-value">{formatBytes(diagnostics.metrics.tx_bytes)}</div>
                <div className="metric-subtitle">{diagnostics.metrics.tx_packets} packets</div>
                <div className="metric-subtitle">{diagnostics.metrics.tx_errors} errors</div>
                <div className="metric-subtitle">{diagnostics.metrics.tx_dropped} dropped</div>
              </div>
            </div>
            
            {diagnostics.connectivity.length > 0 && (
              <>
                <h4>Connectivity Checks</h4>
                <table>
                  <thead>
                    <tr>
                      <th>Source</th>
                      <th>Destination</th>
                      <th>Status</th>
                      <th>Latency</th>
                      <th>Timestamp</th>
                    </tr>
                  </thead>
                  <tbody>
                    {diagnostics.connectivity.map((check, index) => (
                      <tr key={index}>
                        <td>{check.source_name}</td>
                        <td>{check.destination_name}</td>
                        <td className={check.success ? 'status-up' : 'status-down'}>
                          {check.success ? 'Success' : 'Failed'}
                        </td>
                        <td>{check.latency_ms ? `${check.latency_ms.toFixed(2)} ms` : '-'}</td>
                        <td>{new Date(check.timestamp).toLocaleString()}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </>
            )}
          </div>
        </div>
      )}
      
      {/* Connect Container Modal */}
      {showConnectModal && (
        <div className="modal">
          <div className="modal-content">
            <div className="modal-header">
              <h3>Connect Container to Network</h3>
              <button onClick={() => setShowConnectModal(false)} className="close-button">&times;</button>
            </div>
            <div className="modal-body">
              <div className="form-group">
                <label htmlFor="container">Container:</label>
                <select
                  id="container"
                  value={connectRequest.container_id}
                  onChange={(e) => setConnectRequest({ ...connectRequest, container_id: e.target.value })}
                  required
                >
                  <option value="">Select a container</option>
                  {availableContainers.map((container) => (
                    <option key={container.id} value={container.id}>
                      {container.names[0]} ({container.id.substring(0, 12)})
                    </option>
                  ))}
                </select>
              </div>
              
              <div className="form-group">
                <label htmlFor="ipv4">IPv4 Address (optional):</label>
                <input
                  type="text"
                  id="ipv4"
                  value={connectRequest.ipv4_address || ''}
                  onChange={(e) => setConnectRequest({ ...connectRequest, ipv4_address: e.target.value })}
                  placeholder="e.g., 172.20.0.2"
                />
              </div>
              
              <div className="form-group">
                <label htmlFor="ipv6">IPv6 Address (optional):</label>
                <input
                  type="text"
                  id="ipv6"
                  value={connectRequest.ipv6_address || ''}
                  onChange={(e) => setConnectRequest({ ...connectRequest, ipv6_address: e.target.value })}
                  placeholder="e.g., 2001:db8::2"
                />
              </div>
              
              <div className="form-group">
                <label>Aliases:</label>
                <div className="aliases-container">
                  {connectRequest.aliases && connectRequest.aliases.length > 0 ? (
                    <div className="aliases-list">
                      {connectRequest.aliases.map((alias, index) => (
                        <div key={index} className="alias-item">
                          <span>{alias}</span>
                          <button
                            type="button"
                            className="remove-alias"
                            onClick={() => removeAlias(index)}
                          >
                            &times;
                          </button>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <div className="no-aliases">No aliases</div>
                  )}
                  
                  <div className="add-alias">
                    <input
                      type="text"
                      value={newAlias}
                      onChange={(e) => setNewAlias(e.target.value)}
                      placeholder="Enter alias"
                    />
                    <button
                      type="button"
                      className="button"
                      onClick={addAlias}
                      disabled={!newAlias.trim()}
                    >
                      Add
                    </button>
                  </div>
                </div>
              </div>
              
              <div className="form-actions">
                <button
                  type="button"
                  className="button"
                  onClick={handleConnectContainer}
                  disabled={!connectRequest.container_id}
                >
                  Connect
                </button>
                <button
                  type="button"
                  className="button secondary"
                  onClick={() => setShowConnectModal(false)}
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

// Helper function to format bytes
const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes';
  
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

export default NetworkDetail;