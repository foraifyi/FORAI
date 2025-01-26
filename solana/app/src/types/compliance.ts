import { PublicKey } from '@solana/web3.js';

export interface ComplianceCheck {
  id: string;
  name: string;
  description: string;
  required?: boolean;
  frequency?: number;
}

export interface ComplianceResult {
  checkId: string;
  checkName: string;
  status: ComplianceStatus;
  details: string;
  timestamp: number;
  issues: ComplianceIssue[];
}

export interface ComplianceIssue {
  severity: ComplianceSeverity;
  description: string;
  code: string;
  remediation?: string;
}

export enum ComplianceStatus {
  Passed = 'PASSED',
  Failed = 'FAILED',
  Pending = 'PENDING',
  Skipped = 'SKIPPED'
}

export enum ComplianceSeverity {
  Info = 'INFO',
  Warning = 'WARNING',
  Error = 'ERROR',
  Critical = 'CRITICAL'
} 