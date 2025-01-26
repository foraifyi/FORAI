use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum TokenLockError {
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

    #[error("Invalid lock account")]
    InvalidLockAccount,

    #[error("Invalid unlock request")]
    InvalidUnlockRequest,

    #[error("Lock duration too short")]
    LockDurationTooShort,

    #[error("Lock duration too long")]
    LockDurationTooLong,

    #[error("Invalid lock amount")]
    InvalidLockAmount,

    #[error("Invalid unlock amount")]
    InvalidUnlockAmount,

    #[error("Lock not active")]
    LockNotActive,

    #[error("Lock already ended")]
    LockAlreadyEnded,

    #[error("Lock still active")]
    LockStillActive,

    #[error("Unlock not available")]
    UnlockNotAvailable,

    #[error("Early unlock not allowed")]
    EarlyUnlockNotAllowed,

    #[error("Early unlock already requested")]
    EarlyUnlockAlreadyRequested,

    #[error("Unlock request not approved")]
    UnlockRequestNotApproved,

    #[error("Unlock request already processed")]
    UnlockRequestAlreadyProcessed,

    #[error("Unlock request expired")]
    UnlockRequestExpired,

    #[error("Invalid fee calculation")]
    InvalidFeeCalculation,

    #[error("Invalid penalty calculation")]
    InvalidPenaltyCalculation,

    #[error("Insufficient token balance")]
    InsufficientTokenBalance,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<TokenLockError> for ProgramError {
    fn from(e: TokenLockError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TokenLockError {
    fn type_of() -> &'static str {
        "TokenLockError"
    }
} 