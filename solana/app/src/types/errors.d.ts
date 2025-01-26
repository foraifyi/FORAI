// Base error types
export class BaseError extends Error {
  constructor(message: string);
  code: string;
  context?: Record<string, any>;
}

// Blockchain errors
export class BlockchainError extends BaseError {
  constructor(message: string, txId?: string);
  txId?: string;
}

// Contract errors
export class ContractError extends BaseError {
  constructor(message: string, contractAddress: string);
  contractAddress: string;
}

// Validation errors
export class ValidationError extends BaseError {
  constructor(message: string, field: string);
  field: string;
}

// Permission errors
export class AuthorizationError extends BaseError {
  constructor(message: string, requiredRole: string);
  requiredRole: string;
}

// API errors
export class APIError extends BaseError {
  constructor(message: string, statusCode: number);
  statusCode: number;
  isRetryable: boolean;
}

// Configuration errors
export class ConfigurationError extends BaseError {
  constructor(message: string, configKey: string);
  configKey: string;
} 