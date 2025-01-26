use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum GovernanceError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid guardian")]
    InvalidGuardian,

    #[error("Invalid governance token")]
    InvalidGovernanceToken,

    #[error("Invalid proposal program")]
    InvalidProposalProgram,

    #[error("Invalid proposal")]
    InvalidProposal,

    #[error("Invalid action")]
    InvalidAction,

    #[error("Invalid queue")]
    InvalidQueue,

    #[error("Invalid voting delay")]
    InvalidVotingDelay,

    #[error("Invalid voting period")]
    InvalidVotingPeriod,

    #[error("Invalid timelock delay")]
    InvalidTimelockDelay,

    #[error("Invalid execution time")]
    InvalidExecutionTime,

    #[error("Invalid action accounts")]
    InvalidActionAccounts,

    #[error("Invalid action data")]
    InvalidActionData,

    #[error("Action buffer overflow")]
    ActionBufferOverflow,

    #[error("Queue buffer overflow")]
    QueueBufferOverflow,

    #[error("Governance not active")]
    GovernanceNotActive,

    #[error("Execution time not reached")]
    ExecutionTimeNotReached,

    #[error("Actions already executed")]
    ActionsAlreadyExecuted,

    #[error("Actions already canceled")]
    ActionsAlreadyCanceled,

    #[error("Action execution failed")]
    ActionExecutionFailed,

    #[error("Too many actions")]
    TooManyActions,

    #[error("Action data too large")]
    ActionDataTooLarge,

    #[error("Action accounts too many")]
    ActionAccountsTooMany,

    #[error("Insufficient voting power")]
    InsufficientVotingPower,

    #[error("Quorum not reached")]
    QuorumNotReached,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<GovernanceError> for ProgramError {
    fn from(e: GovernanceError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for GovernanceError {
    fn type_of() -> &'static str {
        "GovernanceError"
    }
} 