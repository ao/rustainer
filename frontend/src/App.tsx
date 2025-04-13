import React, { useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import ContainerList from './components/ContainerList';
import ContainerDetail from './components/ContainerDetail';
import VolumeList from './components/VolumeList';
import NetworkList from './components/NetworkList';
import CreateContainer from './components/CreateContainer';

const App: React.FC = () => {
  return (
    <Router>
      <div>
        <header className="header">
          <h1>🚢 Rustainer</h1>
          <nav className="nav">
            <Link to="/">Containers</Link>
            <Link to="/volumes">Volumes</Link>
            <Link to="/networks">Networks</Link>
            <Link to="/create">Create Container</Link>
          </nav>
        </header>

        <div className="container">
          <Routes>
            <Route path="/" element={<ContainerList />} />
            <Route path="/containers/:id" element={<ContainerDetail />} />
            <Route path="/volumes" element={<VolumeList />} />
            <Route path="/networks" element={<NetworkList />} />
            <Route path="/create" element={<CreateContainer />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;