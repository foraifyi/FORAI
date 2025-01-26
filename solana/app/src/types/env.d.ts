declare namespace NodeJS {
  interface ProcessEnv {
    NODE_ENV: 'development' | 'production' | 'test';
    REACT_APP_SOLANA_NETWORK: string;
    REACT_APP_RPC_ENDPOINT: string;
    REACT_APP_PROGRAM_ID: string;
    REACT_APP_SENTRY_DSN: string;
    REACT_APP_ENVIRONMENT: string;
    SONAR_HOST_URL: string;
    SONAR_TOKEN: string;
    TEST_BASE_URL: string;
    [key: string]: string | undefined;
  }
} 