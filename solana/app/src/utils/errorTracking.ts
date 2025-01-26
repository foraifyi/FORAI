import * as Sentry from '@sentry/react';
import { BrowserTracing } from '@sentry/tracing';

export function initializeErrorTracking(): void {
  if (process.env.NODE_ENV === 'production') {
    Sentry.init({
      dsn: process.env.REACT_APP_SENTRY_DSN,
      integrations: [new BrowserTracing()],
      tracesSampleRate: 1.0,
      environment: process.env.REACT_APP_ENVIRONMENT
    });
  }
}

export function trackError(error: Error, context?: Record<string, any>): void {
  if (process.env.NODE_ENV === 'production') {
    Sentry.captureException(error, { extra: context });
  } else {
    console.error('Error:', error, 'Context:', context);
  }
} 