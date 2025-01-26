use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct LockConfig {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub total_locked: u64,
    pub min_lock_duration: i64,    // Minimum lock period
    pub max_lock_duration: i64,    // Maximum lock period
    pub unlock_fee: u16,          // Basis points (100 = 1%)
    pub early_unlock_penalty: u16, // Additional penalty for early unlock
    pub lock_count: u32,
}

#[derive(Debug)]
pub struct TokenLock {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub unlock_available: i64,     // Time when tokens can be unlocked
    pub is_early_unlock: bool,     // Whether early unlock was requested
    pub is_active: bool,
}

#[derive(Debug)]
pub struct UnlockRequest {
    pub is_initialized: bool,
    pub lock_account: Pubkey,
    pub requester: Pubkey,
    pub amount: u64,
    pub request_time: i64,
    pub unlock_time: i64,
    pub is_approved: bool,
    pub is_processed: bool,
}

impl Sealed for LockConfig {}
impl IsInitialized for LockConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for LockConfig {
    const LEN: usize = 89; // 1 + 32 + 32 + 8 + 8 + 8 + 2 + 2 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, LockConfig::LEN];
        let (
            is_initialized,
            authority,
            token_mint,
            total_locked,
            min_lock_duration,
            max_lock_duration,
            unlock_fee,
            early_unlock_penalty,
            lock_count,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 2, 2, 4];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(LockConfig {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            token_mint: Pubkey::new_from_array(*token_mint),
            total_locked: u64::from_le_bytes(*total_locked),
            min_lock_duration: i64::from_le_bytes(*min_lock_duration),
            max_lock_duration: i64::from_le_bytes(*max_lock_duration),
            unlock_fee: u16::from_le_bytes(*unlock_fee),
            early_unlock_penalty: u16::from_le_bytes(*early_unlock_penalty),
            lock_count: u32::from_le_bytes(*lock_count),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, LockConfig::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            token_mint_dst,
            total_locked_dst,
            min_lock_duration_dst,
            max_lock_duration_dst,
            unlock_fee_dst,
            early_unlock_penalty_dst,
            lock_count_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 2, 2, 4];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        token_mint_dst.copy_from_slice(self.token_mint.as_ref());
        *total_locked_dst = self.total_locked.to_le_bytes();
        *min_lock_duration_dst = self.min_lock_duration.to_le_bytes();
        *max_lock_duration_dst = self.max_lock_duration.to_le_bytes();
        *unlock_fee_dst = self.unlock_fee.to_le_bytes();
        *early_unlock_penalty_dst = self.early_unlock_penalty.to_le_bytes();
        *lock_count_dst = self.lock_count.to_le_bytes();
    }
}

impl Sealed for TokenLock {}
impl IsInitialized for TokenLock {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TokenLock {
    const LEN: usize = 99; // 1 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, TokenLock::LEN];
        let (
            is_initialized,
            owner,
            token_account,
            amount,
            start_time,
            end_time,
            unlock_available,
            is_early_unlock,
            is_active,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 8, 1, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_early_unlock = match is_early_unlock[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_active = match is_active[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(TokenLock {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            token_account: Pubkey::new_from_array(*token_account),
            amount: u64::from_le_bytes(*amount),
            start_time: i64::from_le_bytes(*start_time),
            end_time: i64::from_le_bytes(*end_time),
            unlock_available: i64::from_le_bytes(*unlock_available),
            is_early_unlock,
            is_active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, TokenLock::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            token_account_dst,
            amount_dst,
            start_time_dst,
            end_time_dst,
            unlock_available_dst,
            is_early_unlock_dst,
            is_active_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 8, 1, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        token_account_dst.copy_from_slice(self.token_account.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *start_time_dst = self.start_time.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
        *unlock_available_dst = self.unlock_available.to_le_bytes();
        is_early_unlock_dst[0] = self.is_early_unlock as u8;
        is_active_dst[0] = self.is_active as u8;
    }
}

impl Sealed for UnlockRequest {}
impl IsInitialized for UnlockRequest {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for UnlockRequest {
    const LEN: usize = 98; // 1 + 32 + 32 + 8 + 8 + 8 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, UnlockRequest::LEN];
        let (
            is_initialized,
            lock_account,
            requester,
            amount,
            request_time,
            unlock_time,
            is_approved,
            is_processed,
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

        let is_processed = match is_processed[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(UnlockRequest {
            is_initialized,
            lock_account: Pubkey::new_from_array(*lock_account),
            requester: Pubkey::new_from_array(*requester),
            amount: u64::from_le_bytes(*amount),
            request_time: i64::from_le_bytes(*request_time),
            unlock_time: i64::from_le_bytes(*unlock_time),
            is_approved,
            is_processed,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, UnlockRequest::LEN];
        let (
            is_initialized_dst,
            lock_account_dst,
            requester_dst,
            amount_dst,
            request_time_dst,
            unlock_time_dst,
            is_approved_dst,
            is_processed_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 1, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        lock_account_dst.copy_from_slice(self.lock_account.as_ref());
        requester_dst.copy_from_slice(self.requester.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *request_time_dst = self.request_time.to_le_bytes();
        *unlock_time_dst = self.unlock_time.to_le_bytes();
        is_approved_dst[0] = self.is_approved as u8;
        is_processed_dst[0] = self.is_processed as u8;
    }
} 