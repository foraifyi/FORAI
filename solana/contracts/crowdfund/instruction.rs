use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Debug)]
pub enum CrowdfundInstruction {
    /// Initialize a new project
    /// 
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[signer]` Project owner
    /// 2. `[]` Treasury account
    /// 3. `[]` System program
    InitializeProject {
        title: [u8; 32],
        description: [u8; 64],
        target_amount: u64,
        start_time: i64,
        end_time: i64,
        milestones: Vec<Milestone>,
    },

    /// Invest in a project
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[writable]` Investment account
    /// 2. `[signer]` Investor
    /// 3. `[writable]` Treasury account
    Invest {
        amount: u64,
    },

    /// Complete a milestone
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[signer]` Project owner
    /// 2. `[writable]` Treasury account
    CompleteMilestone {
        milestone_index: u8,
    },

    /// Claim refund for failed project
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[writable]` Investment account
    /// 2. `[signer]` Investor
    /// 3. `[writable]` Treasury account
    ClaimRefund,

    /// Cancel project (only by owner before funding ends)
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[signer]` Project owner
    CancelProject,
}

impl CrowdfundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // ... implement instruction unpacking
        unimplemented!()
    }
} 