import webpush from 'web-push';
import { WebPushConfig, PushTemplate, Notification } from '../../types';
import { renderTemplate } from '../../utils/template';

export class WebPushService {
  private config: WebPushConfig;

  constructor(config: WebPushConfig) {
    this.config = config;
    webpush.setVapidDetails(
      config.subject,
      config.publicKey,
      config.privateKey
    );
  }

  async sendNotification(notification: Notification): Promise<void> {
    try {
      const template = await this.getTemplate(notification.type);
      const content = await renderTemplate(template, notification.data);
      
      const subscription = await this.getSubscription(notification.userId);
      if (!subscription) return;

      await this.sendPush({
        subscription,
        title: template.title,
        body: content,
        icon: template.icon
      });
    } catch (error) {
      console.error('Failed to send push notification:', error);
      throw error;
    }
  }

  private async sendPush(options: any): Promise<void> {
    try {
      const payload = JSON.stringify({
        title: options.title,
        body: options.body,
        icon: options.icon
      });

      await webpush.sendNotification(options.subscription, payload);
    } catch (error) {
      console.error('Failed to send push notification:', error);
      throw error;
    }
  }

  private async getTemplate(type: string): Promise<PushTemplate> {
    // Get template from template storage
    const template = await this.loadTemplate(type);
    if (!template) {
      throw new Error(`Push template not found for type: ${type}`);
    }
    return template;
  }

  private async loadTemplate(type: string): Promise<PushTemplate> {
    // Implement logic to load template from database or file system
    return {
      title: 'Default Title',
      content: 'Default Content',
      icon: '/default-icon.png'
    };
  }

  private async getSubscription(userId: string): Promise<webpush.PushSubscription | null> {
    // Get user's push subscription information from database
    return null;
  }
} 