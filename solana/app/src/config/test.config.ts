export const testConfig = {
  environment: 'test',
  baseUrl: process.env.TEST_BASE_URL || 'http://localhost:3000',
  api: {
    timeout: 5000,
    retries: 3
  },
  mocks: {
    enabled: true,
    delay: 100
  },
  coverage: {
    statements: 80,
    branches: 70,
    functions: 80,
    lines: 80
  }
} as const;

export type TestConfig = typeof testConfig;

// Unit test configuration
export const unitTestConfig = {
  coverage: {
    statements: 80,
    branches: 70,
    functions: 80,
    lines: 80,
  },
  directories: [
    'src/components',
    'src/utils',
    'src/hooks',
  ],
  exclude: [
    '**/*.d.ts',
    '**/index.ts',
    '**/types/**',
  ],
};

// Integration test configuration
export const integrationTestConfig = {
  testEnvironment: 'jsdom',
  setupFiles: [
    '<rootDir>/src/setupTests.ts',
  ],
  globalSetup: '<rootDir>/test/setup/global.ts',
  globalTeardown: '<rootDir>/test/teardown/global.ts',
};

// E2E test configuration
export const e2eTestConfig = {
  browser: 'chromium',
  viewport: { width: 1280, height: 720 },
  video: 'on-failure',
  screenshot: 'only-on-failure',
  trace: 'retain-on-failure',
};

// Performance test configuration
export const performanceTestConfig = {
  thresholds: {
    firstPaint: 1000,
    firstContentfulPaint: 1500,
    timeToInteractive: 3000,
    totalBlockingTime: 200,
  },
  lighthouse: {
    performance: 90,
    accessibility: 90,
    bestPractices: 90,
    seo: 90,
  },
}; 