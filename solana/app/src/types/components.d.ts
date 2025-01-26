import { Project, Milestone, Investment } from './project';
import { User } from './user';
import { SecurityConfig, SecurityMetrics } from './security';
import { RiskMetrics } from './risk';
import { ComplianceCheck, ComplianceResult } from './compliance';

// Project details component Props
export interface ProjectDetailProps {
  project: Project;
  onUpdate?: (project: Project) => void;
}

// Investment form component Props
export interface InvestmentFormProps {
  project: Project;
  onInvest: (amount: number) => Promise<void>;
}

// Milestone management component Props
export interface MilestoneManagerProps {
  milestones: Milestone[];
  onUpdate: (milestones: Milestone[]) => void;
  onComplete: (milestoneId: string) => Promise<void>;
}

// Management panel component Props
export interface AdminDashboardProps {
  user: User;
  securityConfig: SecurityConfig;
  metrics: SecurityMetrics;
}

// User management component Props
export interface UserManagementProps {
  users: User[];
  onUpdateRole: (userId: string, role: string) => Promise<void>;
  onUpdateStatus: (userId: string, status: string) => Promise<void>;
}

// Security monitoring component Props
export interface SecurityMonitorProps {
  metrics: SecurityMetrics;
  onAlertResolution: (alertId: string) => Promise<void>;
}

// Risk dashboard component Props
export interface RiskDashboardProps {
  metrics: RiskMetrics;
  onRecommendationAction: (recommendationId: string) => Promise<void>;
}

// Compliance check component Props
export interface ComplianceCheckerProps {
  checks: ComplianceCheck[];
  onRunCheck: (checkIds: string[]) => Promise<ComplianceResult[]>;
}

// Audit log component Props
export interface AuditLogProps {
  filters: {
    startDate: string;
    endDate: string;
    eventType: string;
    severity: string;
  };
  onFilterChange: (filters: any) => void;
} 