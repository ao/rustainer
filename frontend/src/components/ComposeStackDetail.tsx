import React, { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import api from '../services/api';
import { ComposeStack, StackLogs } from '../types';

const ComposeStackDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  
  const [stack, setStack] = useState<ComposeStack | null>(null);
  const [logs, setLogs] = useState<StackLogs | null>(null);
  const [showLogs, setShowLogs] = useState<boolean>(false);
  const [selectedService, setSelectedService] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!id) return;

    const fetchStack = async () => {
      try {
        setLoading(true);
        const data = await api.compose.getStack(id);
        setStack(data);
        
        // If there are services, select the first one by default
        if (data.services.length > 0) {
          setSelectedService(data.services[0].name);
        }
        
        setLoading(false);
      } catch (err) {
        setError('Failed to fetch stack details');
        setLoading(false);
      }
    };

    fetchStack();
  }, [id]);

  const fetchLogs = async () => {
    if (!id) return;
    
    try {
      setShowLogs(true);
      const logsData = await api.compose.getStackLogs(id);
      setLogs(logsData);
    } catch (err) {
      setError('Failed to fetch logs');
    }
  };

  const handleStartStack = async () => {
    if (!id || !stack) return;
    
    try {
      await api.compose.startStack(id);
      // Fetch the updated stack
      const updatedStack = await api.compose.getStack(id);
      setStack(updatedStack);
    } catch (err) {
      setError('Failed to start stack');
    }
  };

  const handleStopStack = async () => {
    if (!id || !stack) return;
    
    try {
      await api.compose.stopStack(id);
      // Fetch the updated stack
      const updatedStack = await api.compose.getStack(id);
      setStack(updatedStack);
    } catch (err) {
      setError('Failed to stop stack');
    }
  };

  const handleRestartStack = async () => {
    if (!id || !stack) return;
    
    try {
      await api.compose.restartStack(id);
      // Fetch the updated stack
      const updatedStack = await api.compose.getStack(id);
      setStack(updatedStack);
    } catch (err) {
      setError('Failed to restart stack');
    }
  };

  const handleDeleteStack = async () => {
    if (!id) return;
    
    if (window.confirm('Are you sure you want to delete this stack?')) {
      try {
        await api.compose.deleteStack(id);
        navigate('/compose');
      } catch (err) {
        setError('Failed to delete stack');
      }
    }
  };

  const getStatusClass = (status: string) => {
    switch (status) {
      case 'up':
        return 'status-up';
      case 'down':
        return 'status-down';
      case 'partial':
        return 'status-partial';
      case 'error':
        return 'status-error';
      default:
        return '';
    }
  };

  if (loading) {
    return <div>Loading stack details...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  if (!stack) {
    return <div>Stack not found</div>;
  }

  return (
    <div className="compose-stack-detail">
      <div className="header-actions">
        <h2>{stack.name}</h2>
        <div className="actions">
          <Link to={`/compose/${id}/edit`} className="button">
            Edit Stack
          </Link>
          <button 
            onClick={handleStartStack}
            disabled={stack.status === 'up'}
            title="Start Stack"
          >
            Start
          </button>
          <button 
            onClick={handleStopStack}
            disabled={stack.status === 'down'}
            title="Stop Stack"
          >
            Stop
          </button>
          <button 
            onClick={handleRestartStack}
            disabled={stack.status === 'down'}
            title="Restart Stack"
          >
            Restart
          </button>
          <button 
            onClick={handleDeleteStack}
            className="delete-button"
            title="Delete Stack"
          >
            Delete
          </button>
        </div>
      </div>

      <div className="stack-info">
        <div className="info-item">
          <span className="label">Status:</span>
          <span className={`status ${getStatusClass(stack.status)}`}>
            {stack.status}
          </span>
        </div>
        <div className="info-item">
          <span className="label">Created:</span>
          <span>{new Date(stack.created_at).toLocaleString()}</span>
        </div>
        <div className="info-item">
          <span className="label">Last Updated:</span>
          <span>{new Date(stack.updated_at).toLocaleString()}</span>
        </div>
        <div className="info-item">
          <span className="label">Services:</span>
          <span>{stack.services.length}</span>
        </div>
      </div>

      <h3>Services</h3>
      {stack.services.length === 0 ? (
        <p>No services defined in this stack.</p>
      ) : (
        <table className="services-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Image</th>
              <th>Status</th>
              <th>Container ID</th>
            </tr>
          </thead>
          <tbody>
            {stack.services.map(service => (
              <tr key={service.name}>
                <td>{service.name}</td>
                <td>{service.image}</td>
                <td>{service.status}</td>
                <td>{service.container_id || 'N/A'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      <div className="logs-section">
        <h3>Logs</h3>
        {!showLogs ? (
          <button onClick={fetchLogs}>View Logs</button>
        ) : (
          <>
            {logs ? (
              <div className="logs-container">
                <div className="service-selector">
                  {Object.keys(logs).map(serviceName => (
                    <button
                      key={serviceName}
                      className={selectedService === serviceName ? 'active' : ''}
                      onClick={() => setSelectedService(serviceName)}
                    >
                      {serviceName}
                    </button>
                  ))}
                </div>
                <div className="log-output">
                  {selectedService && logs[selectedService] ? (
                    logs[selectedService].map((line, index) => (
                      <div key={index} className="log-line">
                        {line}
                      </div>
                    ))
                  ) : (
                    <p>Select a service to view logs</p>
                  )}
                </div>
              </div>
            ) : (
              <p>Loading logs...</p>
            )}
          </>
        )}
      </div>
    </div>
  );
};

export default ComposeStackDetail;