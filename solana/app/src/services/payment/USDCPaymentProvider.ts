import { Connection, PublicKey, Transaction } from '@solana/web3.js';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { PaymentProvider, PaymentStatus } from '../../types';

export class USDCPaymentProvider implements PaymentProvider {
  private connection: Connection;
  private usdcMint: PublicKey;

  constructor(connection: Connection) {
    this.connection = connection;
    this.usdcMint = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'); // USDC mint address
  }

  async createPaymentTransaction(
    amount: number,
    from: PublicKey,
    to: PublicKey
  ): Promise<Transaction> {
    const fromTokenAccount = await Token.getAssociatedTokenAddress(
      this.usdcMint,
      from
    );

    const toTokenAccount = await Token.getAssociatedTokenAddress(
      this.usdcMint,
      to
    );

    const transaction = new Transaction().add(
      Token.createTransferInstruction(
        TOKEN_PROGRAM_ID,
        fromTokenAccount,
        toTokenAccount,
        from,
        [],
        amount * 1e6 // Convert USDC to smallest unit
      )
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