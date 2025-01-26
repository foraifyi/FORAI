use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum InsuranceError {
    #[error("Insurance pool already initialized")]
    AlreadyInitialized,

    #[error("Insurance pool not initialized")]
    NotInitialized,

    #[error("Insurance pool is paused")]
    PoolPaused,

    #[error("Insurance pool is liquidated")]
    PoolLiquidated,

    #[error("Invalid pool parameters")]
    InvalidPoolParameters,

    #[error("Insufficient pool capital")]
    InsufficientPoolCapital,

    #[error("Invalid capital amount")]
    InvalidCapitalAmount,

    #[error("Invalid coverage amount")]
    InvalidCoverageAmount,

    #[error("Invalid premium amount")]
    InvalidPremiumAmount,

    #[error("Invalid claim amount")]
    InvalidClaimAmount,

    #[error("Invalid policy duration")]
    InvalidPolicyDuration,

    #[error("Policy already expired")]
    PolicyExpired,

    #[error("Policy not expired")]
    PolicyNotExpired,

    #[error("Policy already claimed")]
    PolicyAlreadyClaimed,

    #[error("Claim period expired")]
    ClaimPeriodExpired,

    #[error("Claim already processed")]
    ClaimAlreadyProcessed,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid account owner")]
    InvalidAccountOwner,

    #[error("Account not rent exempt")]
    NotRentExempt,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Arithmetic overflow")]
    Overflow,

    #[error("Operation not allowed")]
    OperationNotAllowed,
}

impl From<InsuranceError> for ProgramError {
    fn from(e: InsuranceError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 