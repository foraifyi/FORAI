use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum DistributionError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Invalid token mint")]
    InvalidTokenMint,

    #[error("Invalid token account")]
    InvalidTokenAccount,

    #[error("Invalid token vault")]
    InvalidTokenVault,

    #[error("Invalid recipient")]
    InvalidRecipient,

    #[error("Invalid round")]
    InvalidRound,

    #[error("Invalid allocation")]
    InvalidAllocation,

    #[error("Distribution not active")]
    DistributionNotActive,

    #[error("Distribution already active")]
    DistributionAlreadyActive,

    #[error("Round not started")]
    RoundNotStarted,

    #[error("Round already ended")]
    RoundAlreadyEnded,

    #[error("Round already finalized")]
    RoundAlreadyFinalized,

    #[error("Round not finalized")]
    RoundNotFinalized,

    #[error("Invalid round duration")]
    InvalidRoundDuration,

    #[error("Invalid distribution rate")]
    InvalidDistributionRate,

    #[error("Invalid distribution amount")]
    InvalidDistributionAmount,

    #[error("Distribution period not met")]
    DistributionPeriodNotMet,

    #[error("Already claimed")]
    AlreadyClaimed,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Recipient not active")]
    RecipientNotActive,

    #[error("Total allocation exceeded")]
    TotalAllocationExceeded,

    #[error("Invalid claim calculation")]
    InvalidClaimCalculation,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<DistributionError> for ProgramError {
    fn from(e: DistributionError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for DistributionError {
    fn type_of() -> &'static str {
        "DistributionError"
    }
} 