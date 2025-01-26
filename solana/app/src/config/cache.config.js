module.exports = {
  // Browser cache strategy
  browserCache: {
    // Static resource cache
    static: {
      maxAge: '1y',
      immutable: true,
      patterns: [
        '**/*.{js,css,woff2,png,jpg,jpeg,gif,svg}',
      ],
    },
    // API response cache
    api: {
      maxAge: '5m',
      patterns: [
        '/api/v1/projects',
        '/api/v1/stats',
      ],
    },
    // Dynamic content
    dynamic: {
      maxAge: '0',
      mustRevalidate: true,
      patterns: [
        '/api/v1/user/**',
        '/api/v1/transactions',
      ],
    },
  },

  // Redis cache configuration
  redis: {
    host: process.env.REDIS_HOST || 'localhost',
    port: parseInt(process.env.REDIS_PORT || '6379'),
    password: process.env.REDIS_PASSWORD,
    db: 0,
    keyPrefix: 'crowdfund:',
    // Cache time configuration
    ttl: {
      project: 3600,    // 1 hour
      user: 86400,      // 1 day
      stats: 300,       // 5 minutes
    },
  },
}; 