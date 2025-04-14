import React, { useEffect, useState } from 'react';
import { authApi } from '../services/api';
import { User, CreateUserRequest } from '../types';

const UserManagement: React.FC = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateModal, setShowCreateModal] = useState<boolean>(false);
  const [showEditModal, setShowEditModal] = useState<boolean>(false);
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  
  // Form state
  const [formData, setFormData] = useState<CreateUserRequest>({
    username: '',
    password: '',
    role: 'viewer',
    email: undefined,
  });

  const fetchUsers = async () => {
    try {
      setLoading(true);
      const data = await authApi.getUsers();
      setUsers(data);
      setError(null);
    } catch (err) {
      setError('Failed to fetch users');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleCreateUser = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await authApi.createUser(formData);
      setShowCreateModal(false);
      setFormData({
        username: '',
        password: '',
        role: 'viewer',
        email: undefined,
      });
      fetchUsers();
    } catch (err) {
      setError('Failed to create user');
      console.error(err);
    }
  };

  const handleEditUser = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!currentUser) return;
    
    try {
      await authApi.updateUser(currentUser.id, formData);
      setShowEditModal(false);
      setCurrentUser(null);
      setFormData({
        username: '',
        password: '',
        role: 'viewer',
        email: undefined,
      });
      fetchUsers();
    } catch (err) {
      setError('Failed to update user');
      console.error(err);
    }
  };

  const handleDeleteUser = async (userId: string) => {
    if (!window.confirm('Are you sure you want to delete this user?')) {
      return;
    }
    
    try {
      await authApi.deleteUser(userId);
      fetchUsers();
    } catch (err) {
      setError('Failed to delete user');
      console.error(err);
    }
  };

  const openEditModal = (user: User) => {
    setCurrentUser(user);
    setFormData({
      username: user.username,
      password: '', // Don't include the password when editing
      role: user.role,
      email: user.email,
    });
    setShowEditModal(true);
  };

  if (loading && users.length === 0) {
    return <div className="card">Loading users...</div>;
  }

  if (error && users.length === 0) {
    return <div className="error-message">{error}</div>;
  }

  return (
    <div>
      <div className="header-actions">
        <h2>User Management</h2>
        <button className="button" onClick={() => setShowCreateModal(true)}>
          Create User
        </button>
      </div>

      {error && <div className="error-message">{error}</div>}

      <div className="card">
        <table>
          <thead>
            <tr>
              <th>Username</th>
              <th>Role</th>
              <th>Email</th>
              <th>Created</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {users.length === 0 ? (
              <tr>
                <td colSpan={5}>No users found</td>
              </tr>
            ) : (
              users.map((user) => (
                <tr key={user.id}>
                  <td>{user.username}</td>
                  <td>
                    <span className={`badge role-${user.role}`}>
                      {user.role}
                    </span>
                  </td>
                  <td>{user.email || '-'}</td>
                  <td>{new Date(user.created_at).toLocaleString()}</td>
                  <td>
                    <div className="actions">
                      <button
                        className="button"
                        onClick={() => openEditModal(user)}
                      >
                        Edit
                      </button>
                      <button
                        className="button danger"
                        onClick={() => handleDeleteUser(user.id)}
                      >
                        Delete
                      </button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Create User Modal */}
      {showCreateModal && (
        <div className="modal">
          <div className="modal-content">
            <div className="modal-header">
              <h3>Create User</h3>
              <button onClick={() => setShowCreateModal(false)} className="close-button">&times;</button>
            </div>
            <div className="modal-body">
              <form onSubmit={handleCreateUser}>
                <div className="form-group">
                  <label htmlFor="username">Username</label>
                  <input
                    type="text"
                    id="username"
                    name="username"
                    value={formData.username}
                    onChange={handleInputChange}
                    required
                  />
                </div>
                <div className="form-group">
                  <label htmlFor="password">Password</label>
                  <input
                    type="password"
                    id="password"
                    name="password"
                    value={formData.password}
                    onChange={handleInputChange}
                    required
                  />
                </div>
                <div className="form-group">
                  <label htmlFor="role">Role</label>
                  <select
                    id="role"
                    name="role"
                    value={formData.role}
                    onChange={handleInputChange}
                    required
                  >
                    <option value="admin">Admin</option>
                    <option value="operator">Operator</option>
                    <option value="viewer">Viewer</option>
                  </select>
                </div>
                <div className="form-group">
                  <label htmlFor="email">Email (optional)</label>
                  <input
                    type="email"
                    id="email"
                    name="email"
                    value={formData.email || ''}
                    onChange={handleInputChange}
                  />
                </div>
                <div className="form-actions">
                  <button type="submit" className="button">
                    Create User
                  </button>
                  <button
                    type="button"
                    className="button secondary"
                    onClick={() => setShowCreateModal(false)}
                  >
                    Cancel
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}

      {/* Edit User Modal */}
      {showEditModal && currentUser && (
        <div className="modal">
          <div className="modal-content">
            <div className="modal-header">
              <h3>Edit User: {currentUser.username}</h3>
              <button onClick={() => setShowEditModal(false)} className="close-button">&times;</button>
            </div>
            <div className="modal-body">
              <form onSubmit={handleEditUser}>
                <div className="form-group">
                  <label htmlFor="edit-username">Username</label>
                  <input
                    type="text"
                    id="edit-username"
                    name="username"
                    value={formData.username}
                    onChange={handleInputChange}
                    required
                  />
                </div>
                <div className="form-group">
                  <label htmlFor="edit-password">Password (leave blank to keep current)</label>
                  <input
                    type="password"
                    id="edit-password"
                    name="password"
                    value={formData.password}
                    onChange={handleInputChange}
                  />
                </div>
                <div className="form-group">
                  <label htmlFor="edit-role">Role</label>
                  <select
                    id="edit-role"
                    name="role"
                    value={formData.role}
                    onChange={handleInputChange}
                    required
                  >
                    <option value="admin">Admin</option>
                    <option value="operator">Operator</option>
                    <option value="viewer">Viewer</option>
                  </select>
                </div>
                <div className="form-group">
                  <label htmlFor="edit-email">Email (optional)</label>
                  <input
                    type="email"
                    id="edit-email"
                    name="email"
                    value={formData.email || ''}
                    onChange={handleInputChange}
                  />
                </div>
                <div className="form-actions">
                  <button type="submit" className="button">
                    Update User
                  </button>
                  <button
                    type="button"
                    className="button secondary"
                    onClick={() => setShowEditModal(false)}
                  >
                    Cancel
                  </button>
                </div>
              </form>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default UserManagement;