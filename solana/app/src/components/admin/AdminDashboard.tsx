import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { SecurityConfig, SecurityMetrics, User, AdminMetrics } from '../../types';
import { fetchSecurityConfig, fetchSecurityMetrics, fetchAdminMetrics } from '../../utils/admin';
import { SecurityMonitor } from './SecurityMonitor';
import { SecuritySettings } from './SecuritySettings';
import { UserManagement } from './UserManagement';
import { AuditLog } from './AuditLog';

interface AdminDashboardProps {
  user: User;
  onMetricsUpdate?: (metrics: AdminMetrics) => void;
}

export const AdminDashboard: React.FC<AdminDashboardProps> = ({ user, onMetricsUpdate }) => {
  const { connection } = useConnection();
  const [securityConfig, setSecurityConfig] = React.useState<SecurityConfig | null>(null);
  const [metrics, setMetrics] = React.useState<SecurityMetrics | null>(null);
  const [adminMetrics, setAdminMetrics] = React.useState<AdminMetrics | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    loadDashboardData();
  }, [connection]);

  async function loadDashboardData() {
    try {
      const [config, securityMetrics] = await Promise.all([
        fetchSecurityConfig(connection),
        fetchSecurityMetrics(connection)
      ]);
      setSecurityConfig(config);
      setMetrics(securityMetrics);
      const adminMetrics = await fetchAdminMetrics(connection);
      setAdminMetrics(adminMetrics);
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      setLoading(false);
    }
  }

  if (loading) return <div>Loading dashboard...</div>;

  return (
    <div className="admin-dashboard">
      <h1>Admin Dashboard</h1>
      
      <div className="dashboard-grid">
        <SecurityMonitor metrics={metrics} />
        <SecuritySettings config={securityConfig} />
        <UserManagement />
        <AuditLog />
      </div>
    </div>
  );
}; 