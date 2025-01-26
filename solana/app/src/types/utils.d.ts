import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { SecurityConfig, SecurityMetrics } from './security';
import { RiskMetrics } from './risk';
import { ComplianceResult } from './compliance';

// Management tool function types
export interface AdminUtils {
  fetchSecurityConfig(connection: Connection): Promise<SecurityConfig>;
  updateSecuritySettings(connection: Connection, settings: Partial<SecurityConfig>): Promise<boolean>;
  fetchSecurityMetrics(connection: Connection): Promise<SecurityMetrics>;
  fetchRiskMetrics(connection: Connection): Promise<RiskMetrics>;
  runComplianceCheck(connection: Connection, authority: PublicKey, checks: string[]): Promise<ComplianceResult[]>;
}

// Monitoring tool function types
export interface MonitoringUtils {
  initializeMonitoring(): void;
  trackPerformance(metricName: string, value: number): void;
  trackError(error: Error, context?: Record<string, any>): void;
}

// Transaction tool function types
export interface TransactionUtils {
  createInvestmentTransaction(
    connection: Connection,
    project: PublicKey,
    amount: number
  ): Promise<Transaction>;
  
  createMilestoneTransaction(
    connection: Connection,
    milestone: PublicKey,
    action: 'complete' | 'reject'
  ): Promise<Transaction>;
}

// Validation tool function types
export interface ValidationUtils {
  validateProjectData(data: any): boolean;
  validateInvestmentAmount(amount: number, project: any): boolean;
  validateMilestoneCompletion(milestone: any): boolean;
} 