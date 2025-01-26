import { Connection, PublicKey, Transaction, SystemProgram } from '@solana/web3.js';
import { PaymentProvider, PaymentStatus } from '../../types';

export class SolanaPaymentProvider implements PaymentProvider {
  private connection: Connection;

  constructor(connection: Connection) {
    this.connection = connection;
  }

  async createPaymentTransaction(
    amount: number,
    from: PublicKey,
    to: PublicKey
  ): Promise<Transaction> {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from,
        toPubkey: to,
        lamports: amount * 1e9 // Convert SOL to lamports
      })
    );

    transaction.feePayer = from;
    const { blockhash } = await this.connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;

    return transaction;
  }

  async checkPaymentStatus(paymentId: string): Promise<PaymentStatus> {
    try {
      const signature = paymentId;
      const status = await this.connection.getSignatureStatus(signature);
      
      if (!status || !status.value) return 'failed';
      
      switch (status.value.confirmationStatus) {
        case 'finalized':
          return 'completed';
        case 'confirmed':
          return 'pending';
        default:
          return 'failed';
      }
    } catch (error) {
      console.error('Failed to check payment status:', error);
      return 'failed';
    }
  }

  async createRefundTransaction(payment: any): Promise<Transaction> {
    return this.createPaymentTransaction(
      payment.amount,
      payment.to,
      payment.from
    );
  }
} 