import * as Sentry from '@sentry/react';
import { BrowserTracing } from '@sentry/tracing';
import { monitoringConfig } from '../config/monitoring.config';
import { captureException, captureMessage } from '@sentry/react';
import { MonitoringMetrics, ErrorEvent } from '../types';

export function initializeMonitoring() {
  if (process.env.NODE_ENV === 'production') {
    Sentry.init({
      dsn: process.env.REACT_APP_SENTRY_DSN,
      integrations: [new BrowserTracing()],
      tracesSampleRate: 0.2,
      environment: process.env.REACT_APP_ENVIRONMENT,
    });
  }
}

export function trackPerformance(metricName: string, value: number) {
  if (window.performance && window.performance.mark) {
    window.performance.mark(metricName);
  }

  // Send to monitoring system
  if (process.env.NODE_ENV === 'production') {
    Sentry.captureMessage('Performance Metric', {
      level: 'info',
      extra: {
        metricName,
        value,
        timestamp: Date.now(),
      },
    });
  }
}

class MonitoringService {
  private metrics: MonitoringMetrics = {
    errors: 0,
    warnings: 0,
    performance: {
      avgResponseTime: 0,
      requestCount: 0
    }
  };

  initialize(): void {
    if (!monitoringConfig.metrics.enabled) return;

    // Setup periodic metrics collection
    setInterval(() => {
      this.collectMetrics();
    }, monitoringConfig.metrics.interval);
  }

  trackError(error: Error, context?: Record<string, any>): void {
    this.metrics.errors++;
    
    const errorEvent: ErrorEvent = {
      message: error.message,
      stack: error.stack,
      context,
      timestamp: Date.now()
    };

    if (monitoringConfig.logging.destination === 'sentry') {
      captureException(error, { extra: context });
    } else {
      console.error('Error:', errorEvent);
    }
  }

  trackWarning(message: string, context?: Record<string, any>): void {
    this.metrics.warnings++;

    if (monitoringConfig.logging.destination === 'sentry') {
      captureMessage(message, { extra: context, level: 'warning' });
    } else {
      console.warn('Warning:', message, context);
    }
  }

  private async collectMetrics(): Promise<void> {
    try {
      const metrics = { ...this.metrics };
      
      // Reset counters after collection
      this.metrics.errors = 0;
      this.metrics.warnings = 0;
      this.metrics.performance.requestCount = 0;
      this.metrics.performance.avgResponseTime = 0;

      // Send metrics to configured endpoint
      await fetch(monitoringConfig.metrics.endpoints.performance, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(metrics)
      });
    } catch (error) {
      console.error('Failed to collect metrics:', error);
    }
  }
}

export const monitoring = new MonitoringService(); 