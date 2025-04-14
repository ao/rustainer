import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import api from '../services/api';
import { ComposeStack } from '../types';

const ComposeStackList: React.FC = () => {
  const [stacks, setStacks] = useState<ComposeStack[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchStacks = async () => {
      try {
        const data = await api.compose.listStacks();
        setStacks(data);
        setLoading(false);
      } catch (err) {
        setError('Failed to fetch stacks');
        setLoading(false);
      }
    };

    fetchStacks();
  }, []);

  const handleStartStack = async (id: string) => {
    try {
      await api.compose.startStack(id);
      // Fetch the updated stack
      const updatedStack = await api.compose.getStack(id);
      setStacks(stacks.map(stack => stack.id === id ? updatedStack : stack));
    } catch (err) {
      setError('Failed to start stack');
    }
  };

  const handleStopStack = async (id: string) => {
    try {
      await api.compose.stopStack(id);
      // Fetch the updated stack
      const updatedStack = await api.compose.getStack(id);
      setStacks(stacks.map(stack => stack.id === id ? updatedStack : stack));
    } catch (err) {
      setError('Failed to stop stack');
    }
  };

  const handleRestartStack = async (id: string) => {
    try {
      await api.compose.restartStack(id);
      // Fetch the updated stack
      const updatedStack = await api.compose.getStack(id);
      setStacks(stacks.map(stack => stack.id === id ? updatedStack : stack));
    } catch (err) {
      setError('Failed to restart stack');
    }
  };

  const handleDeleteStack = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this stack?')) {
      try {
        await api.compose.deleteStack(id);
        setStacks(stacks.filter(stack => stack.id !== id));
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
    return <div>Loading stacks...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div className="compose-stacks">
      <div className="header-actions">
        <h2>Docker Compose Stacks</h2>
        <Link to="/compose/create" className="button">Create Stack</Link>
      </div>

      {stacks.length === 0 ? (
        <p>No stacks found. Create a new stack to get started.</p>
      ) : (
        <table className="stacks-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Status</th>
              <th>Services</th>
              <th>Created</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {stacks.map(stack => (
              <tr key={stack.id}>
                <td>
                  <Link to={`/compose/${stack.id}`}>{stack.name}</Link>
                </td>
                <td>
                  <span className={`status ${getStatusClass(stack.status)}`}>
                    {stack.status}
                  </span>
                </td>
                <td>{stack.services.length}</td>
                <td>{new Date(stack.created_at).toLocaleString()}</td>
                <td className="actions">
                  <button 
                    onClick={() => handleStartStack(stack.id)}
                    disabled={stack.status === 'up'}
                    title="Start Stack"
                  >
                    Start
                  </button>
                  <button 
                    onClick={() => handleStopStack(stack.id)}
                    disabled={stack.status === 'down'}
                    title="Stop Stack"
                  >
                    Stop
                  </button>
                  <button 
                    onClick={() => handleRestartStack(stack.id)}
                    disabled={stack.status === 'down'}
                    title="Restart Stack"
                  >
                    Restart
                  </button>
                  <button 
                    onClick={() => handleDeleteStack(stack.id)}
                    className="delete-button"
                    title="Delete Stack"
                  >
                    Delete
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};

export default ComposeStackList;