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

    #[error("Invalid expiry time")]
    InvalidExpiryTime,

    #[error("Invalid daily limit")]
    InvalidDailyLimit,

    #[error("Mint not enabled")]
    MintNotEnabled,

    #[error("Transfer not enabled")]
    TransferNotEnabled,

    #[error("Request already approved")]
    RequestAlreadyApproved,

    #[error("Request already executed")]
    RequestAlreadyExecuted,

    #[error("Request expired")]
    RequestExpired,

    #[error("Request not approved")]
    RequestNotApproved,

    #[error("Daily limit exceeded")]
    DailyLimitExceeded,

    #[error("Transfer limit not set")]
    TransferLimitNotSet,

    #[error("Supply exceeded")]
    SupplyExceeded,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid fee calculation")]
    InvalidFeeCalculation,

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