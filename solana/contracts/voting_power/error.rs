use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum VotingPowerError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Invalid delegate")]
    InvalidDelegate,

    #[error("Invalid governance token")]
    InvalidGovernanceToken,

    #[error("Invalid token account")]
    InvalidTokenAccount,

    #[error("Invalid checkpoint")]
    InvalidCheckpoint,

    #[error("Invalid checkpoint interval")]
    InvalidCheckpointInterval,

    #[error("Invalid voting power")]
    InvalidVotingPower,

    #[error("Invalid delegation amount")]
    InvalidDelegationAmount,

    #[error("Delegation not enabled")]
    DelegationNotEnabled,

    #[error("Checkpoint not enabled")]
    CheckpointNotEnabled,

    #[error("Checkpoint too early")]
    CheckpointTooEarly,

    #[error("Checkpoint buffer overflow")]
    CheckpointBufferOverflow,

    #[error("Too many accounts")]
    TooManyAccounts,

    #[error("Self delegation not allowed")]
    SelfDelegationNotAllowed,

    #[error("Already delegated")]
    AlreadyDelegated,

    #[error("Not delegated")]
    NotDelegated,

    #[error("Insufficient delegation balance")]
    InsufficientDelegationBalance,

    #[error("Insufficient voting power")]
    InsufficientVotingPower,

    #[error("Invalid token amount")]
    InvalidTokenAmount,

    #[error("Invalid token owner")]
    InvalidTokenOwner,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<VotingPowerError> for ProgramError {
    fn from(e: VotingPowerError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VotingPowerError {
    fn type_of() -> &'static str {
        "VotingPowerError"
    }
} 