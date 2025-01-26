export const disasterRecoveryConfig = {
  // Primary site configuration
  primarySite: {
    region: process.env.PRIMARY_REGION || 'us-west-1',
    endpoints: {
      api: process.env.PRIMARY_API_ENDPOINT,
      web: process.env.PRIMARY_WEB_ENDPOINT,
    },
  },

  // Disaster recovery site configuration
  drSites: [
    {
      region: 'us-east-1',
      priority: 1,
      endpoints: {
        api: process.env.DR_API_ENDPOINT_1,
        web: process.env.DR_WEB_ENDPOINT_1,
      },
      syncConfig: {
        interval: '5m',
        maxLag: '1m',
      },
    },
    {
      region: 'eu-west-1',
      priority: 2,
      endpoints: {
        api: process.env.DR_API_ENDPOINT_2,
        web: process.env.DR_WEB_ENDPOINT_2,
      },
      syncConfig: {
        interval: '10m',
        maxLag: '2m',
      },
    },
  ],

  // Failover configuration
  failover: {
    enabled: true,
    mode: 'automatic',
    threshold: {
      errorRate: 0.1,
      responseTime: 2000
    },
    regions: [
      'us-east-1',
      'us-west-2',
      'eu-west-1'
    ]
  },

  // Recovery process
  recovery: {
    automatic: true,
    maxAttempts: 3,
    backoffInterval: 5000
  },

  // Drill configuration
  drillConfig: {
    schedule: '0 0 1 */3 *',  // Every 3 months
    automaticRollback: true,
    notificationChannels: ['email', 'slack'],
    scenarios: [
      'primary_db_failure',
      'network_partition',
      'region_outage',
    ],
  },

  backup: {
    enabled: true,
    frequency: 'daily',
    retention: 30,
    type: 'full'
  },

  monitoring: {
    interval: 60000,
    timeout: 5000,
    healthEndpoint: '/health'
  }
};

export type DisasterRecoveryConfig = typeof disasterRecoveryConfig; 