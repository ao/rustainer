import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Container } from '../types';
import api from '../services/api';

const ContainerList: React.FC = () => {
  const [containers, setContainers] = useState<Container[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchContainers = async () => {
    try {
      setLoading(true);
      const data = await api.containers.listContainers();
      setContainers(data);
      setError(null);
    } catch (err) {
      setError('Failed to fetch containers');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchContainers();
    // Poll for container updates every 5 seconds
    const interval = setInterval(fetchContainers, 5000);
    return () => clearInterval(interval);
  }, []);

  const handleStartContainer = async (id: string) => {
    try {
      await api.containers.startContainer(id);
      fetchContainers();
    } catch (err) {
      console.error('Failed to start container:', err);
    }
  };

  const handleStopContainer = async (id: string) => {
    try {
      await api.containers.stopContainer(id);
      fetchContainers();
    } catch (err) {
      console.error('Failed to stop container:', err);
    }
  };

  const handleRestartContainer = async (id: string) => {
    try {
      await api.containers.restartContainer(id);
      fetchContainers();
    } catch (err) {
      console.error('Failed to restart container:', err);
    }
  };

  if (loading && containers.length === 0) {
    return <div className="card">Loading containers...</div>;
  }

  if (error && containers.length === 0) {
    return <div className="card">Error: {error}</div>;
  }

  return (
    <div>
      <div className="card-header">
        <h2>Containers</h2>
        <button className="button" onClick={fetchContainers}>
          Refresh
        </button>
      </div>

      <div className="card">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Image</th>
              <th>Status</th>
              <th>Created</th>
              <th>Ports</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {containers.length === 0 ? (
              <tr>
                <td colSpan={6}>No containers found</td>
              </tr>
            ) : (
              containers.map((container) => (
                <tr key={container.id}>
                  <td>
                    <Link to={`/containers/${container.id}`}>
                      {container.names.join(', ')}
                    </Link>
                  </td>
                  <td>{container.image}</td>
                  <td
                    className={
                      container.state === 'running'
                        ? 'status-running'
                        : 'status-stopped'
                    }
                  >
                    {container.status}
                  </td>
                  <td>
                    {new Date(container.created).toLocaleString()}
                  </td>
                  <td>
                    {container.ports.length > 0
                      ? container.ports.map((port) => (
                          <div key={`${port.container_port}-${port.host_port}`}>
                            {port.host_port}:{port.container_port}/{port.protocol}
                          </div>
                        ))
                      : 'None'}
                  </td>
                  <td>
                    <div className="button-group">
                      {container.state === 'running' ? (
                        <>
                          <button
                            className="button button-danger"
                            onClick={() => handleStopContainer(container.id)}
                          >
                            Stop
                          </button>
                          <button
                            className="button"
                            onClick={() => handleRestartContainer(container.id)}
                          >
                            Restart
                          </button>
                        </>
                      ) : (
                        <button
                          className="button button-success"
                          onClick={() => handleStartContainer(container.id)}
                        >
                          Start
                        </button>
                      )}
                    </div>
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

export default ContainerList;