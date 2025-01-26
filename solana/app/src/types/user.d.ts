import { PublicKey } from '@solana/web3.js';

export interface User {
  id: string;
  publicKey: PublicKey;
  role: UserRole;
  status: UserStatus;
  createdAt: number;
  lastLogin?: number;
  profile?: UserProfile;
}

export enum UserRole {
  Admin = 'ADMIN',
  Manager = 'MANAGER',
  User = 'USER'
}

export enum UserStatus {
  Active = 'ACTIVE',
  Inactive = 'INACTIVE',
  Suspended = 'SUSPENDED'
}

export interface UserProfile {
  name?: string;
  email?: string;
  avatar?: string;
  bio?: string;
  socialLinks?: {
    twitter?: string;
    github?: string;
    linkedin?: string;
  };
} 