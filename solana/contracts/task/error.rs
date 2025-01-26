use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum TaskError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid task creator")]
    InvalidTaskCreator,

    #[error("Invalid agent")]
    InvalidAgent,

    #[error("Agent not qualified")]
    AgentNotQualified,

    #[error("Task already assigned")]
    TaskAlreadyAssigned,

    #[error("Task not assigned")]
    TaskNotAssigned,

    #[error("Task already completed")]
    TaskAlreadyCompleted,

    #[error("Task already failed")]
    TaskAlreadyFailed,

    #[error("Task already cancelled")]
    TaskAlreadyCancelled,

    #[error("Task timeout")]
    TaskTimeout,

    #[error("Invalid task status")]
    InvalidTaskStatus,

    #[error("Invalid task priority")]
    InvalidTaskPriority,

    #[error("Invalid reward multiplier")]
    InvalidRewardMultiplier,

    #[error("Invalid specification URI")]
    InvalidSpecificationURI,

    #[error("Invalid result URI")]
    InvalidResultURI,

    #[error("Invalid feedback")]
    InvalidFeedback,

    #[error("Insufficient stake")]
    InsufficientStake,

    #[error("Invalid performance score")]
    InvalidPerformanceScore,

    #[error("Task in dispute")]
    TaskInDispute,

    #[error("Unauthorized reviewer")]
    UnauthorizedReviewer,

    #[error("Invalid task account")]
    InvalidTaskAccount,

    #[error("Invalid pool account")]
    InvalidPoolAccount,
}

impl From<TaskError> for ProgramError {
    fn from(e: TaskError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for TaskError {
    fn type_of() -> &'static str {
        "TaskError"
    }
} 