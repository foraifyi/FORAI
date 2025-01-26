import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { RiskMetrics, RiskAlert } from '../../types';
import { fetchRiskMetrics, fetchRiskAlerts } from '../../utils/admin';
import { RiskMetricsDisplay } from './RiskMetricsDisplay';
import { RiskAlertList } from './RiskAlertList';
import { RiskRecommendations } from './RiskRecommendations';

interface RiskDashboardProps {
  onRecommendationAction?: (recommendationId: string) => Promise<void>;
}

export const RiskDashboard: React.FC<RiskDashboardProps> = ({
  onRecommendationAction
}) => {
  const { connection } = useConnection();
  const [metrics, setMetrics] = React.useState<RiskMetrics | null>(null);
  const [alerts, setAlerts] = React.useState<RiskAlert[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    loadRiskData();
  }, [connection]);

  async function loadRiskData() {
    try {
      setLoading(true);
      const [riskMetrics, riskAlerts] = await Promise.all([
        fetchRiskMetrics(connection),
        fetchRiskAlerts(connection)
      ]);
      setMetrics(riskMetrics);
      setAlerts(riskAlerts);
    } catch (err) {
      console.error('Failed to load risk data:', err);
      setError('Failed to load risk data. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  async function handleRecommendationAction(recommendationId: string) {
    try {
      if (onRecommendationAction) {
        await onRecommendationAction(recommendationId);
      }
      await loadRiskData();
    } catch (err) {
      console.error('Failed to process recommendation:', err);
      setError('Failed to process recommendation. Please try again.');
    }
  }

  if (loading) return <div>Loading risk dashboard...</div>;

  return (
    <div className="risk-dashboard">
      <h2>Risk Management Dashboard</h2>

      {metrics && <RiskMetricsDisplay metrics={metrics} />}

      <div className="risk-content">
        <div className="alerts-section">
          <h3>Active Risk Alerts</h3>
          <RiskAlertList alerts={alerts} />
        </div>

        <div className="recommendations-section">
          <h3>Risk Recommendations</h3>
          <RiskRecommendations
            metrics={metrics}
            onAction={handleRecommendationAction}
          />
        </div>
      </div>

      {error && <div className="error-message">{error}</div>}
    </div>
  );
}; 