import { Connection, PublicKey, Transaction, SystemProgram as SolanaSystemProgram } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

export class EscrowContract {
  private connection: Connection;
  private program: Program;

  constructor(connection: Connection, program: Program) {
    this.connection = connection;
    this.program = program;
  }

  // Create escrow account
  async createEscrow(
    projectId: PublicKey,
    amount: number,
    milestones: number[]
  ): Promise<Transaction> {
    const escrowAccount = new PublicKey(); // Generate new escrow account
    
    return this.program.methods
      .createEscrow({
        projectId,
        amount,
        milestones,
        releaseSchedule: milestones.map((m, i) => ({
          milestone: i,
          amount: m,
          released: false
        }))
      })
      .accounts({
        escrow: escrowAccount,
        project: projectId,
        systemProgram: SolanaSystemProgram.programId
      })
      .transaction();
  }

  // Hold funds in escrow
  async holdFunds(
    escrowId: PublicKey,
    amount: number
  ): Promise<Transaction> {
    return this.program.methods
      .holdFunds(amount)
      .accounts({
        escrow: escrowId,
        systemProgram: SolanaSystemProgram.programId
      })
      .transaction();
  }

  // Distribute funds
  async distributeFunds(
    escrowId: PublicKey,
    milestoneIndex: number
  ): Promise<Transaction> {
    return this.program.methods
      .distributeFunds(milestoneIndex)
      .accounts({
        escrow: escrowId,
        systemProgram: SolanaSystemProgram.programId
      })
      .transaction();
  }

  // Emergency stop
  async emergencyStop(escrowId: PublicKey): Promise<Transaction> {
    return this.program.methods
      .emergencyStop()
      .accounts({
        escrow: escrowId,
        systemProgram: SolanaSystemProgram.programId
      })
      .transaction();
  }

  // Cancel escrow
  async cancelEscrow(escrowId: PublicKey): Promise<Transaction> {
    return this.program.methods
      .cancelEscrow()
      .accounts({
        escrow: escrowId,
        systemProgram: SolanaSystemProgram.programId
      })
      .transaction();
  }
} 