export enum MetricType {
  ResponseTime = 'response_time',
  ErrorRate = 'error_rate',
  CPUUsage = 'cpu_usage',
  MemoryUsage = 'memory_usage',
}

export enum AlertLevel {
  Info = 'INFO',
  Warning = 'WARNING',
  Error = 'ERROR',
  Critical = 'CRITICAL',
}

export interface MetricConfig {
  threshold: number;
  alertLevel: AlertLevel;
}

export interface AlertRule {
  name: string;
  condition: string;
  duration: string;
  level: AlertLevel;
  channels: string[];
}

export interface AlertChannel {
  enabled: boolean;
  [key: string]: any;
} 