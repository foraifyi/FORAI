use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug)]
pub enum CrowdfundInstruction {
    /// Initialize a new project
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New project account
    /// 2. `[signer]` Project owner
    /// 3. `[writable]` Treasury account
    /// 4. `[]` System program
    InitializeProject {
        target_amount: u64,
        deadline: i64,
        milestone_count: u8,
    },

    /// Invest in a project
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Project account
    /// 2. `[signer]` Investor
    /// 3. `[writable]` Investment account
    /// 4. `[writable]` Treasury account
    Invest {
        amount: u64,
    },

    /// Release milestone payment
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Project account
    /// 2. `[signer]` Project owner
    /// 3. `[writable]` Treasury account
    ReleaseMilestone,
} 