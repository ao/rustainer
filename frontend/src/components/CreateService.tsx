import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { CreateServiceRequest } from '../types';
import api from '../services/api';

const CreateService: React.FC = () => {
  const navigate = useNavigate();
  const [formData, setFormData] = useState<CreateServiceRequest>({
    name: '',
    domain: '',
    service_type: 'Container',
    target: '',
    port: 80,
    ssl: {
      enabled: false,
      auto_generate: false,
    },
    headers: {},
  });
  const [saving, setSaving] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target as HTMLInputElement;
    
    if (type === 'checkbox') {
      const checked = (e.target as HTMLInputElement).checked;
      setFormData((prev) => ({
        ...prev,
        [name]: checked,
      }));
    } else if (name === 'port') {
      setFormData((prev) => ({
        ...prev,
        [name]: parseInt(value, 10),
      }));
    } else {
      setFormData((prev) => ({
        ...prev,
        [name]: value,
      }));
    }
  };

  const handleSslChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, checked } = e.target;
    
    setFormData((prev) => {
      // Ensure ssl object exists
      const ssl = prev.ssl || { enabled: false, auto_generate: false };
      
      return {
        ...prev,
        ssl: {
          ...ssl,
          [name === 'ssl_enabled' ? 'enabled' : name]: checked,
        },
      };
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    setSaving(true);
    setError(null);
    
    try {
      await api.services.createService(formData);
      navigate('/services');
    } catch (err) {
      console.error('Failed to create service:', err);
      setError('Failed to create service. Please try again later.');
      setSaving(false);
    }
  };

  return (
    <div className="create-service">
      <h2>Create New Service</h2>
      
      {error && <div className="error-message">{error}</div>}
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="name">Name</label>
          <input
            type="text"
            id="name"
            name="name"
            value={formData.name}
            onChange={handleChange}
            required
            placeholder="My Website"
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="domain">Domain</label>
          <input
            type="text"
            id="domain"
            name="domain"
            value={formData.domain}
            onChange={handleChange}
            required
            placeholder="example.com"
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="service_type">Type</label>
          <select
            id="service_type"
            name="service_type"
            value={formData.service_type}
            onChange={handleChange}
            required
          >
            <option value="Container">Container</option>
            <option value="StaticSite">Static Site</option>
            <option value="CustomURL">Custom URL</option>
          </select>
        </div>
        
        <div className="form-group">
          <label htmlFor="target">Target</label>
          <input
            type="text"
            id="target"
            name="target"
            value={formData.target}
            onChange={handleChange}
            required
            placeholder={
              formData.service_type === 'Container'
                ? 'web-container'
                : formData.service_type === 'StaticSite'
                ? '/var/www/html'
                : 'https://api.example.com'
            }
          />
          <small>
            {formData.service_type === 'Container' && 'Container name or ID'}
            {formData.service_type === 'StaticSite' && 'Path to static files'}
            {formData.service_type === 'CustomURL' && 'URL to proxy to'}
          </small>
        </div>
        
        <div className="form-group">
          <label htmlFor="port">Port</label>
          <input
            type="number"
            id="port"
            name="port"
            value={formData.port}
            onChange={handleChange}
            required
            min="1"
            max="65535"
          />
        </div>
        
        <div className="form-group">
          <label>SSL Configuration</label>
          <div className="checkbox-group">
            <input
              type="checkbox"
              id="ssl_enabled"
              name="ssl_enabled"
              checked={formData.ssl?.enabled || false}
              onChange={handleSslChange}
            />
            <label htmlFor="ssl_enabled">Enable SSL</label>
          </div>
          
          <div className="checkbox-group">
            <input
              type="checkbox"
              id="auto_generate"
              name="auto_generate"
              checked={formData.ssl?.auto_generate || false}
              onChange={handleSslChange}
              disabled={!formData.ssl?.enabled}
            />
            <label htmlFor="auto_generate">Auto-generate certificate (Let's Encrypt)</label>
          </div>
        </div>
        
        <div className="form-actions">
          <button
            type="button"
            className="button secondary"
            onClick={() => navigate('/services')}
            disabled={saving}
          >
            Cancel
          </button>
          <button type="submit" className="button primary" disabled={saving}>
            {saving ? 'Creating...' : 'Create Service'}
          </button>
        </div>
      </form>
    </div>
  );
};

export default CreateService;