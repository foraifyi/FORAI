import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { SecurityMetrics, SecurityAlert } from '../../types';
import { fetchSecurityAlerts, resolveAlert } from '../../utils/admin';
import { AlertList } from './AlertList';
import { MetricsDisplay } from './MetricsDisplay';

interface SecurityMonitorProps {
  metrics: SecurityMetrics | null;
  onAlertResolution?: (alertId: string) => Promise<void>;
}

export const SecurityMonitor: React.FC<SecurityMonitorProps> = ({
  metrics,
  onAlertResolution
}) => {
  const { connection } = useConnection();
  const [alerts, setAlerts] = React.useState<SecurityAlert[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    loadAlerts();
  }, [connection]);

  async function loadAlerts() {
    try {
      setLoading(true);
      const securityAlerts = await fetchSecurityAlerts(connection);
      setAlerts(securityAlerts);
    } catch (err) {
      console.error('Failed to load security alerts:', err);
      setError('Failed to load security alerts. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  async function handleAlertResolve(alertId: string) {
    try {
      setError(null);
      await resolveAlert(connection, alertId);
      
      if (onAlertResolution) {
        await onAlertResolution(alertId);
      }
      
      await loadAlerts();
    } catch (err) {
      console.error('Failed to resolve alert:', err);
      setError('Failed to resolve alert. Please try again.');
    }
  }

  return (
    <div className="security-monitor">
      <h2>Security Monitor</h2>

      {metrics && <MetricsDisplay metrics={metrics} />}

      {loading ? (
        <div>Loading security alerts...</div>
      ) : (
        <AlertList
          alerts={alerts}
          onResolve={handleAlertResolve}
        />
      )}

      {error && <div className="error-message">{error}</div>}
    </div>
  );
}; 