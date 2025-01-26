use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct TokenConfig {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub treasury: Pubkey,
    pub total_supply: u64,
    pub circulating_supply: u64,
    pub decimals: u8,
    pub transfer_enabled: bool,
    pub mint_enabled: bool,
}

#[derive(Debug)]
pub struct MintRequest {
    pub is_initialized: bool,
    pub requester: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub request_time: i64,
    pub expiry_time: i64,
    pub is_approved: bool,
    pub is_executed: bool,
}

#[derive(Debug)]
pub struct TransferLimit {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub daily_limit: u64,
    pub transferred_today: u64,
    pub last_day: i64,
    pub is_exempt: bool,
}

impl Sealed for TokenConfig {}
impl IsInitialized for TokenConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

// Implement Pack traits
