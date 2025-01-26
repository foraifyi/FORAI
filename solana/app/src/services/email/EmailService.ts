import nodemailer from 'nodemailer';
import { EmailConfig, EmailTemplate, Notification } from '../../types';
import { BaseNotificationService } from '../base/BaseNotificationService';

export class EmailService extends BaseNotificationService {
  private transporter: nodemailer.Transporter;
  private config: EmailConfig;

  constructor(config: EmailConfig) {
    super();
    this.config = config;
    this.transporter = nodemailer.createTransport({
      host: config.smtpHost,
      port: config.smtpPort,
      secure: true,
      auth: {
        user: config.username,
        pass: config.password
      }
    });
  }

  protected async getTemplate(type: string): Promise<EmailTemplate> {
    const template = await this.loadTemplate(type) as EmailTemplate;
    if (!template) {
      throw new Error(`Email template not found for type: ${type}`);
    }
    return template;
  }

  protected async sendNotification(notification: Notification, content: string): Promise<void> {
    const template = await this.getTemplate(notification.type);
    await this.sendEmail({
      to: notification.userId,
      subject: template.subject,
      html: content,
      from: this.config.fromAddress
    });
  }

  private async sendEmail(options: nodemailer.SendMailOptions): Promise<void> {
    try {
      await this.transporter.sendMail(options);
    } catch (error) {
      console.error('Failed to send email:', error);
      throw error;
    }
  }
} 