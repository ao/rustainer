import React, { useEffect, useState } from 'react';
import { Volume } from '../types';
import { volumeApi } from '../services/api';

const VolumeList: React.FC = () => {
  const [volumes, setVolumes] = useState<Volume[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchVolumes = async () => {
    try {
      setLoading(true);
      const data = await volumeApi.getVolumes();
      setVolumes(data);
      setError(null);
    } catch (err) {
      setError('Failed to fetch volumes');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchVolumes();
  }, []);

  if (loading && volumes.length === 0) {
    return <div className="card">Loading volumes...</div>;
  }

  if (error && volumes.length === 0) {
    return <div className="card">Error: {error}</div>;
  }

  return (
    <div>
      <div className="card-header">
        <h2>Volumes</h2>
        <button className="button" onClick={fetchVolumes}>
          Refresh
        </button>
      </div>

      <div className="card">
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Driver</th>
              <th>Mountpoint</th>
              <th>Created</th>
              <th>Labels</th>
            </tr>
          </thead>
          <tbody>
            {volumes.length === 0 ? (
              <tr>
                <td colSpan={5}>No volumes found</td>
              </tr>
            ) : (
              volumes.map((volume) => (
                <tr key={volume.name}>
                  <td>{volume.name}</td>
                  <td>{volume.driver}</td>
                  <td>{volume.mountpoint}</td>
                  <td>{volume.created_at || 'N/A'}</td>
                  <td>
                    {volume.labels && Object.keys(volume.labels).length > 0
                      ? Object.entries(volume.labels).map(([key, value]) => (
                          <div key={key}>
                            {key}: {value}
                          </div>
                        ))
                      : 'None'}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default VolumeList;