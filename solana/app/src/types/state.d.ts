import { Project, Investment, Milestone } from './project';
import { User } from './user';
import { SecurityConfig, SecurityMetrics } from './security';
import { RiskMetrics } from './risk';

// Application state
export interface AppState {
  projects: ProjectState;
  investments: InvestmentState;
  users: UserState;
  security: SecurityState;
  ui: UIState;
}

// Project state
export interface ProjectState {
  items: { [key: string]: Project };
  loading: boolean;
  error: Error | null;
  selectedId: string | null;
}

// Investment state
export interface InvestmentState {
  items: { [key: string]: Investment };
  loading: boolean;
  error: Error | null;
  userInvestments: string[];
}

// User state
export interface UserState {
  currentUser: User | null;
  users: { [key: string]: User };
  loading: boolean;
  error: Error | null;
}

// Security state
export interface SecurityState {
  config: SecurityConfig | null;
  metrics: SecurityMetrics | null;
  riskMetrics: RiskMetrics | null;
  loading: boolean;
  error: Error | null;
}

// UI state
export interface UIState {
  theme: 'light' | 'dark';
  language: string;
  notifications: Notification[];
  modals: {
    [key: string]: boolean;
  };
  sidebar: {
    isOpen: boolean;
    activeItem: string;
  };
}

// Notifications
export interface Notification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  message: string;
  timestamp: number;
  read: boolean;
} 