use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CrowdfundError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid project owner")]
    InvalidProjectOwner,

    #[error("Project deadline has passed")]
    DeadlinePassed,

    #[error("Project not fully funded")]
    NotFullyFunded,

    #[error("Project already completed")]
    ProjectCompleted,

    #[error("Invalid milestone")]
    InvalidMilestone,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid investment amount")]
    InvalidInvestmentAmount,

    #[error("Investment account already exists")]
    InvestmentExists,

    #[error("Treasury account mismatch")]
    TreasuryMismatch,
}

impl From<CrowdfundError> for ProgramError {
    fn from(e: CrowdfundError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 