import { Connection, PublicKey } from '@solana/web3.js';
import { Project, Investment, Milestone } from './project';
import { SecurityConfig, SecurityMetrics } from './security';
import { RiskMetrics } from './risk';

// Project related hooks
export interface UseProject {
  project: Project | null;
  loading: boolean;
  error: Error | null;
  refreshProject: () => Promise<void>;
  updateProject: (data: Partial<Project>) => Promise<void>;
}

export interface UseInvestment {
  investments: Investment[];
  loading: boolean;
  error: Error | null;
  invest: (amount: number) => Promise<void>;
  withdraw: (investmentId: string) => Promise<void>;
}

export interface UseMilestone {
  milestones: Milestone[];
  loading: boolean;
  error: Error | null;
  completeMilestone: (milestoneId: string) => Promise<void>;
  updateMilestone: (milestoneId: string, data: Partial<Milestone>) => Promise<void>;
}

// Management related hooks
export interface UseAdmin {
  securityConfig: SecurityConfig | null;
  securityMetrics: SecurityMetrics | null;
  riskMetrics: RiskMetrics | null;
  loading: boolean;
  error: Error | null;
  updateSecurityConfig: (config: Partial<SecurityConfig>) => Promise<void>;
  refreshMetrics: () => Promise<void>;
}

// Wallet related hooks
export interface UseWalletConnection {
  connection: Connection | null;
  publicKey: PublicKey | null;
  connected: boolean;
  connecting: boolean;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
} 