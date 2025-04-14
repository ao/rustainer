import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { ContainerTemplate } from '../types';
import api from '../services/api';

const TemplateList: React.FC = () => {
  const [templates, setTemplates] = useState<ContainerTemplate[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<string>('');
  const [categoryFilter, setCategoryFilter] = useState<string>('');
  const [categories, setCategories] = useState<string[]>([]);

  useEffect(() => {
    fetchTemplates();
  }, []);

  const fetchTemplates = async () => {
    try {
      setLoading(true);
      const data = await api.templates.listTemplates();
      setTemplates(data);
      
      // Extract unique categories
      const uniqueCategories = Array.from(new Set(data.map(template => template.category)));
      setCategories(uniqueCategories);
      
      setLoading(false);
    } catch (err) {
      setError('Failed to fetch templates');
      setLoading(false);
      console.error('Error fetching templates:', err);
    }
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this template?')) {
      try {
        await api.templates.deleteTemplate(id);
        setTemplates(templates.filter(template => template.id !== id));
      } catch (err) {
        setError('Failed to delete template');
        console.error('Error deleting template:', err);
      }
    }
  };

  const filteredTemplates = templates.filter(template => {
    const matchesSearch = template.name.toLowerCase().includes(filter.toLowerCase()) ||
                         template.description.toLowerCase().includes(filter.toLowerCase()) ||
                         template.image.toLowerCase().includes(filter.toLowerCase());
    
    const matchesCategory = categoryFilter === '' || template.category === categoryFilter;
    
    return matchesSearch && matchesCategory;
  });

  if (loading) {
    return <div className="loading">Loading templates...</div>;
  }

  if (error) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div className="template-list">
      <div className="header-actions">
        <h2>Container Templates</h2>
        <Link to="/templates/create" className="button">Create Template</Link>
      </div>
      
      <div className="filters">
        <div className="search-box">
          <input
            type="text"
            placeholder="Search templates..."
            value={filter}
            onChange={(e) => setFilter(e.target.value)}
          />
        </div>
        
        <div className="category-filter">
          <select 
            value={categoryFilter} 
            onChange={(e) => setCategoryFilter(e.target.value)}
          >
            <option value="">All Categories</option>
            {categories.map(category => (
              <option key={category} value={category}>{category}</option>
            ))}
          </select>
        </div>
      </div>
      
      {filteredTemplates.length === 0 ? (
        <div className="no-templates">
          No templates found. {templates.length > 0 ? 'Try adjusting your filters.' : 'Create your first template!'}
        </div>
      ) : (
        <div className="template-grid">
          {filteredTemplates.map(template => (
            <div key={template.id} className="template-card card">
              <div className="card-header">
                <h3 className="card-title">{template.name}</h3>
                <span className="template-category">{template.category}</span>
              </div>
              
              <div className="template-description">
                {template.description}
              </div>
              
              <div className="template-details">
                <div className="template-image">
                  <strong>Image:</strong> {template.image}:{template.tag}
                </div>
                
                <div className="template-ports">
                  <strong>Ports:</strong> {template.ports.length > 0 
                    ? template.ports.map(p => `${p.host_port || 'auto'}:${p.container_port}`).join(', ') 
                    : 'None'}
                </div>
                
                <div className="template-volumes">
                  <strong>Volumes:</strong> {template.volumes.length > 0 
                    ? `${template.volumes.length} volume(s)` 
                    : 'None'}
                </div>
              </div>
              
              <div className="template-actions">
                <Link to={`/templates/${template.id}`} className="button">View</Link>
                <Link to={`/templates/${template.id}/deploy`} className="button">Deploy</Link>
                <button onClick={() => handleDelete(template.id)} className="button danger">Delete</button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default TemplateList;