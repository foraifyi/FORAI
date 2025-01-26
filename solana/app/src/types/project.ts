import { PublicKey } from '@solana/web3.js';

export interface Project {
  publicKey: PublicKey;
  owner: PublicKey;
  title: string;
  description: string;
  targetAmount: number;
  currentAmount: number;
  status: ProjectStatus;
  milestones: Milestone[];
  startTime: number;
  endTime: number;
}

export enum ProjectStatus {
  Draft = 'DRAFT',
  Active = 'ACTIVE',
  Funded = 'FUNDED',
  Completed = 'COMPLETED',
  Cancelled = 'CANCELLED'
}

export interface Milestone {
  description: string;
  targetAmount: number;
  completionTime: number;
  completed: boolean;
  fundsReleased: boolean;
} 