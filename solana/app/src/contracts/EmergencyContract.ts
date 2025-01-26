import { Connection, PublicKey, Transaction, SystemProgram } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

export class EmergencyContract {
  private connection: Connection;
  private program: Program;

  constructor(connection: Connection, program: Program) {
    this.connection = connection;
    this.program = program;
  }

  // Emergency pause project
  async pauseProject(projectId: PublicKey): Promise<Transaction> {
    return this.program.methods
      .pauseProject()
      .accounts({
        project: projectId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }

  // Resume project
  async resumeProject(projectId: PublicKey): Promise<Transaction> {
    return this.program.methods
      .resumeProject()
      .accounts({
        project: projectId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }

  // Emergency withdrawal
  async emergencyWithdraw(
    projectId: PublicKey,
    amount: number
  ): Promise<Transaction> {
    return this.program.methods
      .emergencyWithdraw(amount)
      .accounts({
        project: projectId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }

  // Set emergency contact
  async setEmergencyContact(
    projectId: PublicKey,
    contact: PublicKey
  ): Promise<Transaction> {
    return this.program.methods
      .setEmergencyContact(contact)
      .accounts({
        project: projectId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }
} 