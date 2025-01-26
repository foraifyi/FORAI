import { Project, Investment, Milestone } from './project';
import { User } from './user';
import { SecurityConfig, SecurityMetrics } from './security';

// API response types
export interface APIResponse<T> {
  success: boolean;
  data?: T;
  error?: {
    code: string;
    message: string;
    details?: any;
  };
  meta?: {
    page?: number;
    limit?: number;
    total?: number;
  };
}

// API request types
export interface APIRequest {
  path: string;
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  params?: Record<string, any>;
  body?: any;
  headers?: Record<string, string>;
}

// API endpoint types
export interface APIEndpoints {
  projects: {
    list: () => Promise<APIResponse<Project[]>>;
    get: (id: string) => Promise<APIResponse<Project>>;
    create: (data: Partial<Project>) => Promise<APIResponse<Project>>;
    update: (id: string, data: Partial<Project>) => Promise<APIResponse<Project>>;
    delete: (id: string) => Promise<APIResponse<void>>;
  };
  investments: {
    list: () => Promise<APIResponse<Investment[]>>;
    create: (data: Partial<Investment>) => Promise<APIResponse<Investment>>;
    withdraw: (id: string) => Promise<APIResponse<void>>;
  };
  users: {
    me: () => Promise<APIResponse<User>>;
    update: (data: Partial<User>) => Promise<APIResponse<User>>;
    list: () => Promise<APIResponse<User[]>>;
  };
  security: {
    getConfig: () => Promise<APIResponse<SecurityConfig>>;
    updateConfig: (data: Partial<SecurityConfig>) => Promise<APIResponse<SecurityConfig>>;
    getMetrics: () => Promise<APIResponse<SecurityMetrics>>;
  };
} 