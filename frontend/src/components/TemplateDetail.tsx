import React, { useState, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { ContainerTemplate, DeployTemplateRequest } from '../types';
import { templateApi } from '../services/api';

const TemplateDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [template, setTemplate] = useState<ContainerTemplate | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [deployName, setDeployName] = useState<string>('');
  const [showDeployForm, setShowDeployForm] = useState<boolean>(false);
  const [deploying, setDeploying] = useState<boolean>(false);

  useEffect(() => {
    if (id) {
      fetchTemplate(id);
    }
  }, [id]);

  const fetchTemplate = async (templateId: string) => {
    try {
      setLoading(true);
      const data = await templateApi.getTemplate(templateId);
      setTemplate(data);
      setDeployName(data.name.toLowerCase().replace(/[^a-z0-9]/g, '-'));
      setLoading(false);
    } catch (err) {
      setError('Failed to fetch template details');
      setLoading(false);
      console.error('Error fetching template:', err);
    }
  };

  const handleDelete = async () => {
    if (!template || !id) return;
    
    if (window.confirm(`Are you sure you want to delete the template "${template.name}"?`)) {
      try {
        await templateApi.deleteTemplate(id);
        navigate('/templates');
      } catch (err) {
        setError('Failed to delete template');
        console.error('Error deleting template:', err);
      }
    }
  };

  const handleDeploy = async () => {
    if (!template || !id) return;
    
    try {
      setDeploying(true);
      
      const deployRequest: DeployTemplateRequest = {
        template_id: id,
        name: deployName,
      };
      
      const containerId = await templateApi.deployTemplate(deployRequest);
      setDeploying(false);
      setShowDeployForm(false);
      
      // Navigate to the container detail page
      navigate(`/containers/${containerId}`);
    } catch (err) {
      setError('Failed to deploy container');
      setDeploying(false);
      console.error('Error deploying container:', err);
    }
  };

  if (loading) {
    return <div className="loading">Loading template details...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  if (!template) {
    return <div className="error-message">Template not found</div>;
  }

  return (
    <div className="template-detail">
      <div className="header-actions">
        <h2>{template.name}</h2>
        <div>
          <button onClick={() => setShowDeployForm(true)} className="button">Deploy</button>
          <Link to={`/templates/${id}/edit`} className="button">Edit</Link>
          <button onClick={handleDelete} className="button danger">Delete</button>
        </div>
      </div>
      
      <div className="card">
        <div className="template-info">
          <div className="template-header">
            <span className="template-category">{template.category}</span>
            <span className="template-version">v{template.version}</span>
          </div>
          
          <div className="template-description">
            {template.description}
          </div>
          
          <div className="template-section">
            <h3>Image</h3>
            <div className="template-image-info">
              <code>{template.image}:{template.tag}</code>
            </div>
          </div>
          
          {template.command && (
            <div className="template-section">
              <h3>Command</h3>
              <div className="template-command">
                <code>{template.command}</code>
              </div>
            </div>
          )}
          
          <div className="template-section">
            <h3>Port Mappings</h3>
            {template.ports.length > 0 ? (
              <table className="template-ports-table">
                <thead>
                  <tr>
                    <th>Host Port</th>
                    <th>Container Port</th>
                    <th>Protocol</th>
                  </tr>
                </thead>
                <tbody>
                  {template.ports.map((port, index) => (
                    <tr key={index}>
                      <td>{port.host_port || 'auto'}</td>
                      <td>{port.container_port}</td>
                      <td>{port.protocol}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <div className="no-data">No port mappings defined</div>
            )}
          </div>
          
          <div className="template-section">
            <h3>Volume Mappings</h3>
            {template.volumes.length > 0 ? (
              <table className="template-volumes-table">
                <thead>
                  <tr>
                    <th>Host Path</th>
                    <th>Container Path</th>
                    <th>Mode</th>
                  </tr>
                </thead>
                <tbody>
                  {template.volumes.map((volume, index) => (
                    <tr key={index}>
                      <td>{volume.host_path}</td>
                      <td>{volume.container_path}</td>
                      <td>{volume.read_only ? 'read-only' : 'read-write'}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <div className="no-data">No volume mappings defined</div>
            )}
          </div>
          
          <div className="template-section">
            <h3>Environment Variables</h3>
            {Object.keys(template.env).length > 0 ? (
              <table className="template-env-table">
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>Value</th>
                  </tr>
                </thead>
                <tbody>
                  {Object.entries(template.env).map(([key, value], index) => (
                    <tr key={index}>
                      <td>{key}</td>
                      <td>{value}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            ) : (
              <div className="no-data">No environment variables defined</div>
            )}
          </div>
          
          <div className="template-section">
            <h3>Advanced Settings</h3>
            <div className="template-advanced-settings">
              <div className="setting">
                <span className="setting-name">Network Mode:</span>
                <span className="setting-value">{template.network_mode || 'default'}</span>
              </div>
              <div className="setting">
                <span className="setting-name">Restart Policy:</span>
                <span className="setting-value">{template.restart_policy || 'no'}</span>
              </div>
              {template.resources && (
                <>
                  {template.resources.cpu && (
                    <div className="setting">
                      <span className="setting-name">CPU Limit:</span>
                      <span className="setting-value">{template.resources.cpu} cores</span>
                    </div>
                  )}
                  {template.resources.memory && (
                    <div className="setting">
                      <span className="setting-name">Memory Limit:</span>
                      <span className="setting-value">{(template.resources.memory / (1024 * 1024)).toFixed(0)} MB</span>
                    </div>
                  )}
                </>
              )}
            </div>
          </div>
          
          <div className="template-metadata">
            <div>Created: {new Date(template.created_at).toLocaleString()}</div>
            <div>Last Updated: {new Date(template.updated_at).toLocaleString()}</div>
            {template.created_by && <div>Created by: {template.created_by}</div>}
          </div>
        </div>
      </div>
      
      {showDeployForm && (
        <div className="modal">
          <div className="modal-content">
            <div className="modal-header">
              <h3>Deploy Container from Template</h3>
              <button onClick={() => setShowDeployForm(false)} className="close-button">&times;</button>
            </div>
            <div className="modal-body">
              <div className="form-group">
                <label htmlFor="container-name">Container Name:</label>
                <input
                  type="text"
                  id="container-name"
                  value={deployName}
                  onChange={(e) => setDeployName(e.target.value)}
                  disabled={deploying}
                />
              </div>
              
              <div className="form-actions">
                <button 
                  onClick={handleDeploy} 
                  className="button" 
                  disabled={deploying || !deployName.trim()}
                >
                  {deploying ? 'Deploying...' : 'Deploy Container'}
                </button>
                <button 
                  onClick={() => setShowDeployForm(false)} 
                  className="button secondary"
                  disabled={deploying}
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

export default TemplateDetail;