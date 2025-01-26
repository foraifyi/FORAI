use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug)]
pub enum InsuranceInstruction {
    /// Initialize insurance pool
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New pool account
    /// 2. `[signer]` Authority
    /// 3. `[]` System program
    InitializePool {
        min_stake_amount: u64,
        claim_delay_period: i64,
        max_claim_amount: u64,
    },

    /// Stake funds into the pool
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Pool account
    /// 2. `[writable]` Stake account
    /// 3. `[signer]` Staker
    Stake {
        amount: u64,
        lock_period: i64,
    },

    /// File a claim
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Pool account
    /// 2. `[writable]` Claim account
    /// 3. `[signer]` Claimer
    /// 4. `[]` Project account
    FileClaim {
        amount: u64,
        evidence_uri: [u8; 128],
    },

    /// Review and approve/reject claim
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Pool account
    /// 2. `[writable]` Claim account
    /// 3. `[signer]` Authority
    ReviewClaim {
        approved: bool,
    },

    /// Process approved claim payment
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Pool account
    /// 2. `[writable]` Claim account
    /// 3. `[writable]` Claimer account
    /// 4. `[signer]` Authority
    ProcessClaim,
} 