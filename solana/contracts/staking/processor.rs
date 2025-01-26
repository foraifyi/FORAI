use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
    program::{invoke, invoke_signed},
};
use spl_token::state::Account as TokenAccount;

use crate::{
    state::{StakePool, StakeAccount, RewardDistribution},
    error::StakingError,
    instruction::StakingInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = StakingInstruction::unpack(instruction_data)?;

        match instruction {
            StakingInstruction::InitializePool { 
                min_stake_duration,
                reward_rate,
                early_unstake_penalty,
            } => {
                Self::process_initialize_pool(
                    accounts,
                    min_stake_duration,
                    reward_rate,
                    early_unstake_penalty,
                )
            }
            StakingInstruction::Stake { amount, lock_duration } => {
                Self::process_stake(accounts, amount, lock_duration)
            }
            StakingInstruction::Unstake => {
                Self::process_unstake(accounts)
            }
            StakingInstruction::ClaimRewards => {
                Self::process_claim_rewards(accounts)
            }
            StakingInstruction::StartDistribution { total_rewards, duration } => {
                Self::process_start_distribution(accounts, total_rewards, duration)
            }
        }
    }

    fn process_initialize_pool(
        accounts: &[AccountInfo],
        min_stake_duration: i64,
        reward_rate: u16,
        early_unstake_penalty: u16,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;
        let token_vault = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = StakePool::unpack_unchecked(&pool_account.data.borrow())?;
        if pool.is_initialized {
            return Err(StakingError::AlreadyInitialized.into());
        }

        if reward_rate > 10000 || early_unstake_penalty > 10000 {
            return Err(StakingError::InvalidRewardRate.into());
        }

        pool.is_initialized = true;
        pool.authority = *authority.key;
        pool.token_mint = *token_mint.key;
        pool.token_vault = *token_vault.key;
        pool.total_staked = 0;
        pool.min_stake_duration = min_stake_duration;
        pool.reward_rate = reward_rate;
        pool.early_unstake_penalty = early_unstake_penalty;
        pool.stake_count = 0;

        StakePool::pack(pool, &mut pool_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_stake(
        accounts: &[AccountInfo],
        amount: u64,
        lock_duration: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let stake_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let owner_token = next_account_info(account_info_iter)?;
        let pool_token_vault = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = StakePool::unpack(&pool_account.data.borrow())?;
        let mut stake = StakeAccount::unpack_unchecked(&stake_account.data.borrow())?;

        if stake.is_initialized {
            return Err(StakingError::AlreadyInitialized.into());
        }

        if lock_duration < pool.min_stake_duration {
            return Err(StakingError::InvalidLockDuration.into());
        }

        // Transfer tokens from owner to vault
        let transfer_ix = spl_token::instruction::transfer(
            token_program.key,
            owner_token.key,
            pool_token_vault.key,
            owner.key,
            &[],
            amount,
        )?;

        invoke(
            &transfer_ix,
            &[
                owner_token.clone(),
                pool_token_vault.clone(),
                owner.clone(),
                token_program.clone(),
            ],
        )?;

        // Initialize stake account
        stake.is_initialized = true;
        stake.owner = *owner.key;
        stake.pool = *pool_account.key;
        stake.amount = amount;
        stake.start_time = clock.unix_timestamp;
        stake.lock_duration = lock_duration;
        stake.last_reward_time = clock.unix_timestamp;
        stake.rewards_earned = 0;
        stake.is_locked = true;

        pool.total_staked = pool.total_staked.checked_add(amount)
            .ok_or(StakingError::MathOverflow)?;
        pool.stake_count = pool.stake_count.checked_add(1)
            .ok_or(StakingError::MathOverflow)?;

        StakeAccount::pack(stake, &mut stake_account.data.borrow_mut())?;
        StakePool::pack(pool, &mut pool_account.data.borrow_mut())?;

        Ok(())
    }

    // TODO: Implement process_unstake, process_claim_rewards, and process_start_distribution
} 