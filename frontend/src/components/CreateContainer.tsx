import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { containerApi } from '../services/api';

const CreateContainer: React.FC = () => {
  const navigate = useNavigate();
  const [image, setImage] = useState<string>('');
  const [name, setName] = useState<string>('');
  const [ports, setPorts] = useState<string>('');
  const [env, setEnv] = useState<string>('');
  const [volumes, setVolumes] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!image) {
      setError('Image is required');
      return;
    }
    
    if (!name) {
      setError('Name is required');
      return;
    }
    
    try {
      setLoading(true);
      setError(null);
      
      const request = {
        image,
        name,
        ports: ports ? ports.split(',').map(p => p.trim()) : undefined,
        env: env ? env.split(',').map(e => e.trim()) : undefined,
        volumes: volumes ? volumes.split(',').map(v => v.trim()) : undefined,
      };
      
      const container = await containerApi.createContainer(request);
      navigate(`/containers/${container.id}`);
    } catch (err) {
      setError('Failed to create container');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <div className="card-header">
        <h2>Create Container</h2>
      </div>

      <div className="card">
        {error && <div className="error">{error}</div>}
        
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="image">Image *</label>
            <input
              type="text"
              id="image"
              className="form-control"
              value={image}
              onChange={(e) => setImage(e.target.value)}
              placeholder="e.g., nginx:latest"
              required
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="name">Name *</label>
            <input
              type="text"
              id="name"
              className="form-control"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., my-nginx"
              required
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="ports">Ports (host:container, comma separated)</label>
            <input
              type="text"
              id="ports"
              className="form-control"
              value={ports}
              onChange={(e) => setPorts(e.target.value)}
              placeholder="e.g., 8080:80, 8443:443"
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="env">Environment Variables (comma separated)</label>
            <input
              type="text"
              id="env"
              className="form-control"
              value={env}
              onChange={(e) => setEnv(e.target.value)}
              placeholder="e.g., MYSQL_ROOT_PASSWORD=password, MYSQL_DATABASE=mydb"
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="volumes">Volumes (host:container, comma separated)</label>
            <input
              type="text"
              id="volumes"
              className="form-control"
              value={volumes}
              onChange={(e) => setVolumes(e.target.value)}
              placeholder="e.g., /host/path:/container/path, /another/path:/another/container/path"
            />
          </div>
          
          <button
            type="submit"
            className="button button-success"
            disabled={loading}
          >
            {loading ? 'Creating...' : 'Create Container'}
          </button>
        </form>
      </div>
    </div>
  );
};

export default CreateContainer;