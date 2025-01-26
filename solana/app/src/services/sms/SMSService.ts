import twilio from 'twilio';
import { SMSConfig, SMSTemplate, Notification } from '../../types';
import { renderTemplate } from '../../utils/template';

export class SMSService {
  private client: twilio.Twilio;
  private config: SMSConfig;

  constructor(config: SMSConfig) {
    this.config = config;
    this.client = twilio(config.accountSid, config.authToken);
  }

  async sendNotification(notification: Notification): Promise<void> {
    try {
      const template = await this.getTemplate(notification.type);
      const content = await renderTemplate(template, notification.data);
      
      await this.sendSMS({
        to: notification.userId,
        body: content,
        from: this.config.fromNumber
      });
    } catch (error) {
      console.error('Failed to send SMS notification:', error);
      throw error;
    }
  }

  private async sendSMS(options: twilio.MessageInstance): Promise<void> {
    try {
      await this.client.messages.create(options);
    } catch (error) {
      console.error('Failed to send SMS:', error);
      throw error;
    }
  }

  private async getTemplate(type: string): Promise<SMSTemplate> {
    // Get template from template storage
    const template = await this.loadTemplate(type);
    if (!template) {
      throw new Error(`SMS template not found for type: ${type}`);
    }
    return template;
  }

  private async loadTemplate(type: string): Promise<SMSTemplate> {
    // Implement logic to load template from database or file system
    return {
      content: 'Default SMS Content'
    };
  }
} 