use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct DistributionConfig {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub total_distributed: u64,
    pub distribution_rate: u64,     // Tokens per epoch
    pub min_epoch_duration: i64,    // Minimum time between distributions
    pub last_distribution: i64,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct Recipient {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub allocation: u16,            // Basis points (100 = 1%)
    pub total_received: u64,
    pub last_claim: i64,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct DistributionRound {
    pub is_initialized: bool,
    pub round_number: u32,
    pub total_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub is_finalized: bool,
    pub recipients_claimed: u32,
}

impl Sealed for DistributionConfig {}
impl IsInitialized for DistributionConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for DistributionConfig {
    const LEN: usize = 90; // 1 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, DistributionConfig::LEN];
        let (
            is_initialized,
            authority,
            token_mint,
            token_vault,
            total_distributed,
            distribution_rate,
            min_epoch_duration,
            last_distribution,
            is_active,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8, 8, 8, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_active = match is_active[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(DistributionConfig {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            token_mint: Pubkey::new_from_array(*token_mint),
            token_vault: Pubkey::new_from_array(*token_vault),
            total_distributed: u64::from_le_bytes(*total_distributed),
            distribution_rate: u64::from_le_bytes(*distribution_rate),
            min_epoch_duration: i64::from_le_bytes(*min_epoch_duration),
            last_distribution: i64::from_le_bytes(*last_distribution),
            is_active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, DistributionConfig::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            token_mint_dst,
            token_vault_dst,
            total_distributed_dst,
            distribution_rate_dst,
            min_epoch_duration_dst,
            last_distribution_dst,
            is_active_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 8, 8, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        token_mint_dst.copy_from_slice(self.token_mint.as_ref());
        token_vault_dst.copy_from_slice(self.token_vault.as_ref());
        *total_distributed_dst = self.total_distributed.to_le_bytes();
        *distribution_rate_dst = self.distribution_rate.to_le_bytes();
        *min_epoch_duration_dst = self.min_epoch_duration.to_le_bytes();
        *last_distribution_dst = self.last_distribution.to_le_bytes();
        is_active_dst[0] = self.is_active as u8;
    }
}

impl Sealed for Recipient {}
impl IsInitialized for Recipient {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Recipient {
    const LEN: usize = 87; // 1 + 32 + 32 + 2 + 8 + 8 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Recipient::LEN];
        let (
            is_initialized,
            owner,
            token_account,
            allocation,
            total_received,
            last_claim,
            is_active,
        ) = array_refs![src, 1, 32, 32, 2, 8, 8, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_active = match is_active[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Recipient {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            token_account: Pubkey::new_from_array(*token_account),
            allocation: u16::from_le_bytes(*allocation),
            total_received: u64::from_le_bytes(*total_received),
            last_claim: i64::from_le_bytes(*last_claim),
            is_active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Recipient::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            token_account_dst,
            allocation_dst,
            total_received_dst,
            last_claim_dst,
            is_active_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 2, 8, 8, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        token_account_dst.copy_from_slice(self.token_account.as_ref());
        *allocation_dst = self.allocation.to_le_bytes();
        *total_received_dst = self.total_received.to_le_bytes();
        *last_claim_dst = self.last_claim.to_le_bytes();
        is_active_dst[0] = self.is_active as u8;
    }
}

impl Sealed for DistributionRound {}
impl IsInitialized for DistributionRound {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for DistributionRound {
    const LEN: usize = 38; // 1 + 4 + 8 + 8 + 8 + 1 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, DistributionRound::LEN];
        let (
            is_initialized,
            round_number,
            total_amount,
            start_time,
            end_time,
            is_finalized,
            recipients_claimed,
        ) = array_refs![src, 1, 4, 8, 8, 8, 1, 4];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_finalized = match is_finalized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(DistributionRound {
            is_initialized,
            round_number: u32::from_le_bytes(*round_number),
            total_amount: u64::from_le_bytes(*total_amount),
            start_time: i64::from_le_bytes(*start_time),
            end_time: i64::from_le_bytes(*end_time),
            is_finalized,
            recipients_claimed: u32::from_le_bytes(*recipients_claimed),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, DistributionRound::LEN];
        let (
            is_initialized_dst,
            round_number_dst,
            total_amount_dst,
            start_time_dst,
            end_time_dst,
            is_finalized_dst,
            recipients_claimed_dst,
        ) = mut_array_refs![dst, 1, 4, 8, 8, 8, 1, 4];

        is_initialized_dst[0] = self.is_initialized as u8;
        *round_number_dst = self.round_number.to_le_bytes();
        *total_amount_dst = self.total_amount.to_le_bytes();
        *start_time_dst = self.start_time.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
        is_finalized_dst[0] = self.is_finalized as u8;
        *recipients_claimed_dst = self.recipients_claimed.to_le_bytes();
    }
} 