import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { 
  ContainerTemplate, 
  CreateTemplateRequest, 
  UpdateTemplateRequest, 
  PortMapping, 
  VolumeMapping, 
  ResourceLimits 
} from '../types';
import { templateApi } from '../services/api';

interface TemplateEditorProps {
  isEdit?: boolean;
}

const TemplateEditor: React.FC<TemplateEditorProps> = ({ isEdit = false }) => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  
  const [loading, setLoading] = useState<boolean>(isEdit);
  const [saving, setSaving] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  
  // Form state
  const [name, setName] = useState<string>('');
  const [description, setDescription] = useState<string>('');
  const [category, setCategory] = useState<string>('');
  const [image, setImage] = useState<string>('');
  const [tag, setTag] = useState<string>('latest');
  const [command, setCommand] = useState<string>('');
  const [ports, setPorts] = useState<PortMapping[]>([]);
  const [volumes, setVolumes] = useState<VolumeMapping[]>([]);
  const [env, setEnv] = useState<{ key: string; value: string }[]>([]);
  const [networkMode, setNetworkMode] = useState<string>('');
  const [restartPolicy, setRestartPolicy] = useState<string>('no');
  const [resources, setResources] = useState<ResourceLimits>({});
  
  // New port/volume/env state
  const [newPort, setNewPort] = useState<PortMapping>({ 
    host_port: undefined, 
    container_port: 0, 
    protocol: 'tcp' 
  });
  const [newVolume, setNewVolume] = useState<VolumeMapping>({ 
    host_path: '', 
    container_path: '', 
    read_only: false 
  });
  const [newEnv, setNewEnv] = useState<{ key: string; value: string }>({ 
    key: '', 
    value: '' 
  });
  
  useEffect(() => {
    if (isEdit && id) {
      fetchTemplate(id);
    }
  }, [isEdit, id]);
  
  const fetchTemplate = async (templateId: string) => {
    try {
      setLoading(true);
      const template = await templateApi.getTemplate(templateId);
      
      // Populate form with template data
      setName(template.name);
      setDescription(template.description);
      setCategory(template.category);
      setImage(template.image);
      setTag(template.tag);
      setCommand(template.command || '');
      setPorts(template.ports);
      setVolumes(template.volumes);
      
      // Convert env object to array of key-value pairs
      const envArray = Object.entries(template.env).map(([key, value]) => ({ key, value }));
      setEnv(envArray);
      
      setNetworkMode(template.network_mode || '');
      setRestartPolicy(template.restart_policy || 'no');
      setResources(template.resources || {});
      
      setLoading(false);
    } catch (err) {
      setError('Failed to fetch template');
      setLoading(false);
      console.error('Error fetching template:', err);
    }
  };
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    try {
      setSaving(true);
      
      // Convert env array to object
      const envObject: Record<string, string> = {};
      env.forEach(({ key, value }) => {
        if (key.trim()) {
          envObject[key] = value;
        }
      });
      
      if (isEdit && id) {
        // Update existing template
        const updateRequest: UpdateTemplateRequest = {
          name,
          description,
          category,
          image,
          tag,
          command: command || undefined,
          env: envObject,
          ports,
          volumes,
          network_mode: networkMode || undefined,
          restart_policy: restartPolicy || undefined,
          resources: Object.keys(resources).length > 0 ? resources : undefined,
        };
        
        await templateApi.updateTemplate(id, updateRequest);
        navigate(`/templates/${id}`);
      } else {
        // Create new template
        const createRequest: CreateTemplateRequest = {
          name,
          description,
          category,
          image,
          tag,
          command: command || undefined,
          env: envObject,
          ports,
          volumes,
          network_mode: networkMode || undefined,
          restart_policy: restartPolicy || undefined,
          resources: Object.keys(resources).length > 0 ? resources : undefined,
          labels: {},
        };
        
        const template = await templateApi.createTemplate(createRequest);
        navigate(`/templates/${template.id}`);
      }
      
      setSaving(false);
    } catch (err) {
      setError(`Failed to ${isEdit ? 'update' : 'create'} template`);
      setSaving(false);
      console.error(`Error ${isEdit ? 'updating' : 'creating'} template:`, err);
    }
  };
  
  // Port handlers
  const addPort = () => {
    if (newPort.container_port > 0) {
      setPorts([...ports, { ...newPort }]);
      setNewPort({ host_port: undefined, container_port: 0, protocol: 'tcp' });
    }
  };
  
  const removePort = (index: number) => {
    setPorts(ports.filter((_, i) => i !== index));
  };
  
  // Volume handlers
  const addVolume = () => {
    if (newVolume.host_path && newVolume.container_path) {
      setVolumes([...volumes, { ...newVolume }]);
      setNewVolume({ host_path: '', container_path: '', read_only: false });
    }
  };
  
  const removeVolume = (index: number) => {
    setVolumes(volumes.filter((_, i) => i !== index));
  };
  
  // Environment variable handlers
  const addEnv = () => {
    if (newEnv.key) {
      setEnv([...env, { ...newEnv }]);
      setNewEnv({ key: '', value: '' });
    }
  };
  
  const removeEnv = (index: number) => {
    setEnv(env.filter((_, i) => i !== index));
  };
  
  // Resource handlers
  const updateResources = (field: keyof ResourceLimits, value: string) => {
    const numValue = value ? parseFloat(value) : undefined;
    setResources({ ...resources, [field]: numValue });
  };
  
  if (loading) {
    return <div className="loading">Loading template...</div>;
  }
  
  return (
    <div className="template-editor">
      <div className="header-actions">
        <h2>{isEdit ? 'Edit Template' : 'Create Template'}</h2>
      </div>
      
      {error && <div className="error-message">{error}</div>}
      
      <form onSubmit={handleSubmit} className="template-form">
        <div className="form-section">
          <h3>Basic Information</h3>
          
          <div className="form-group">
            <label htmlFor="name">Name:</label>
            <input
              type="text"
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
              disabled={saving}
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="description">Description:</label>
            <textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              required
              disabled={saving}
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="category">Category:</label>
            <input
              type="text"
              id="category"
              value={category}
              onChange={(e) => setCategory(e.target.value)}
              required
              disabled={saving}
              placeholder="e.g., Database, Web Server, etc."
            />
          </div>
        </div>
        
        <div className="form-section">
          <h3>Image Configuration</h3>
          
          <div className="form-row">
            <div className="form-group">
              <label htmlFor="image">Image:</label>
              <input
                type="text"
                id="image"
                value={image}
                onChange={(e) => setImage(e.target.value)}
                required
                disabled={saving}
                placeholder="e.g., nginx"
              />
            </div>
            
            <div className="form-group">
              <label htmlFor="tag">Tag:</label>
              <input
                type="text"
                id="tag"
                value={tag}
                onChange={(e) => setTag(e.target.value)}
                required
                disabled={saving}
                placeholder="e.g., latest"
              />
            </div>
          </div>
          
          <div className="form-group">
            <label htmlFor="command">Command (optional):</label>
            <input
              type="text"
              id="command"
              value={command}
              onChange={(e) => setCommand(e.target.value)}
              disabled={saving}
              placeholder="e.g., nginx -g 'daemon off;'"
            />
          </div>
        </div>
        
        <div className="form-section">
          <h3>Port Mappings</h3>
          
          <table className="form-table">
            <thead>
              <tr>
                <th>Host Port</th>
                <th>Container Port</th>
                <th>Protocol</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {ports.map((port, index) => (
                <tr key={index}>
                  <td>{port.host_port || 'auto'}</td>
                  <td>{port.container_port}</td>
                  <td>{port.protocol}</td>
                  <td>
                    <button
                      type="button"
                      onClick={() => removePort(index)}
                      disabled={saving}
                      className="button danger"
                    >
                      Remove
                    </button>
                  </td>
                </tr>
              ))}
              <tr>
                <td>
                  <input
                    type="number"
                    value={newPort.host_port || ''}
                    onChange={(e) => setNewPort({
                      ...newPort,
                      host_port: e.target.value ? parseInt(e.target.value) : undefined
                    })}
                    disabled={saving}
                    placeholder="Auto"
                  />
                </td>
                <td>
                  <input
                    type="number"
                    value={newPort.container_port || ''}
                    onChange={(e) => setNewPort({
                      ...newPort,
                      container_port: parseInt(e.target.value) || 0
                    })}
                    required
                    disabled={saving}
                  />
                </td>
                <td>
                  <select
                    value={newPort.protocol}
                    onChange={(e) => setNewPort({
                      ...newPort,
                      protocol: e.target.value
                    })}
                    disabled={saving}
                  >
                    <option value="tcp">TCP</option>
                    <option value="udp">UDP</option>
                  </select>
                </td>
                <td>
                  <button
                    type="button"
                    onClick={addPort}
                    disabled={saving || !newPort.container_port}
                    className="button"
                  >
                    Add
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        
        <div className="form-section">
          <h3>Volume Mappings</h3>
          
          <table className="form-table">
            <thead>
              <tr>
                <th>Host Path</th>
                <th>Container Path</th>
                <th>Mode</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {volumes.map((volume, index) => (
                <tr key={index}>
                  <td>{volume.host_path}</td>
                  <td>{volume.container_path}</td>
                  <td>{volume.read_only ? 'Read-only' : 'Read-write'}</td>
                  <td>
                    <button
                      type="button"
                      onClick={() => removeVolume(index)}
                      disabled={saving}
                      className="button danger"
                    >
                      Remove
                    </button>
                  </td>
                </tr>
              ))}
              <tr>
                <td>
                  <input
                    type="text"
                    value={newVolume.host_path}
                    onChange={(e) => setNewVolume({
                      ...newVolume,
                      host_path: e.target.value
                    })}
                    disabled={saving}
                    placeholder="e.g., ./data"
                  />
                </td>
                <td>
                  <input
                    type="text"
                    value={newVolume.container_path}
                    onChange={(e) => setNewVolume({
                      ...newVolume,
                      container_path: e.target.value
                    })}
                    disabled={saving}
                    placeholder="e.g., /data"
                  />
                </td>
                <td>
                  <select
                    value={newVolume.read_only ? 'ro' : 'rw'}
                    onChange={(e) => setNewVolume({
                      ...newVolume,
                      read_only: e.target.value === 'ro'
                    })}
                    disabled={saving}
                  >
                    <option value="rw">Read-write</option>
                    <option value="ro">Read-only</option>
                  </select>
                </td>
                <td>
                  <button
                    type="button"
                    onClick={addVolume}
                    disabled={saving || !newVolume.host_path || !newVolume.container_path}
                    className="button"
                  >
                    Add
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        
        <div className="form-section">
          <h3>Environment Variables</h3>
          
          <table className="form-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Value</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {env.map((variable, index) => (
                <tr key={index}>
                  <td>{variable.key}</td>
                  <td>{variable.value}</td>
                  <td>
                    <button
                      type="button"
                      onClick={() => removeEnv(index)}
                      disabled={saving}
                      className="button danger"
                    >
                      Remove
                    </button>
                  </td>
                </tr>
              ))}
              <tr>
                <td>
                  <input
                    type="text"
                    value={newEnv.key}
                    onChange={(e) => setNewEnv({
                      ...newEnv,
                      key: e.target.value
                    })}
                    disabled={saving}
                    placeholder="e.g., POSTGRES_PASSWORD"
                  />
                </td>
                <td>
                  <input
                    type="text"
                    value={newEnv.value}
                    onChange={(e) => setNewEnv({
                      ...newEnv,
                      value: e.target.value
                    })}
                    disabled={saving}
                    placeholder="e.g., password123"
                  />
                </td>
                <td>
                  <button
                    type="button"
                    onClick={addEnv}
                    disabled={saving || !newEnv.key}
                    className="button"
                  >
                    Add
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        
        <div className="form-section">
          <h3>Advanced Settings</h3>
          
          <div className="form-group">
            <label htmlFor="network-mode">Network Mode:</label>
            <select
              id="network-mode"
              value={networkMode}
              onChange={(e) => setNetworkMode(e.target.value)}
              disabled={saving}
            >
              <option value="">Default</option>
              <option value="bridge">Bridge</option>
              <option value="host">Host</option>
              <option value="none">None</option>
            </select>
          </div>
          
          <div className="form-group">
            <label htmlFor="restart-policy">Restart Policy:</label>
            <select
              id="restart-policy"
              value={restartPolicy}
              onChange={(e) => setRestartPolicy(e.target.value)}
              disabled={saving}
            >
              <option value="no">No</option>
              <option value="always">Always</option>
              <option value="unless-stopped">Unless Stopped</option>
              <option value="on-failure">On Failure</option>
            </select>
          </div>
          
          <div className="form-group">
            <label htmlFor="cpu-limit">CPU Limit (cores):</label>
            <input
              type="number"
              id="cpu-limit"
              value={resources.cpu || ''}
              onChange={(e) => updateResources('cpu', e.target.value)}
              step="0.1"
              min="0"
              disabled={saving}
              placeholder="e.g., 1.0"
            />
          </div>
          
          <div className="form-group">
            <label htmlFor="memory-limit">Memory Limit (MB):</label>
            <input
              type="number"
              id="memory-limit"
              value={resources.memory ? resources.memory / (1024 * 1024) : ''}
              onChange={(e) => updateResources('memory', e.target.value ? (parseFloat(e.target.value) * 1024 * 1024).toString() : '')}
              step="1"
              min="0"
              disabled={saving}
              placeholder="e.g., 512"
            />
          </div>
        </div>
        
        <div className="form-actions">
          <button
            type="submit"
            className="button"
            disabled={saving || !name || !description || !category || !image || !tag}
          >
            {saving ? 'Saving...' : isEdit ? 'Update Template' : 'Create Template'}
          </button>
          <button
            type="button"
            className="button secondary"
            onClick={() => navigate(isEdit && id ? `/templates/${id}` : '/templates')}
            disabled={saving}
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  );
};

export default TemplateEditor;