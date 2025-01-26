import { PublicKey } from '@solana/web3.js';

export interface Project {
  id: string;
  title: string;
  description: string;
  owner: PublicKey;
  status: ProjectStatus;
  targetAmount: number;
  raisedAmount: number;
  startDate: number;
  endDate: number;
  milestones: Milestone[];
}

export enum ProjectStatus {
  Draft = 'DRAFT',
  Active = 'ACTIVE',
  Funded = 'FUNDED',
  Completed = 'COMPLETED',
  Cancelled = 'CANCELLED'
}

export interface Milestone {
  id: string;
  title: string;
  description: string;
  targetAmount: number;
  deadline: number;
  status: MilestoneStatus;
  completionDate?: number;
}

export enum MilestoneStatus {
  Pending = 'PENDING',
  InProgress = 'IN_PROGRESS',
  Completed = 'COMPLETED',
  Failed = 'FAILED'
}

export interface Investment {
  id: string;
  investor: PublicKey;
  project: PublicKey;
  amount: number;
  timestamp: number;
  status: InvestmentStatus;
}

export enum InvestmentStatus {
  Pending = 'PENDING',
  Confirmed = 'CONFIRMED',
  Failed = 'FAILED',
  Refunded = 'REFUNDED'
} 