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

    #[error("Milestone not completed")]
    MilestoneNotCompleted,

    #[error("Milestone already completed")]
    MilestoneAlreadyCompleted,

    #[error("Invalid investment amount")]
    InvalidInvestmentAmount,

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Project not active")]
    ProjectNotActive,

    #[error("Project already completed")]
    ProjectAlreadyCompleted,

    #[error("Invalid project status transition")]
    InvalidStatusTransition,

    #[error("Refund already claimed")]
    RefundAlreadyClaimed,
}

impl From<CrowdfundError> for ProgramError {
    fn from(e: CrowdfundError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 