use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CrowdfundError {
    #[error("Project already initialized")]
    AlreadyInitialized,

    #[error("Project not initialized")]
    NotInitialized,

    #[error("Project already funded")]
    AlreadyFunded,

    #[error("Project funding period ended")]
    FundingPeriodEnded,

    #[error("Project funding goal not reached")]
    FundingGoalNotReached,

    #[error("Invalid milestone index")]
    InvalidMilestoneIndex,

    #[error("Invalid milestone sequence")]
    InvalidMilestoneSequence,

    #[error("Invalid investment amount")]
    InvalidInvestmentAmount,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Insufficient project funds")]
    InsufficientProjectFunds,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Project not active")]
    ProjectNotActive,

    #[error("Project not started")]
    ProjectNotStarted,

    #[error("Project ended")]
    ProjectEnded,

    #[error("Invalid project status transition")]
    InvalidStatusTransition,

    #[error("Refund already claimed")]
    RefundAlreadyClaimed,

    #[error("Account not rent exempt")]
    NotRentExempt,

    #[error("Duplicate account in instruction")]
    DuplicateAccount,
}

impl From<CrowdfundError> for ProgramError {
    fn from(e: CrowdfundError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 