use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
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

    #[error("Invalid pool")]
    InvalidPool,

    #[error("Invalid stake account")]
    InvalidStakeAccount,

    #[error("Invalid reward distribution")]
    InvalidRewardDistribution,

    #[error("Insufficient stake amount")]
    InsufficientStakeAmount,

    #[error("Stake locked")]
    StakeLocked,

    #[error("Stake not locked")]
    StakeNotLocked,

    #[error("Invalid lock duration")]
    InvalidLockDuration,

    #[error("Lock period not met")]
    LockPeriodNotMet,

    #[error("Invalid reward rate")]
    InvalidRewardRate,

    #[error("Invalid penalty rate")]
    InvalidPenaltyRate,

    #[error("Invalid reward calculation")]
    InvalidRewardCalculation,

    #[error("Insufficient rewards")]
    InsufficientRewards,

    #[error("Distribution not active")]
    DistributionNotActive,

    #[error("Distribution already active")]
    DistributionAlreadyActive,

    #[error("Invalid distribution period")]
    InvalidDistributionPeriod,

    #[error("Distribution period ended")]
    DistributionPeriodEnded,

    #[error("Invalid unstake amount")]
    InvalidUnstakeAmount,

    #[error("Insufficient token balance")]
    InsufficientTokenBalance,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for StakingError {
    fn type_of() -> &'static str {
        "StakingError"
    }
} 