use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum InsuranceError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Stake amount below minimum")]
    StakeBelowMinimum,

    #[error("Stake still locked")]
    StakeLocked,

    #[error("Claim amount exceeds maximum")]
    ClaimExceedsMaximum,

    #[error("Claim delay period not met")]
    ClaimDelayNotMet,

    #[error("Insufficient pool funds")]
    InsufficientPoolFunds,

    #[error("Claim already processed")]
    ClaimAlreadyProcessed,

    #[error("Claim not approved")]
    ClaimNotApproved,

    #[error("Invalid claim evidence")]
    InvalidClaimEvidence,

    #[error("Invalid stake account")]
    InvalidStakeAccount,
}

impl From<InsuranceError> for ProgramError {
    fn from(e: InsuranceError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 