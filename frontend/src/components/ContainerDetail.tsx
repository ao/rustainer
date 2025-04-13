import React, { useEffect, useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Container, ContainerLogs, ContainerStats } from '../types';
import { containerApi } from '../services/api';

const ContainerDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [container, setContainer] = useState<Container | null>(null);
  const [logs, setLogs] = useState<string[]>([]);
  const [stats, setStats] = useState<ContainerStats | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchContainer = async () => {
    if (!id) return;
    
    try {
      setLoading(true);
      const containers = await containerApi.getContainers();
      const foundContainer = containers.find(c => c.id === id);
      
      if (foundContainer) {
        setContainer(foundContainer);
        setError(null);
      } else {
        setError('Container not found');
      }
    } catch (err) {
      setError('Failed to fetch container details');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const fetchLogs = async () => {
    if (!id) return;
    
    try {
      const logsData = await containerApi.getContainerLogs(id);
      setLogs(logsData.logs);
    } catch (err) {
      console.error('Failed to fetch container logs:', err);
    }
  };

  const fetchStats = async () => {
    if (!id) return;
    
    try {
      const statsData = await containerApi.getContainerStats(id);
      setStats(statsData);
    } catch (err) {
      console.error('Failed to fetch container stats:', err);
    }
  };

  useEffect(() => {
    fetchContainer();
    fetchLogs();
    fetchStats();

    // Poll for updates every 5 seconds
    const interval = setInterval(() => {
      fetchContainer();
      fetchStats();
    }, 5000);

    return () => clearInterval(interval);
  }, [id]);

  const handleStartContainer = async () => {
    if (!id) return;
    
    try {
      await containerApi.startContainer(id);
      fetchContainer();
    } catch (err) {
      console.error('Failed to start container:', err);
    }
  };

  const handleStopContainer = async () => {
    if (!id) return;
    
    try {
      await containerApi.stopContainer(id);
      fetchContainer();
    } catch (err) {
      console.error('Failed to stop container:', err);
    }
  };

  const handleRestartContainer = async () => {
    if (!id) return;
    
    try {
      await containerApi.restartContainer(id);
      fetchContainer();
    } catch (err) {
      console.error('Failed to restart container:', err);
    }
  };

  const handleRefreshLogs = () => {
    fetchLogs();
  };

  if (loading && !container) {
    return <div className="card">Loading container details...</div>;
  }

  if (error && !container) {
    return <div className="card">Error: {error}</div>;
  }

  if (!container) {
    return <div className="card">Container not found</div>;
  }

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return (
    <div>
      <div className="card-header">
        <h2>
          <Link to="/">Containers</Link> &gt; {container.names.join(', ')}
        </h2>
        <div className="button-group">
          {container.state === 'running' ? (
            <>
              <button
                className="button button-danger"
                onClick={handleStopContainer}
              >
                Stop
              </button>
              <button className="button" onClick={handleRestartContainer}>
                Restart
              </button>
            </>
          ) : (
            <button
              className="button button-success"
              onClick={handleStartContainer}
            >
              Start
            </button>
          )}
        </div>
      </div>

      <div className="card">
        <h3>Container Information</h3>
        <table>
          <tbody>
            <tr>
              <td>ID</td>
              <td>{container.id}</td>
            </tr>
            <tr>
              <td>Name</td>
              <td>{container.names.join(', ')}</td>
            </tr>
            <tr>
              <td>Image</td>
              <td>{container.image}</td>
            </tr>
            <tr>
              <td>Status</td>
              <td
                className={
                  container.state === 'running'
                    ? 'status-running'
                    : 'status-stopped'
                }
              >
                {container.status}
              </td>
            </tr>
            <tr>
              <td>Created</td>
              <td>{new Date(container.created).toLocaleString()}</td>
            </tr>
            <tr>
              <td>Ports</td>
              <td>
                {container.ports.length > 0
                  ? container.ports.map((port) => (
                      <div key={`${port.container_port}-${port.host_port}`}>
                        {port.host_port}:{port.container_port}/{port.protocol}
                      </div>
                    ))
                  : 'None'}
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      {stats && (
        <div className="card">
          <h3>Resource Usage</h3>
          <div className="stats">
            <div className="stat-card">
              <div className="stat-label">CPU Usage</div>
              <div className="stat-value">{stats.cpu_usage_percent.toFixed(2)}%</div>
            </div>
            <div className="stat-card">
              <div className="stat-label">Memory Usage</div>
              <div className="stat-value">
                {formatBytes(stats.memory_usage_bytes)} ({stats.memory_usage_percent.toFixed(2)}%)
              </div>
            </div>
            <div className="stat-card">
              <div className="stat-label">Network I/O</div>
              <div className="stat-value">
                ↓ {formatBytes(stats.network_input_bytes)}
                <br />
                ↑ {formatBytes(stats.network_output_bytes)}
              </div>
            </div>
            <div className="stat-card">
              <div className="stat-label">Block I/O</div>
              <div className="stat-value">
                ↓ {formatBytes(stats.block_input_bytes)}
                <br />
                ↑ {formatBytes(stats.block_output_bytes)}
              </div>
            </div>
            <div className="stat-card">
              <div className="stat-label">Processes</div>
              <div className="stat-value">{stats.process_count}</div>
            </div>
          </div>
        </div>
      )}

      <div className="card">
        <div className="card-header">
          <h3>Logs</h3>
          <button className="button" onClick={handleRefreshLogs}>
            Refresh Logs
          </button>
        </div>
        <div className="logs">
          {logs.length === 0 ? (
            <p>No logs available</p>
          ) : (
            logs.map((log, index) => <div key={index}>{log}</div>)
          )}
        </div>
      </div>
    </div>
  );
};

export default ContainerDetail;