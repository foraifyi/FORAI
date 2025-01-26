// Handle Sentry module
declare module '@sentry/react' {
  export * from '@sentry/browser';
  export const init: any;
  export const captureException: any;
  export const captureMessage: any;
}

declare module '@sentry/tracing' {
  export class BrowserTracing {
    constructor(options?: any);
  }
}

// Handle Chart.js module
declare module 'chart.js/auto' {
  export * from 'chart.js';
}

// Handle React Chart.js 2
declare module 'react-chartjs-2' {
  export * from '@types/react-chartjs-2';
}

// Handle Jest environment
declare module 'jest-environment-jsdom' {
  export default class JSDOMEnvironment {
    constructor(config: any);
  }
}

// Handle other modules
declare module '*.css' {
  const content: { [className: string]: string };
  export default content;
}

declare module '*.scss' {
  const content: { [className: string]: string };
  export default content;
}

declare module '*.json' {
  const value: any;
  export default value;
} 