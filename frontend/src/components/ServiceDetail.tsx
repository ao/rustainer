import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Service, UpdateServiceRequest } from '../types';
import api from '../services/api';

const ServiceDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [service, setService] = useState<Service | null>(null);
  const [formData, setFormData] = useState<UpdateServiceRequest>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [saving, setSaving] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchService = async () => {
      if (!id) return;
      
      try {
        const data = await api.services.getService(id);
        setService(data);
        setFormData({
          name: data.name,
          domain: data.domain,
          service_type: data.service_type,
          target: data.target,
          port: data.port,
          ssl: data.ssl,
          headers: data.headers,
          enabled: data.enabled,
        });
        setLoading(false);
      } catch (err) {
        console.error('Failed to fetch service:', err);
        setError('Failed to load service. Please try again later.');
        setLoading(false);
      }
    };

    fetchService();
  }, [id]);

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
    if (!id) return;
    
    setSaving(true);
    setError(null);
    
    try {
      await api.services.updateService(id, formData);
      navigate('/services');
    } catch (err) {
      console.error('Failed to update service:', err);
      setError('Failed to update service. Please try again later.');
      setSaving(false);
    }
  };

  if (loading) {
    return <div className="loading">Loading service...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  if (!service) {
    return <div className="error-message">Service not found</div>;
  }

  return (
    <div className="service-detail">
      <h2>Edit Service: {service.name}</h2>
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="name">Name</label>
          <input
            type="text"
            id="name"
            name="name"
            value={formData.name || ''}
            onChange={handleChange}
            required
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="domain">Domain</label>
          <input
            type="text"
            id="domain"
            name="domain"
            value={formData.domain || ''}
            onChange={handleChange}
            required
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="service_type">Type</label>
          <select
            id="service_type"
            name="service_type"
            value={formData.service_type || 'Container'}
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
            value={formData.target || ''}
            onChange={handleChange}
            required
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
            value={formData.port || 80}
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
        
        <div className="form-group">
          <label>Status</label>
          <div className="checkbox-group">
            <input
              type="checkbox"
              id="enabled"
              name="enabled"
              checked={formData.enabled || false}
              onChange={handleChange}
            />
            <label htmlFor="enabled">Service Enabled</label>
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
            {saving ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </form>
    </div>
  );
};

export default ServiceDetail;