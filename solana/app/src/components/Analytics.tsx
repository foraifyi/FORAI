import * as React from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { Chart } from 'react-chartjs-2';
import { AnalyticsData } from '../types';
import { fetchAnalyticsData } from '../utils/analytics';
import { ProjectMetrics } from './analytics/ProjectMetrics';
import { InvestmentMetrics } from './analytics/InvestmentMetrics';
import { UserMetrics } from './analytics/UserMetrics';
import { PerformanceMetrics } from './analytics/PerformanceMetrics';

interface ChartOptions {
  responsive: boolean;
  maintainAspectRatio: boolean;
  plugins: {
    legend: {
      position: 'bottom' as const;
    };
  };
}

const chartOptions: ChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      position: 'bottom'
    }
  }
};

export const Analytics: React.FC = () => {
  const { connection } = useConnection();
  const [data, setData] = React.useState<AnalyticsData | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  React.useEffect(() => {
    loadAnalyticsData();
  }, [connection]);

  async function loadAnalyticsData() {
    try {
      setLoading(true);
      const analyticsData = await fetchAnalyticsData(connection);
      setData(analyticsData);
    } catch (err) {
      console.error('Failed to load analytics:', err);
      setError('Failed to load analytics data. Please try again.');
    } finally {
      setLoading(false);
    }
  }

  if (loading) return <div>Loading analytics...</div>;
  if (error) return <div className="error-message">{error}</div>;
  if (!data) return <div>No analytics data available</div>;

  return (
    <div className="analytics-dashboard">
      <h1>Platform Analytics</h1>

      <div className="metrics-grid">
        <ProjectMetrics data={data.projects} />
        <InvestmentMetrics data={data.investments} />
        <UserMetrics data={data.users} />
        <PerformanceMetrics data={data.performance} />
      </div>

      <div className="charts-section">
        <div className="chart-container">
          <h3>Investment Distribution</h3>
          <Chart 
            type="pie"
            data={data.investmentDistribution}
            options={chartOptions}
          />
        </div>

        <div className="chart-container">
          <h3>Project Success Rate</h3>
          <Chart 
            type="bar"
            data={data.successRate}
            options={chartOptions}
          />
        </div>

        <div className="chart-container">
          <h3>Monthly Activity</h3>
          <Chart 
            type="line"
            data={data.monthlyActivity}
            options={chartOptions}
          />
        </div>
      </div>
    </div>
  );
}; 