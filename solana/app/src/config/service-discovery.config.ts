export const serviceDiscoveryConfig = {
  registry: {
    endpoint: process.env.REACT_APP_SERVICE_REGISTRY || 'http://localhost:8500',
    interval: 30000, // 30 seconds
    timeout: 5000
  },
  services: {
    auth: {
      name: 'auth-service',
      version: 'v1',
      healthCheck: '/health'
    },
    blockchain: {
      name: 'blockchain-service',
      version: 'v1',
      healthCheck: '/health'
    },
    storage: {
      name: 'storage-service',
      version: 'v1',
      healthCheck: '/health'
    }
  },
  loadBalancing: {
    strategy: 'round-robin',
    healthCheckInterval: 10000
  }
} as const;

export type ServiceDiscoveryConfig = typeof serviceDiscoveryConfig; 