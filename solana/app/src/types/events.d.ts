import { PublicKey } from '@solana/web3.js';
import { Project, Investment, Milestone } from './project';

// Project events
export interface ProjectEvent {
  type: 'PROJECT_CREATED' | 'PROJECT_UPDATED' | 'PROJECT_DELETED';
  projectId: string;
  data: Partial<Project>;
  timestamp: number;
}

// Investment events
export interface InvestmentEvent {
  type: 'INVESTMENT_MADE' | 'INVESTMENT_WITHDRAWN';
  investmentId: string;
  projectId: string;
  investor: PublicKey;
  amount: number;
  timestamp: number;
}

// Milestone events
export interface MilestoneEvent {
  type: 'MILESTONE_COMPLETED' | 'MILESTONE_FAILED';
  milestoneId: string;
  projectId: string;
  data: Partial<Milestone>;
  timestamp: number;
}

// System events
export interface SystemEvent {
  type: 'SECURITY_ALERT' | 'PERFORMANCE_ALERT' | 'COMPLIANCE_ALERT';
  severity: 'INFO' | 'WARNING' | 'ERROR' | 'CRITICAL';
  message: string;
  data?: Record<string, any>;
  timestamp: number;
}

// User events
export interface UserEvent {
  type: 'USER_LOGIN' | 'USER_LOGOUT' | 'USER_ACTION';
  userId: string;
  action?: string;
  data?: Record<string, any>;
  timestamp: number;
} 