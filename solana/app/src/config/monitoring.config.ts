import { AlertLevel, MetricType } from '../types/monitoring';

export const monitoringConfig = {
  // Metrics configuration
  metrics: {
    enabled: true,
    interval: 60000, // 1 minute
    endpoints: {
      performance: '/api/metrics/performance',
      errors: '/api/metrics/errors',
      usage: '/api/metrics/usage'
    }
  },

  // Alert configuration
  alerts: {
    enabled: true,
    channels: ['email', 'slack'],
    thresholds: {
      errorRate: 0.05,
      responseTime: 1000,
      cpuUsage: 0.8,
      memoryUsage: 0.8
    }
  },

  // Logging configuration
  logging: {
    level: process.env.NODE_ENV === 'production' ? 'error' : 'debug',
    format: 'json',
    destination: process.env.NODE_ENV === 'production' ? 'sentry' : 'console'
  },

  // Tracing configuration
  tracing: {
    enabled: true,
    sampleRate: 0.1,
    excludePaths: ['/health', '/metrics']
  },

  enabled: true,
  errorThreshold: 5,
  warningThreshold: 3
} as const;

export type MonitoringConfig = typeof monitoringConfig; 