export interface SiteConfig {
  region: string;
  endpoints: {
    api: string;
    web: string;
  };
}

export interface DrSiteConfig extends SiteConfig {
  priority: number;
  syncConfig: {
    interval: string;
    maxLag: string;
  };
}

export interface FailoverConditions {
  responseTime: number;
  errorRate: number;
  downtime: string;
}

export interface RecoveryPriority {
  type: string;
  priority: number;
  maxDowntime: string;
}

export interface ValidationConfig {
  requiredChecks: string[];
  timeout: string;
}

export interface DrillConfig {
  schedule: string;
  automaticRollback: boolean;
  notificationChannels: string[];
  scenarios: string[];
}

export interface DisasterRecoveryConfig {
  primarySite: SiteConfig;
  drSites: DrSiteConfig[];
  failover: {
    autoFailoverConditions: FailoverConditions;
    dns: {
      provider: string;
      ttl: number;
      healthCheckInterval: string;
    };
    dataSync: {
      requiredConsistency: number;
      maxAllowedLag: string;
    };
  };
  recovery: {
    priorities: RecoveryPriority[];
    validation: ValidationConfig;
  };
  drillConfig: DrillConfig;
} 