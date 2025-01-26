export const securityTestConfig = {
  scanners: {
    static: {
      enabled: true,
      excludePaths: ['node_modules', 'build', 'coverage']
    },
    dynamic: {
      enabled: true,
      endpoints: ['/api/*']
    },
    dependency: {
      enabled: true,
      failOnHigh: true
    }
  },
  rules: {
    auth: {
      requireAuth: ['/api/admin/*', '/api/user/*'],
      publicPaths: ['/api/public/*', '/health']
    },
    rateLimit: {
      enabled: true,
      maxRequests: 100,
      timeWindow: 60000
    }
  }
} as const;

export type SecurityTestConfig = typeof securityTestConfig; 