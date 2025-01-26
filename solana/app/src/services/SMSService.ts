import { SMSConfig, SMSTemplate, Notification } from '../types';
import { renderTemplate } from '../utils/template';

export class SMSService {
  private config: SMSConfig;

  constructor(config: SMSConfig) {
    this.config = config;
  }

  async sendNotification(notification: Notification): Promise<void> {
    try {
      const template = await this.getTemplate(notification.type);
      const content = await renderTemplate(template, notification.data);
      
      await this.sendSMS({
        to: notification.userId,
        content,
        from: this.config.fromNumber
      });
    } catch (error) {
      console.error('Failed to send SMS notification:', error);
      throw error;
    }
  }

  private async sendSMS(options: any): Promise<void> {
    // Implement SMS sending logic
  }

  private async getTemplate(type: string): Promise<SMSTemplate> {
    // Get SMS template
    return {
      content: ''
    };
  }
} 