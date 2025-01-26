import { PublicKey, Transaction } from '@solana/web3.js';

export type PaymentStatus = 'pending' | 'completed' | 'failed' | 'refunded';

export type PaymentMethod = 'SOL' | 'USDC';

export interface PaymentProvider {
  createPaymentTransaction(
    amount: number,
    from: PublicKey,
    to: PublicKey
  ): Promise<Transaction>;
  
  checkPaymentStatus(paymentId: string): Promise<PaymentStatus>;
  
  createRefundTransaction(payment: Payment): Promise<Transaction>;
}

export interface Payment {
  id: string;
  amount: number;
  from: PublicKey;
  to: PublicKey;
  method: PaymentMethod;
  status: PaymentStatus;
  timestamp: Date;
} 