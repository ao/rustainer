import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Link, Navigate, useLocation } from 'react-router-dom';
import ContainerList from './components/ContainerList';
import ContainerDetail from './components/ContainerDetail';
import VolumeList from './components/VolumeList';
import NetworkList from './components/NetworkList';
import NetworkDetail from './components/NetworkDetail';
import CreateContainer from './components/CreateContainer';
import Login from './components/Login';
import ComposeStackList from './components/ComposeStackList';
import ComposeStackDetail from './components/ComposeStackDetail';
import ComposeStackEditor from './components/ComposeStackEditor';
import TemplateList from './components/TemplateList';
import TemplateDetail from './components/TemplateDetail';
import TemplateEditor from './components/TemplateEditor';
import UserManagement from './components/UserManagement';
import ServiceList from './components/ServiceList';
import ServiceDetail from './components/ServiceDetail';
import CreateService from './components/CreateService';
import api from './services/api';
import { User } from './types';
import { useTheme } from './context/ThemeContext';

// NavLink component that adds active class when the current path matches
const NavLink: React.FC<{ to: string; children: React.ReactNode }> = ({ to, children }) => {
  const location = useLocation();
  const isActive = location.pathname === to || 
                  (to !== '/' && location.pathname.startsWith(to));
  
  return (
    <Link to={to} className={isActive ? 'active' : ''}>
      {children}
    </Link>
  );
};

// Protected route component
interface ProtectedRouteProps {
  element: React.ReactNode;
  requiredRole?: 'admin' | 'operator' | 'viewer';
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ element, requiredRole }) => {
  const isAuthenticated = api.auth.isAuthenticated();
  const location = useLocation();
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  
  useEffect(() => {
    const fetchUser = async () => {
      if (isAuthenticated) {
        try {
          const userData = await api.auth.getCurrentUser();
          setUser(userData);
        } catch (error) {
          console.error('Failed to fetch user data:', error);
          // If we can't get the user data, log them out
          api.auth.logout();
          window.location.href = '/login';
        } finally {
          setLoading(false);
        }
      } else {
        setLoading(false);
      }
    };
    
    fetchUser();
  }, [isAuthenticated]);
  
  if (loading) {
    return <div className="loading">Loading...</div>;
  }
  
  if (!isAuthenticated) {
    // Redirect to login page with return URL
    return <Navigate to={`/login?returnUrl=${encodeURIComponent(location.pathname)}`} />;
  }
  
  // Check role-based access if a role is required
  if (requiredRole && user) {
    const roleHierarchy = { admin: 3, operator: 2, viewer: 1 };
    const userRoleLevel = roleHierarchy[user.role] || 0;
    const requiredRoleLevel = roleHierarchy[requiredRole] || 0;
    
    if (userRoleLevel < requiredRoleLevel) {
      return <div className="error-message">You don't have permission to access this page.</div>;
    }
  }
  
  return <>{element}</>;
};

const App: React.FC = () => {
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(
    api.auth.isAuthenticated()
  );
  const [user, setUser] = useState<User | null>(null);

  useEffect(() => {
    // Check if user is authenticated and fetch user data
    if (isAuthenticated) {
      api.auth.getCurrentUser()
        .then(userData => setUser(userData))
        .catch(() => {
          // If there's an error fetching user data, log out
          handleLogout();
        });
    }
  }, [isAuthenticated]);

  const handleLoginSuccess = () => {
    setIsAuthenticated(true);
  };

  const handleLogout = () => {
    api.auth.logout();
    setIsAuthenticated(false);
    setUser(null);
  };

  return (
    <Router>
      <div>
        <header className="header">
          <h1><Link to="/">🚢 Rustainer</Link></h1>
          <ThemeToggle />
          {isAuthenticated ? (
            <>
              <nav className="nav">
                <NavLink to="/">Containers</NavLink>
                <NavLink to="/volumes">Volumes</NavLink>
                <NavLink to="/networks">Networks</NavLink>
                <NavLink to="/compose">Compose</NavLink>
                <NavLink to="/templates">Templates</NavLink>
                <NavLink to="/services">Services</NavLink>
                {user && user.role === 'admin' && (
                  <NavLink to="/users">Users</NavLink>
                )}
                <NavLink to="/create">Create Container</NavLink>
                <button onClick={handleLogout} className="logout-button">Logout</button>
              </nav>
              {user && <div className="user-info">Logged in as: {user.username} ({user.role})</div>}
            </>
          ) : (
            <nav className="nav">
              <Link to="/login">Login</Link>
            </nav>
          )}
        </header>

        <div className="container">
          <Routes>
            <Route path="/login" element={<Login onLoginSuccess={handleLoginSuccess} />} />
            <Route path="/" element={<ProtectedRoute element={<ContainerList />} />} />
            <Route path="/containers/:id" element={<ProtectedRoute element={<ContainerDetail />} />} />
            <Route path="/volumes" element={<ProtectedRoute element={<VolumeList />} />} />
            <Route path="/networks" element={<ProtectedRoute element={<NetworkList />} />} />
            <Route path="/networks/:id" element={<ProtectedRoute element={<NetworkDetail />} />} />
            <Route path="/create" element={<ProtectedRoute element={<CreateContainer />} />} />
            {/* Docker Compose Routes */}
            <Route path="/compose/create" element={<ProtectedRoute element={<ComposeStackEditor />} />} />
            <Route path="/compose/:id/edit" element={<ProtectedRoute element={<ComposeStackEditor isEdit={true} />} />} />
            <Route path="/compose/:id" element={<ProtectedRoute element={<ComposeStackDetail />} />} />
            <Route path="/compose" element={<ProtectedRoute element={<ComposeStackList />} />} />
            {/* Template Routes */}
            <Route path="/templates/create" element={<ProtectedRoute element={<TemplateEditor />} />} />
            <Route path="/templates/:id/edit" element={<ProtectedRoute element={<TemplateEditor isEdit={true} />} />} />
            <Route path="/templates/:id/deploy" element={<ProtectedRoute element={<TemplateDetail />} />} />
            <Route path="/templates/:id" element={<ProtectedRoute element={<TemplateDetail />} />} />
            <Route path="/templates" element={<ProtectedRoute element={<TemplateList />} />} />
            {/* User Management Route */}
            <Route path="/users" element={<ProtectedRoute element={<UserManagement />} requiredRole="admin" />} />
            {/* Service Routes */}
            <Route path="/services" element={<ProtectedRoute element={<ServiceList />} />} />
            <Route path="/services/create" element={<ProtectedRoute element={<CreateService />} />} />
            <Route path="/services/:id" element={<ProtectedRoute element={<ServiceDetail />} />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

// Theme toggle component
const ThemeToggle: React.FC = () => {
  const { theme, toggleTheme } = useTheme();
  
  return (
    <button 
      onClick={toggleTheme} 
      className="theme-toggle"
      title={`Switch to ${theme === 'dark' ? 'light' : 'dark'} mode`}
    >
      {theme === 'dark' ? '☀️' : '🌙'}
    </button>
  );
};

export default App;