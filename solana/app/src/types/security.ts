import { PublicKey } from '@solana/web3.js';

export interface SecurityConfig {
  multiSigRequired: boolean;
  requiredSignatures: number;
  maxTransactionAmount: number;
  dailyVolumeLimit: number;
  emergencyMode: boolean;
  lastUpdated: number;
  authority: PublicKey;
  riskLevel: RiskLevel;
  auditConfig: AuditConfig;
}

export enum RiskLevel {
  Low = 'LOW',
  Medium = 'MEDIUM',
  High = 'HIGH',
  Critical = 'CRITICAL'
}

export interface AuditConfig {
  enabled: boolean;
  interval: number;
  lastAudit: number;
  auditor: PublicKey;
}

export interface SecurityMetrics {
  totalTransactions: number;
  failedAttempts: number;
  averageResponseTime: number;
  activeAlerts: Alert[];
}

export interface Alert {
  id: string;
  type: AlertType;
  severity: AlertSeverity;
  timestamp: number;
  description: string;
  resolved: boolean;
}

export enum AlertType {
  UnauthorizedAccess = 'UNAUTHORIZED_ACCESS',
  SuspiciousActivity = 'SUSPICIOUS_ACTIVITY',
  ThresholdExceeded = 'THRESHOLD_EXCEEDED',
  SystemError = 'SYSTEM_ERROR'
}

export enum AlertSeverity {
  Info = 'INFO',
  Warning = 'WARNING',
  Error = 'ERROR',
  Critical = 'CRITICAL'
} 