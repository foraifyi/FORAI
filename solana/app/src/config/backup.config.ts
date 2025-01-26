export const backupConfig = {
  schedule: {
    full: '0 0 * * 0',  // Every Sunday at midnight
    incremental: '0 0 * * 1-6'  // Monday to Saturday at midnight
  },
  storage: {
    type: 'cloud',
    provider: process.env.BACKUP_STORAGE_PROVIDER || 'aws',
    bucket: process.env.BACKUP_BUCKET_NAME,
    region: process.env.BACKUP_REGION || 'us-east-1'
  },
  retention: {
    full: 30,  // Keep for 30 days
    incremental: 7  // Keep for 7 days
  },
  encryption: {
    enabled: true,
    algorithm: 'AES-256-GCM'
  }
} as const;

export type BackupConfig = typeof backupConfig;

// Database backup
export const database = {
  schedule: '0 0 * * *', // Daily at midnight
  retention: {
    days: 30,
    copies: 10,
  },
  storage: {
    type: 's3',
    bucket: process.env.BACKUP_BUCKET || 'crowdfund-backups',
    path: 'database',
  },
};

// File backup
export const files = {
  schedule: '0 0 * * 0', // Every Sunday
  paths: [
    '/app/uploads',
    '/app/config',
  ],
  exclude: [
    '*.tmp',
    '*.log',
  ],
  compression: {
    enabled: true,
    type: 'gzip',
  },
};

// Recovery configuration
export const recovery = {
  verifyBackup: true,
  maxAttempts: 3,
  notifyOnFailure: true,
}; 