declare module '@sentry/react' {
  export function init(options: any): void;
  export function captureException(error: any, context?: any): void;
  export function captureMessage(message: string, level?: string): void;
  export const Severity: {
    Fatal: string;
    Error: string;
    Warning: string;
    Info: string;
    Debug: string;
  };
}

declare module '@sentry/tracing' {
  export class BrowserTracing {
    constructor(options?: any);
  }
} 