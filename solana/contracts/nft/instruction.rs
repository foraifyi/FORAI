use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(Debug)]
pub enum NFTInstruction {
    /// Mint a new software NFT
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New NFT account
    /// 2. `[signer]` Project owner
    /// 3. `[]` Project account
    /// 4. `[]` System program
    MintNFT {
        metadata_uri: [u8; 128],
        version: u32,
        is_transferable: bool,
        can_modify: bool,
        royalty_percentage: u8,
    },

    /// Transfer NFT ownership
    /// 
    /// Accounts expected:
    /// 1. `[writable]` NFT account
    /// 2. `[signer]` Current owner
    /// 3. `[]` New owner
    TransferNFT,

    /// Update software version
    /// 
    /// Accounts expected:
    /// 1. `[writable]` NFT account
    /// 2. `[signer]` Owner
    /// 3. `[]` Project account
    UpdateVersion {
        new_version: u32,
        new_metadata_uri: [u8; 128],
    },
} 