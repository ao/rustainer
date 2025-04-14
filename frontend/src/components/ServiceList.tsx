import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { Service } from '../types';
import api from '../services/api';

const ServiceList: React.FC = () => {
  const [services, setServices] = useState<Service[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchServices = async () => {
      try {
        const data = await api.services.listServices();
        setServices(data);
        setLoading(false);
      } catch (err) {
        console.error('Failed to fetch services:', err);
        setError('Failed to load services. Please try again later.');
        setLoading(false);
      }
    };

    fetchServices();
  }, []);

  const handleToggleService = async (id: string, enabled: boolean) => {
    try {
      if (enabled) {
        await api.services.disableService(id);
      } else {
        await api.services.enableService(id);
      }
      
      // Refresh the list
      const data = await api.services.listServices();
      setServices(data);
    } catch (err) {
      console.error('Failed to toggle service:', err);
      setError('Failed to update service. Please try again later.');
    }
  };

  const handleDeleteService = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this service?')) {
      return;
    }

    try {
      await api.services.deleteService(id);
      
      // Refresh the list
      const data = await api.services.listServices();
      setServices(data);
    } catch (err) {
      console.error('Failed to delete service:', err);
      setError('Failed to delete service. Please try again later.');
    }
  };

  if (loading) {
    return <div className="loading">Loading services...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div className="service-list">
      <div className="service-list-header">
        <h2>Services</h2>
        <Link to="/services/create" className="button">
          Create Service
        </Link>
      </div>

      {services.length === 0 ? (
        <div className="empty-state">
          <p>No services found. Create a new service to get started.</p>
        </div>
      ) : (
        <table className="data-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Domain</th>
              <th>Type</th>
              <th>Target</th>
              <th>Port</th>
              <th>SSL</th>
              <th>Status</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {services.map((service) => (
              <tr key={service.id} className={!service.enabled ? 'disabled' : ''}>
                <td>{service.name}</td>
                <td>{service.domain}</td>
                <td>{service.service_type}</td>
                <td>{service.target}</td>
                <td>{service.port}</td>
                <td>{service.ssl.enabled ? 'Enabled' : 'Disabled'}</td>
                <td>
                  <span className={`status ${service.enabled ? 'active' : 'inactive'}`}>
                    {service.enabled ? 'Active' : 'Inactive'}
                  </span>
                </td>
                <td className="actions">
                  <Link to={`/services/${service.id}`} className="button small">
                    Edit
                  </Link>
                  <button
                    className={`button small ${service.enabled ? 'warning' : 'success'}`}
                    onClick={() => handleToggleService(service.id, service.enabled)}
                  >
                    {service.enabled ? 'Disable' : 'Enable'}
                  </button>
                  <button
                    className="button small danger"
                    onClick={() => handleDeleteService(service.id)}
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

export default ServiceList;