import { PublicKey } from '@solana/web3.js';

export interface AuditLogEntry {
  id: string;
  timestamp: number;
  eventType: AuditEventType;
  description: string;
  user: string;
  severity: AuditSeverity;
  metadata?: Record<string, any>;
}

export enum AuditEventType {
  Transaction = 'TRANSACTION',
  Security = 'SECURITY',
  Admin = 'ADMIN',
  System = 'SYSTEM'
}

export enum AuditSeverity {
  Info = 'INFO',
  Warning = 'WARNING',
  Error = 'ERROR',
  Critical = 'CRITICAL'
} 