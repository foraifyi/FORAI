export interface Template {
  id: string;
  content: string;
  subject?: string;
  variables: string[];
}

export interface EmailTemplate extends Template {
  subject: string;
}

export interface SMSTemplate extends Template {
  content: string;
}

export interface PushTemplate extends Template {
  title: string;
  content: string;
  icon: string;
}

export interface TemplateData {
  [key: string]: any;
} 