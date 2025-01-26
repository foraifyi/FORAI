import { Notification, Template } from '../../types';
import { renderTemplate } from '../../utils/template';

export abstract class BaseNotificationService {
  protected abstract getTemplate(type: string): Promise<Template>;
  protected abstract sendNotification(notification: Notification, content: string): Promise<void>;

  async processNotification(notification: Notification): Promise<void> {
    try {
      const template = await this.getTemplate(notification.type);
      const content = await renderTemplate(template, notification.data);
      await this.sendNotification(notification, content);
    } catch (error) {
      console.error('Failed to process notification:', error);
      throw error;
    }
  }

  protected async loadTemplate(type: string): Promise<Template> {
    // Implement logic to load template from database or file system
    return {
      id: type,
      content: '',
      variables: []
    };
  }
} 