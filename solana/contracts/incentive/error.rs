use solana_program::{program_error::ProgramError, decode_error::DecodeError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum IncentiveError {
    #[error("Account not initialized")]
    UninitializedAccount,
    
    #[error("Account already initialized")]
    AlreadyInitialized,
    
    #[error("Invalid program authority")]
    InvalidProgramAuthority,
    
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<IncentiveError> for ProgramError {
    fn from(e: IncentiveError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 