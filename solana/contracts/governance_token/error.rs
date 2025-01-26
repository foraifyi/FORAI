use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum GovernanceTokenError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid requester")]
    InvalidRequester,

    #[error("Invalid recipient")]
    InvalidRecipient,

    #[error("Invalid token mint")]
    InvalidTokenMint,

    #[error("Invalid token account")]
    InvalidTokenAccount,

    #[error("Invalid treasury account")]
    InvalidTreasuryAccount,

    #[error("Invalid mint request")]
    InvalidMintRequest,

    #[error("Invalid transfer limit")]
    InvalidTransferLimit,

    #[error("Invalid decimals")]
    InvalidDecimals,

    #[error("Invalid amount")]
    InvalidAmount,

    #[error("Request expired")]
    RequestExpired,

    #[error("Request not approved")]
    RequestNotApproved,

    #[error("Daily limit exceeded")]
    DailyLimitExceeded,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<GovernanceTokenError> for ProgramError {
    fn from(e: GovernanceTokenError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for GovernanceTokenError {
    fn type_of() -> &'static str {
        "GovernanceTokenError"
    }
}
