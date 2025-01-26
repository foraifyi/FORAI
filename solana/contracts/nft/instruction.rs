use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
pub enum NFTInstruction {
    /// Initialize a new NFT collection
    /// 
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[signer]` Creator account
    /// 2. `[]` Project account
    /// 3. `[]` System program
    InitializeCollection {
        name: [u8; 32],
        symbol: [u8; 8],
        uri: [u8; 128],
        royalty_percentage: u8,
        total_supply: u64,
    },

    /// Mint new NFT tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[writable]` NFT holder account
    /// 2. `[signer]` Creator account
    /// 3. `[signer]` Recipient account
    MintNFT {
        amount: u64,
    },

    /// Transfer NFT tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[writable]` Sender's NFT holder account
    /// 2. `[writable]` Recipient's NFT holder account
    /// 3. `[signer]` Sender account
    TransferNFT {
        amount: u64,
    },

    /// Lock NFT tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[writable]` NFT holder account
    /// 2. `[signer]` Owner account
    LockNFT {
        amount: u64,
    },

    /// Unlock NFT tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[writable]` NFT holder account
    /// 2. `[signer]` Owner account
    UnlockNFT {
        amount: u64,
    },

    /// Burn NFT tokens
    ///
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[writable]` NFT holder account
    /// 2. `[signer]` Owner account
    BurnNFT {
        amount: u64,
    },

    /// Update NFT metadata
    ///
    /// Accounts expected:
    /// 0. `[writable]` NFT metadata account
    /// 1. `[signer]` Creator account
    UpdateMetadata {
        name: Option<[u8; 32]>,
        uri: Option<[u8; 128]>,
        royalty_percentage: Option<u8>,
    },
}

impl NFTInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let name: [u8; 32] = rest[..32].try_into().unwrap();
                let symbol: [u8; 8] = rest[32..40].try_into().unwrap();
                let uri: [u8; 128] = rest[40..168].try_into().unwrap();
                let royalty_percentage = rest[168];
                let total_supply = rest[169..177].try_into().map(u64::from_le_bytes).unwrap();

                Self::InitializeCollection {
                    name,
                    symbol,
                    uri,
                    royalty_percentage,
                    total_supply,
                }
            }
            1 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::MintNFT { amount }
            }
            2 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::TransferNFT { amount }
            }
            3 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::LockNFT { amount }
            }
            4 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::UnlockNFT { amount }
            }
            5 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::BurnNFT { amount }
            }
            6 => {
                let mut pos = 0;
                let name = if rest[pos] == 1 {
                    pos += 1;
                    let name: [u8; 32] = rest[pos..pos+32].try_into().unwrap();
                    pos += 32;
                    Some(name)
                } else {
                    pos += 1;
                    None
                };

                let uri = if rest[pos] == 1 {
                    pos += 1;
                    let uri: [u8; 128] = rest[pos..pos+128].try_into().unwrap();
                    pos += 128;
                    Some(uri)
                } else {
                    pos += 1;
                    None
                };

                let royalty_percentage = if rest[pos] == 1 {
                    pos += 1;
                    Some(rest[pos])
                } else {
                    None
                };

                Self::UpdateMetadata {
                    name,
                    uri,
                    royalty_percentage,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitializeCollection {
                name,
                symbol,
                uri,
                royalty_percentage,
                total_supply,
            } => {
                buf.push(0);
                buf.extend_from_slice(name);
                buf.extend_from_slice(symbol);
                buf.extend_from_slice(uri);
                buf.push(*royalty_percentage);
                buf.extend_from_slice(&total_supply.to_le_bytes());
            }
            Self::MintNFT { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::TransferNFT { amount } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::LockNFT { amount } => {
                buf.push(3);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::UnlockNFT { amount } => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::BurnNFT { amount } => {
                buf.push(5);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::UpdateMetadata {
                name,
                uri,
                royalty_percentage,
            } => {
                buf.push(6);
                
                // Pack name
                if let Some(name) = name {
                    buf.push(1);
                    buf.extend_from_slice(name);
                } else {
                    buf.push(0);
                }

                // Pack uri
                if let Some(uri) = uri {
                    buf.push(1);
                    buf.extend_from_slice(uri);
                } else {
                    buf.push(0);
                }

                // Pack royalty_percentage
                if let Some(percentage) = royalty_percentage {
                    buf.push(1);
                    buf.push(*percentage);
                } else {
                    buf.push(0);
                }
            }
        }
        buf
    }
} 