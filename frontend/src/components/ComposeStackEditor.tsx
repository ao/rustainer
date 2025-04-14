import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { composeApi } from '../services/api';
import { ComposeStack, CreateStackRequest, UpdateStackRequest } from '../types';

interface ComposeStackEditorProps {
  isEdit?: boolean;
}

const ComposeStackEditor: React.FC<ComposeStackEditorProps> = ({ isEdit = false }) => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  
  const [name, setName] = useState<string>('');
  const [composeContent, setComposeContent] = useState<string>('');
  const [startAfterCreate, setStartAfterCreate] = useState<boolean>(true);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // If in edit mode, fetch the existing stack
    if (isEdit && id) {
      const fetchStack = async () => {
        try {
          setLoading(true);
          const stack = await composeApi.getStack(id);
          setName(stack.name);
          // Fetch the compose file content
          // In a real implementation, we would need an API endpoint to get the file content
          setComposeContent('version: "3"\nservices:\n  # Your services here');
          setLoading(false);
        } catch (err) {
          setError('Failed to fetch stack details');
          setLoading(false);
        }
      };

      fetchStack();
    }
  }, [isEdit, id]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (isEdit && id) {
        // Update existing stack
        const request: UpdateStackRequest = {
          compose_content: composeContent,
          restart: startAfterCreate,
        };
        await composeApi.updateStack(id, request);
        navigate(`/compose/${id}`);
      } else {
        // Create new stack
        const request: CreateStackRequest = {
          name,
          compose_content: composeContent,
          start: startAfterCreate,
        };
        const newStack = await composeApi.createStack(request);
        navigate(`/compose/${newStack.id}`);
      }
    } catch (err) {
      setError(isEdit ? 'Failed to update stack' : 'Failed to create stack');
    } finally {
      setLoading(false);
    }
  };

  if (loading && isEdit) {
    return <div>Loading stack details...</div>;
  }

  return (
    <div className="compose-editor">
      <h2>{isEdit ? 'Edit Stack' : 'Create New Stack'}</h2>
      
      {error && <div className="error-message">{error}</div>}
      
      <form onSubmit={handleSubmit}>
        <div className="form-group">
          <label htmlFor="name">Stack Name</label>
          <input
            type="text"
            id="name"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
            disabled={isEdit} // Can't change name in edit mode
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="compose-content">Docker Compose File</label>
          <textarea
            id="compose-content"
            value={composeContent}
            onChange={(e) => setComposeContent(e.target.value)}
            rows={20}
            required
            placeholder="version: '3'\nservices:\n  app:\n    image: nginx:latest\n    ports:\n      - '80:80'"
          />
        </div>
        
        <div className="form-group checkbox">
          <input
            type="checkbox"
            id="start-after-create"
            checked={startAfterCreate}
            onChange={(e) => setStartAfterCreate(e.target.checked)}
          />
          <label htmlFor="start-after-create">
            {isEdit ? 'Restart stack after update' : 'Start stack after creation'}
          </label>
        </div>
        
        <div className="form-actions">
          <button type="button" onClick={() => navigate('/compose')}>
            Cancel
          </button>
          <button type="submit" disabled={loading}>
            {loading ? 'Saving...' : isEdit ? 'Update Stack' : 'Create Stack'}
          </button>
        </div>
      </form>
    </div>
  );
};

export default ComposeStackEditor;