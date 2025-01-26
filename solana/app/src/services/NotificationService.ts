import { EmailService } from './email/EmailService';
import { SMSService } from './sms/SMSService';
import { WebPushService } from './push/WebPushService';
import { ProjectEvent, InvestmentEvent, MilestoneEvent, NotificationType } from '../types';

export class NotificationService {
  private emailService: EmailService;
  private smsService: SMSService;
  private webPushService: WebPushService;

  constructor(
    emailService: EmailService,
    smsService: SMSService,
    webPushService: WebPushService
  ) {
    this.emailService = emailService;
    this.smsService = smsService;
    this.webPushService = webPushService;
  }

  // Send notification
  private async sendNotification(
    userId: string,
    type: NotificationType,
    data: any,
    channels: string[]
  ): Promise<void> {
    const notification = {
      userId,
      type,
      data,
      timestamp: new Date(),
    };

    const promises = [];

    if (channels.includes('email')) {
      promises.push(this.emailService.sendNotification(notification));
    }

    if (channels.includes('sms')) {
      promises.push(this.smsService.sendNotification(notification));
    }

    if (channels.includes('webpush')) {
      promises.push(this.webPushService.sendNotification(notification));
    }

    await Promise.all(promises);
  }

  // Project related notifications
  async notifyProjectCreated(event: ProjectEvent): Promise<void> {
    await this.sendNotification(
      event.projectOwner,
      'PROJECT_CREATED',
      event,
      ['email', 'webpush']
    );
  }

  async notifyProjectUpdated(event: ProjectEvent): Promise<void> {
    await this.sendNotification(
      event.projectOwner,
      'PROJECT_UPDATED',
      event,
      ['email', 'webpush']
    );
  }

  // Investment related notifications
  async notifyNewInvestment(event: InvestmentEvent): Promise<void> {
    await this.sendNotification(
      event.investor,
      'NEW_INVESTMENT',
      event,
      ['email', 'sms', 'webpush']
    );
  }

  // Milestone related notifications
  async notifyMilestoneCompleted(event: MilestoneEvent): Promise<void> {
    await this.sendNotification(
      event.projectOwner,
      'MILESTONE_COMPLETED',
      event,
      ['email', 'webpush']
    );
  }

  // ... other notification methods
} 