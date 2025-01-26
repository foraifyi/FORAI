export const performanceConfig = {
  // Performance metric thresholds

  // Page load performance
  metrics: {
    // 页面加载性能
    loading: {
      firstPaint: 1000,            // 1 second
      firstContentfulPaint: 1500,  // 1.5 seconds
      largestContentfulPaint: 2500,// 2.5 seconds
      timeToInteractive: 3000,     // 3 seconds
      totalBlockingTime: 200,      // 200 milliseconds
    },
    
    // API response time
    api: {
      p50: 100,  // 50% of requests complete within 100ms
      p90: 300,  // 90% of requests complete within 300ms
      p99: 1000, // 99% of requests complete within 1 second
    },

    // Resource loading
    resources: {
      maxJSSize: 500000,    // 500KB
      maxCSSSize: 100000,   // 100KB
      maxImageSize: 1000000,// 1MB
      maxFontSize: 100000,  // 100KB
    },
  },

  // Load test configuration
  loadTest: {
    scenarios: [
      {
        name: 'normal_load',
        duration: '5m',
        arrivalRate: 10,
      },
      {
        name: 'peak_load',
        duration: '2m',
        arrivalRate: 50,
      },
      {
        name: 'stress_test',
        duration: '1m',
        arrivalRate: 100,
      },
    ],
    thresholds: {
      http_req_duration: ['p95<500'],
      http_req_failed: ['rate<0.01'],
    },
  },

  // Monitoring configuration
  monitoring: {
    collectMetrics: true,
    sampleRate: 0.1,
    reportingInterval: 60000,
    retentionPeriod: '30d',
  },
}; 