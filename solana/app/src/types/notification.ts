export type NotificationType = 
  | 'PROJECT_CREATED'
  | 'PROJECT_UPDATED'
  | 'PROJECT_COMPLETED'
  | 'NEW_INVESTMENT'
  | 'MILESTONE_COMPLETED'
  | 'MILESTONE_VERIFIED';

export interface Notification {
  userId: string;
  type: NotificationType;
  data: any;
  timestamp: Date;
}

export interface EmailConfig {
  fromAddress: string;
  smtpHost: string;
  smtpPort: number;
  username: string;
  password: string;
}

export interface SMSConfig {
  fromNumber: string;
  accountSid: string;
  authToken: string;
}

export interface WebPushConfig {
  publicKey: string;
  privateKey: string;
  subject: string;
}

export interface EmailTemplate {
  subject: string;
  content: string;
}

export interface SMSTemplate {
  content: string;
}

export interface PushTemplate {
  title: string;
  content: string;
  icon: string;
} 