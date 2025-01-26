use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum NFTError {
    #[error("NFT collection already initialized")]
    AlreadyInitialized,

    #[error("NFT collection not initialized")]
    NotInitialized,

    #[error("Invalid NFT metadata")]
    InvalidMetadata,

    #[error("Invalid NFT amount")]
    InvalidAmount,

    #[error("NFT supply exceeded")]
    SupplyExceeded,

    #[error("NFT is locked")]
    TokenLocked,

    #[error("Insufficient NFT balance")]
    InsufficientBalance,

    #[error("Invalid NFT authority")]
    InvalidAuthority,

    #[error("NFT operation not allowed")]
    OperationNotAllowed,

    #[error("Invalid royalty percentage")]
    InvalidRoyaltyPercentage,

    #[error("NFT already burned")]
    AlreadyBurned,

    #[error("Invalid NFT status")]
    InvalidStatus,

    #[error("NFT holder account not found")]
    HolderNotFound,

    #[error("Invalid NFT holder account")]
    InvalidHolderAccount,

    #[error("NFT transfer not allowed")]
    TransferNotAllowed,
}

impl From<NFTError> for ProgramError {
    fn from(e: NFTError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 