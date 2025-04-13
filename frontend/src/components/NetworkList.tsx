import React, { useEffect, useState } from 'react';
import { Network } from '../types';
import { networkApi } from '../services/api';

const NetworkList: React.FC = () => {
  const [networks, setNetworks] = useState<Network[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

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

  if (loading && networks.length === 0) {
    return <div className="card">Loading networks...</div>;
  }

  if (error && networks.length === 0) {
    return <div className="card">Error: {error}</div>;
  }

  return (
    <div>
      <div className="card-header">
        <h2>Networks</h2>
        <button className="button" onClick={fetchNetworks}>
          Refresh
        </button>
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
            </tr>
          </thead>
          <tbody>
            {networks.length === 0 ? (
              <tr>
                <td colSpan={5}>No networks found</td>
              </tr>
            ) : (
              networks.map((network) => (
                <tr key={network.id}>
                  <td>{network.name}</td>
                  <td>{network.id.substring(0, 12)}</td>
                  <td>{network.driver}</td>
                  <td>{network.scope}</td>
                  <td>
                    {network.ipam && network.ipam.config.length > 0
                      ? network.ipam.config.map((config, index) => (
                          <div key={index}>
                            {config.subnet && <div>Subnet: {config.subnet}</div>}
                            {config.gateway && <div>Gateway: {config.gateway}</div>}
                            {config.ip_range && <div>IP Range: {config.ip_range}</div>}
                          </div>
                        ))
                      : 'No IPAM config'}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default NetworkList;