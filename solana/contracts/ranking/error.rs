use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum RankingError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid agent owner")]
    InvalidAgentOwner,

    #[error("Invalid reviewer")]
    InvalidReviewer,

    #[error("Stake amount below minimum")]
    StakeBelowMinimum,

    #[error("Agent not active")]
    AgentNotActive,

    #[error("Invalid performance score")]
    InvalidPerformanceScore,

    #[error("Performance period not met")]
    PerformancePeriodNotMet,

    #[error("Insufficient stake")]
    InsufficientStake,

    #[error("Invalid reward calculation")]
    InvalidRewardCalculation,

    #[error("Invalid penalty calculation")]
    InvalidPenaltyCalculation,

    #[error("Task already reviewed")]
    TaskAlreadyReviewed,

    #[error("Invalid task account")]
    InvalidTaskAccount,

    #[error("Invalid feedback URI")]
    InvalidFeedbackURI,

    #[error("Agent already registered")]
    AgentAlreadyRegistered,

    #[error("Performance record exists")]
    PerformanceRecordExists,

    #[error("Invalid performance update")]
    InvalidPerformanceUpdate,

    #[error("Insufficient rewards")]
    InsufficientRewards,

    #[error("Invalid claim period")]
    InvalidClaimPeriod,
}

impl From<RankingError> for ProgramError {
    fn from(e: RankingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for RankingError {
    fn type_of() -> &'static str {
        "RankingError"
    }
} 