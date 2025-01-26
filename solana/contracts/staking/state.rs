use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct StakePool {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub total_staked: u64,
    pub min_stake_duration: i64,  // Minimum staking period
    pub reward_rate: u16,         // Basis points per day (100 = 1%)
    pub early_unstake_penalty: u16, // Basis points (100 = 1%)
    pub stake_count: u32,
}

#[derive(Debug)]
pub struct StakeAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
    pub start_time: i64,
    pub lock_duration: i64,
    pub last_reward_time: i64,
    pub rewards_earned: u64,
    pub is_locked: bool,
}

#[derive(Debug)]
pub struct RewardDistribution {
    pub is_initialized: bool,
    pub pool: Pubkey,
    pub total_rewards: u64,
    pub distributed_rewards: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub is_active: bool,
}

impl Sealed for StakePool {}
impl IsInitialized for StakePool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for StakePool {
    const LEN: usize = 89; // 1 + 32 + 32 + 32 + 8 + 8 + 2 + 2 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, StakePool::LEN];
        let (
            is_initialized,
            authority,
            token_mint,
            token_vault,
            total_staked,
            min_stake_duration,
            reward_rate,
            early_unstake_penalty,
            stake_count,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8, 2, 2, 4];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(StakePool {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            token_mint: Pubkey::new_from_array(*token_mint),
            token_vault: Pubkey::new_from_array(*token_vault),
            total_staked: u64::from_le_bytes(*total_staked),
            min_stake_duration: i64::from_le_bytes(*min_stake_duration),
            reward_rate: u16::from_le_bytes(*reward_rate),
            early_unstake_penalty: u16::from_le_bytes(*early_unstake_penalty),
            stake_count: u32::from_le_bytes(*stake_count),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, StakePool::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            token_mint_dst,
            token_vault_dst,
            total_staked_dst,
            min_stake_duration_dst,
            reward_rate_dst,
            early_unstake_penalty_dst,
            stake_count_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 2, 2, 4];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        token_mint_dst.copy_from_slice(self.token_mint.as_ref());
        token_vault_dst.copy_from_slice(self.token_vault.as_ref());
        *total_staked_dst = self.total_staked.to_le_bytes();
        *min_stake_duration_dst = self.min_stake_duration.to_le_bytes();
        *reward_rate_dst = self.reward_rate.to_le_bytes();
        *early_unstake_penalty_dst = self.early_unstake_penalty.to_le_bytes();
        *stake_count_dst = self.stake_count.to_le_bytes();
    }
}

impl Sealed for StakeAccount {}
impl IsInitialized for StakeAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for StakeAccount {
    const LEN: usize = 98; // 1 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, StakeAccount::LEN];
        let (
            is_initialized,
            owner,
            pool,
            amount,
            start_time,
            lock_duration,
            last_reward_time,
            rewards_earned,
            is_locked,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 8, 8, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_locked = match is_locked[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(StakeAccount {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            pool: Pubkey::new_from_array(*pool),
            amount: u64::from_le_bytes(*amount),
            start_time: i64::from_le_bytes(*start_time),
            lock_duration: i64::from_le_bytes(*lock_duration),
            last_reward_time: i64::from_le_bytes(*last_reward_time),
            rewards_earned: u64::from_le_bytes(*rewards_earned),
            is_locked,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, StakeAccount::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            pool_dst,
            amount_dst,
            start_time_dst,
            lock_duration_dst,
            last_reward_time_dst,
            rewards_earned_dst,
            is_locked_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 8, 8, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        pool_dst.copy_from_slice(self.pool.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *start_time_dst = self.start_time.to_le_bytes();
        *lock_duration_dst = self.lock_duration.to_le_bytes();
        *last_reward_time_dst = self.last_reward_time.to_le_bytes();
        *rewards_earned_dst = self.rewards_earned.to_le_bytes();
        is_locked_dst[0] = self.is_locked as u8;
    }
}

impl Sealed for RewardDistribution {}
impl IsInitialized for RewardDistribution {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for RewardDistribution {
    const LEN: usize = 91; // 1 + 32 + 8 + 8 + 8 + 8 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, RewardDistribution::LEN];
        let (
            is_initialized,
            pool,
            total_rewards,
            distributed_rewards,
            start_time,
            end_time,
            is_active,
        ) = array_refs![src, 1, 32, 8, 8, 8, 8, 1];

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

        Ok(RewardDistribution {
            is_initialized,
            pool: Pubkey::new_from_array(*pool),
            total_rewards: u64::from_le_bytes(*total_rewards),
            distributed_rewards: u64::from_le_bytes(*distributed_rewards),
            start_time: i64::from_le_bytes(*start_time),
            end_time: i64::from_le_bytes(*end_time),
            is_active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, RewardDistribution::LEN];
        let (
            is_initialized_dst,
            pool_dst,
            total_rewards_dst,
            distributed_rewards_dst,
            start_time_dst,
            end_time_dst,
            is_active_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 8, 8, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        pool_dst.copy_from_slice(self.pool.as_ref());
        *total_rewards_dst = self.total_rewards.to_le_bytes();
        *distributed_rewards_dst = self.distributed_rewards.to_le_bytes();
        *start_time_dst = self.start_time.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
        is_active_dst[0] = self.is_active as u8;
    }
} 