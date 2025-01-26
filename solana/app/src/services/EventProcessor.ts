import { Connection, PublicKey } from '@solana/web3.js';
import { NotificationService } from './NotificationService';
import { AnalyticsService } from './AnalyticsService';
import { ProjectEvent, InvestmentEvent, MilestoneEvent } from '../types';

export class EventProcessor {
  private connection: Connection;
  private notificationService: NotificationService;
  private analyticsService: AnalyticsService;

  constructor(
    connection: Connection,
    notificationService: NotificationService,
    analyticsService: AnalyticsService
  ) {
    this.connection = connection;
    this.notificationService = notificationService;
    this.analyticsService = analyticsService;
  }

  // Handle project event
  async handleProjectEvent(event: ProjectEvent): Promise<void> {
    try {
      // Record event
      await this.analyticsService.trackEvent('project', event);

      // Send notifications
      switch (event.type) {
        case 'created':
          await this.notificationService.notifyProjectCreated(event);
          break;
        case 'updated':
          await this.notificationService.notifyProjectUpdated(event);
          break;
        case 'completed':
          await this.notificationService.notifyProjectCompleted(event);
          break;
        case 'cancelled':
          await this.notificationService.notifyProjectCancelled(event);
          break;
      }
    } catch (error) {
      console.error('Failed to handle project event:', error);
      throw error;
    }
  }

  // Handle investment event
  async handleInvestmentEvent(event: InvestmentEvent): Promise<void> {
    try {
      // Record event
      await this.analyticsService.trackEvent('investment', event);

      // Send notifications
      switch (event.type) {
        case 'invested':
          await this.notificationService.notifyNewInvestment(event);
          break;
        case 'withdrawn':
          await this.notificationService.notifyInvestmentWithdrawn(event);
          break;
        case 'refunded':
          await this.notificationService.notifyInvestmentRefunded(event);
          break;
      }
    } catch (error) {
      console.error('Failed to handle investment event:', error);
      throw error;
    }
  }

  // Handle milestone event
  async handleMilestoneEvent(event: MilestoneEvent): Promise<void> {
    try {
      // Record event
      await this.analyticsService.trackEvent('milestone', event);

      // Send notifications
      switch (event.type) {
        case 'completed':
          await this.notificationService.notifyMilestoneCompleted(event);
          break;
        case 'verified':
          await this.notificationService.notifyMilestoneVerified(event);
          break;
        case 'rejected':
          await this.notificationService.notifyMilestoneRejected(event);
          break;
      }
    } catch (error) {
      console.error('Failed to handle milestone event:', error);
      throw error;
    }
  }
} 