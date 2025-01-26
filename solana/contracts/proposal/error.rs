use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ProposalError {
    #[error("Account not initialized")]
    UninitializedAccount,

    #[error("Account already initialized")]
    AlreadyInitialized,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Invalid proposer")]
    InvalidProposer,

    #[error("Invalid voter")]
    InvalidVoter,

    #[error("Invalid voting power program")]
    InvalidVotingPowerProgram,

    #[error("Invalid voting power account")]
    InvalidVotingPowerAccount,

    #[error("Invalid voting checkpoint")]
    InvalidVotingCheckpoint,

    #[error("Invalid proposal")]
    InvalidProposal,

    #[error("Invalid vote")]
    InvalidVote,

    #[error("Invalid vote support value")]
    InvalidVoteSupport,

    #[error("Insufficient voting power")]
    InsufficientVotingPower,

    #[error("Voting not started")]
    VotingNotStarted,

    #[error("Voting already ended")]
    VotingAlreadyEnded,

    #[error("Voting still active")]
    VotingStillActive,

    #[error("Invalid voting period")]
    InvalidVotingPeriod,

    #[error("Invalid execution time")]
    InvalidExecutionTime,

    #[error("Execution time not reached")]
    ExecutionTimeNotReached,

    #[error("Proposal already canceled")]
    ProposalAlreadyCanceled,

    #[error("Proposal already executed")]
    ProposalAlreadyExecuted,

    #[error("Proposal not executable")]
    ProposalNotExecutable,

    #[error("Quorum not reached")]
    QuorumNotReached,

    #[error("Already voted")]
    AlreadyVoted,

    #[error("Title too long")]
    TitleTooLong,

    #[error("Description URL too long")]
    DescriptionUrlTooLong,

    #[error("Invalid vote calculation")]
    InvalidVoteCalculation,

    #[error("Math overflow")]
    MathOverflow,
}

impl From<ProposalError> for ProgramError {
    fn from(e: ProposalError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ProposalError {
    fn type_of() -> &'static str {
        "ProposalError"
    }
} 