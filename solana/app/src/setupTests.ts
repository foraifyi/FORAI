import '@testing-library/jest-dom';
import { TextEncoder, TextDecoder } from 'util';
import { jest } from '@jest/globals';

global.TextEncoder = TextEncoder;
global.TextDecoder = TextDecoder as any;

// Mock window.solana
const mockSolana = {
  isPhantom: true,
  connect: jest.fn(),
  disconnect: jest.fn(),
  on: jest.fn(),
  request: jest.fn(),
};

Object.defineProperty(window, 'solana', {
  value: mockSolana,
});

// Mock window.matchMedia
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(),
    removeListener: jest.fn(),
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

// Mock window.ResizeObserver
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

// Mock Solana web3 objects
jest.mock('@solana/web3.js', () => ({
  Connection: jest.fn(),
  PublicKey: jest.fn(),
  Transaction: jest.fn(),
  SystemProgram: {
    transfer: jest.fn()
  }
}));

// Mock wallet adapter
jest.mock('@solana/wallet-adapter-react', () => ({
  useConnection: () => ({ connection: {} }),
  useWallet: () => ({
    publicKey: null,
    connected: false,
    connecting: false,
    connect: jest.fn(),
    disconnect: jest.fn(),
    signTransaction: jest.fn()
  })
})); 