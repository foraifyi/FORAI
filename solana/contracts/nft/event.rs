use solana_program::{
    account_info::AccountInfo,
    msg,
    pubkey::Pubkey,
};

pub enum NFTEvent<'a> {
    // Collection events
    CollectionCreated {
        metadata: &'a Pubkey,
        creator: &'a Pubkey,
        name: [u8; 32],
        symbol: [u8; 8],
        total_supply: u64,
    },
    MetadataUpdated {
        metadata: &'a Pubkey,
        updater: &'a Pubkey,
        timestamp: i64,
    },

    // Token events
    TokensMinted {
        metadata: &'a Pubkey,
        recipient: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    TokensTransferred {
        metadata: &'a Pubkey,
        from: &'a Pubkey,
        to: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    TokensLocked {
        metadata: &'a Pubkey,
        owner: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    TokensUnlocked {
        metadata: &'a Pubkey,
        owner: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    TokensBurned {
        metadata: &'a Pubkey,
        owner: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },

    // Royalty events
    RoyaltyPaid {
        metadata: &'a Pubkey,
        recipient: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },

    // Error events
    OperationFailed {
        metadata: &'a Pubkey,
        operation: &'a str,
        error: &'a str,
        timestamp: i64,
    },
}

impl<'a> NFTEvent<'a> {
    pub fn emit(&self) {
        match self {
            // Collection events
            Self::CollectionCreated { 
                metadata, 
                creator, 
                name, 
                symbol, 
                total_supply 
            } => {
                msg!("NFT Collection Created: metadata={}, creator={}, name={:?}, symbol={:?}, supply={}", 
                    metadata, creator, name, symbol, total_supply);
            }
            Self::MetadataUpdated { 
                metadata, 
                updater, 
                timestamp 
            } => {
                msg!("NFT Metadata Updated: metadata={}, updater={}, time={}", 
                    metadata, updater, timestamp);
            }

            // Token events
            Self::TokensMinted { 
                metadata, 
                recipient, 
                amount, 
                timestamp 
            } => {
                msg!("NFT Tokens Minted: metadata={}, recipient={}, amount={}, time={}", 
                    metadata, recipient, amount, timestamp);
            }
            Self::TokensTransferred { 
                metadata, 
                from, 
                to, 
                amount, 
                timestamp 
            } => {
                msg!("NFT Tokens Transferred: metadata={}, from={}, to={}, amount={}, time={}", 
                    metadata, from, to, amount, timestamp);
            }
            Self::TokensLocked { 
                metadata, 
                owner, 
                amount, 
                timestamp 
            } => {
                msg!("NFT Tokens Locked: metadata={}, owner={}, amount={}, time={}", 
                    metadata, owner, amount, timestamp);
            }
            Self::TokensUnlocked { 
                metadata, 
                owner, 
                amount, 
                timestamp 
            } => {
                msg!("NFT Tokens Unlocked: metadata={}, owner={}, amount={}, time={}", 
                    metadata, owner, amount, timestamp);
            }
            Self::TokensBurned { 
                metadata, 
                owner, 
                amount, 
                timestamp 
            } => {
                msg!("NFT Tokens Burned: metadata={}, owner={}, amount={}, time={}", 
                    metadata, owner, amount, timestamp);
            }

            // Royalty events
            Self::RoyaltyPaid { 
                metadata, 
                recipient, 
                amount, 
                timestamp 
            } => {
                msg!("NFT Royalty Paid: metadata={}, recipient={}, amount={}, time={}", 
                    metadata, recipient, amount, timestamp);
            }

            // Error events
            Self::OperationFailed { 
                metadata, 
                operation, 
                error, 
                timestamp 
            } => {
                msg!("NFT Operation Failed: metadata={}, operation={}, error={}, time={}", 
                    metadata, operation, error, timestamp);
            }
        }
    }
} 