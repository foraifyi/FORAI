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

impl Pack for TokenConfig {
    const LEN: usize = 115; // 1 + 32 + 32 + 32 + 8 + 8 + 1 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, TokenConfig::LEN];
        let (
            is_initialized,
            authority,
            token_mint,
            treasury,
            total_supply,
            circulating_supply,
            decimals,
            transfer_enabled,
            mint_enabled,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8, 1, 1, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let transfer_enabled = match transfer_enabled[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let mint_enabled = match mint_enabled[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(TokenConfig {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            token_mint: Pubkey::new_from_array(*token_mint),
            treasury: Pubkey::new_from_array(*treasury),
            total_supply: u64::from_le_bytes(*total_supply),
            circulating_supply: u64::from_le_bytes(*circulating_supply),
            decimals: decimals[0],
            transfer_enabled,
            mint_enabled,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, TokenConfig::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            token_mint_dst,
            treasury_dst,
            total_supply_dst,
            circulating_supply_dst,
            decimals_dst,
            transfer_enabled_dst,
            mint_enabled_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 1, 1, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        token_mint_dst.copy_from_slice(self.token_mint.as_ref());
        treasury_dst.copy_from_slice(self.treasury.as_ref());
        *total_supply_dst = self.total_supply.to_le_bytes();
        *circulating_supply_dst = self.circulating_supply.to_le_bytes();
        decimals_dst[0] = self.decimals;
        transfer_enabled_dst[0] = self.transfer_enabled as u8;
        mint_enabled_dst[0] = self.mint_enabled as u8;
    }
}

impl Sealed for MintRequest {}
impl IsInitialized for MintRequest {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for MintRequest {
    const LEN: usize = 98; // 1 + 32 + 32 + 8 + 8 + 8 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, MintRequest::LEN];
        let (
            is_initialized,
            requester,
            recipient,
            amount,
            request_time,
            expiry_time,
            is_approved,
            is_executed,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 1, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_approved = match is_approved[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_executed = match is_executed[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(MintRequest {
            is_initialized,
            requester: Pubkey::new_from_array(*requester),
            recipient: Pubkey::new_from_array(*recipient),
            amount: u64::from_le_bytes(*amount),
            request_time: i64::from_le_bytes(*request_time),
            expiry_time: i64::from_le_bytes(*expiry_time),
            is_approved,
            is_executed,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, MintRequest::LEN];
        let (
            is_initialized_dst,
            requester_dst,
            recipient_dst,
            amount_dst,
            request_time_dst,
            expiry_time_dst,
            is_approved_dst,
            is_executed_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 1, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        requester_dst.copy_from_slice(self.requester.as_ref());
        recipient_dst.copy_from_slice(self.recipient.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *request_time_dst = self.request_time.to_le_bytes();
        *expiry_time_dst = self.expiry_time.to_le_bytes();
        is_approved_dst[0] = self.is_approved as u8;
        is_executed_dst[0] = self.is_executed as u8;
    }
}

impl Sealed for TransferLimit {}
impl IsInitialized for TransferLimit {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TransferLimit {
    const LEN: usize = 54; // 1 + 32 + 8 + 8 + 8 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, TransferLimit::LEN];
        let (
            is_initialized,
            owner,
            daily_limit,
            transferred_today,
            last_day,
            is_exempt,
        ) = array_refs![src, 1, 32, 8, 8, 8, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_exempt = match is_exempt[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(TransferLimit {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            daily_limit: u64::from_le_bytes(*daily_limit),
            transferred_today: u64::from_le_bytes(*transferred_today),
            last_day: i64::from_le_bytes(*last_day),
            is_exempt,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, TransferLimit::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            daily_limit_dst,
            transferred_today_dst,
            last_day_dst,
            is_exempt_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 8, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        *daily_limit_dst = self.daily_limit.to_le_bytes();
        *transferred_today_dst = self.transferred_today.to_le_bytes();
        *last_day_dst = self.last_day.to_le_bytes();
        is_exempt_dst[0] = self.is_exempt as u8;
    }
} 