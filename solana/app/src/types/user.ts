import { PublicKey } from '@solana/web3.js';

export interface User {
  id: string;
  publicKey: PublicKey;
  role: UserRole;
  status: UserStatus;
  createdAt: number;
  lastActive: number;
}

export enum UserRole {
  Admin = 'ADMIN',
  Manager = 'MANAGER',
  Investor = 'INVESTOR',
  Basic = 'BASIC'
}

export enum UserStatus {
  Active = 'ACTIVE',
  Suspended = 'SUSPENDED',
  Pending = 'PENDING'
} 