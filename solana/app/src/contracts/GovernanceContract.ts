import { Connection, PublicKey, Transaction, SystemProgram } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

export class GovernanceContract {
  private connection: Connection;
  private program: Program;

  constructor(connection: Connection, program: Program) {
    this.connection = connection;
    this.program = program;
  }

  // Create proposal
  async createProposal(
    projectId: PublicKey,
    title: string,
    description: string,
    options: string[]
  ): Promise<Transaction> {
    const proposalAccount = new PublicKey(); // Generate new proposal account

    return this.program.methods
      .createProposal({
        projectId,
        title,
        description,
        options,
        status: 'active',
        votingPeriod: 7 * 24 * 60 * 60 // 7 days
      })
      .accounts({
        proposal: proposalAccount,
        project: projectId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }

  // Vote
  async vote(
    proposalId: PublicKey,
    optionIndex: number,
    voter: PublicKey
  ): Promise<Transaction> {
    return this.program.methods
      .vote(optionIndex)
      .accounts({
        proposal: proposalId,
        voter,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }

  // Execute proposal
  async executeProposal(proposalId: PublicKey): Promise<Transaction> {
    return this.program.methods
      .executeProposal()
      .accounts({
        proposal: proposalId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }

  // Cancel proposal
  async cancelProposal(proposalId: PublicKey): Promise<Transaction> {
    return this.program.methods
      .cancelProposal()
      .accounts({
        proposal: proposalId,
        systemProgram: SystemProgram.programId
      })
      .transaction();
  }
} 