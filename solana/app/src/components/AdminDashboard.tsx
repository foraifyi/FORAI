import React, { useState } from 'react';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { ProjectManagement } from './admin/ProjectManagement';
import { UserManagement } from './admin/UserManagement';
import { SecuritySettings } from './admin/SecuritySettings';
import { Analytics } from './admin/Analytics';
import { SecurityMonitor } from './admin/SecurityMonitor';

export function AdminDashboard() {
  const [activeTab, setActiveTab] = useState('projects');
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [loading, setLoading] = useState(false);

  const tabs = {
    projects: <ProjectManagement />,
    users: <UserManagement />,
    security: <SecuritySettings />,
    analytics: <Analytics />,
    monitor: <SecurityMonitor />
  };

  const handleTabChange = (tab: string) => {
    setActiveTab(tab);
  };

  const loadDashboardData = async () => {
    setLoading(true);
    try {
      // Load admin data
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="admin-dashboard">
      <nav className="admin-nav">
        <button 
          onClick={() => handleTabChange('projects')}
          className={activeTab === 'projects' ? 'active' : ''}
        >
          Projects
        </button>
        <button 
          onClick={() => handleTabChange('users')}
          className={activeTab === 'users' ? 'active' : ''}
        >
          Users
        </button>
        <button 
          onClick={() => handleTabChange('security')}
          className={activeTab === 'security' ? 'active' : ''}
        >
          Security
        </button>
        <button 
          onClick={() => handleTabChange('analytics')}
          className={activeTab === 'analytics' ? 'active' : ''}
        >
          Analytics
        </button>
        <button 
          onClick={() => handleTabChange('monitor')}
          className={activeTab === 'monitor' ? 'active' : ''}
        >
          Monitor
        </button>
      </nav>

      <main className="admin-content">
        {tabs[activeTab]}
      </main>
    </div>
  );
} 