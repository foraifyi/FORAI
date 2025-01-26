declare module '@sentry/react';
declare module '@sentry/tracing';

declare global {
  interface Window {
    solana: any;
  }
}

export {}; 