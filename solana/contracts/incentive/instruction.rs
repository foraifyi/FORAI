use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug)]
pub enum IncentiveInstruction {
    /// Initialize a new agent account
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New agent account
    /// 2. `[signer]` Agent authority
    /// 3. `[]` System program
    InitializeAgent,

    /// Reward an agent for completed task
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Agent account to reward
    /// 2. `[signer]` Program authority
    /// 3. `[writable]` Treasury account
    RewardAgent {
        amount: u64,
        reputation_increase: u64,
    },
} 