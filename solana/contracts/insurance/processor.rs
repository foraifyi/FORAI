use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};

use crate::{
    state::{InsurancePool, Stake, Claim},
    error::InsuranceError,
    instruction::InsuranceInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = InsuranceInstruction::unpack(instruction_data)?;

        match instruction {
            InsuranceInstruction::InitializePool { min_stake_amount, claim_delay_period, max_claim_amount } => {
                Self::process_initialize_pool(accounts, min_stake_amount, claim_delay_period, max_claim_amount)
            }
            InsuranceInstruction::Stake { amount, lock_period } => {
                Self::process_stake(accounts, amount, lock_period)
            }
            InsuranceInstruction::FileClaim { amount, evidence_uri } => {
                Self::process_file_claim(accounts, amount, evidence_uri)
            }
            InsuranceInstruction::ReviewClaim { approved } => {
                Self::process_review_claim(accounts, approved)
            }
            InsuranceInstruction::ProcessClaim => {
                Self::process_process_claim(accounts)
            }
        }
    }

    fn process_initialize_pool(
        accounts: &[AccountInfo],
        min_stake_amount: u64,
        claim_delay_period: i64,
        max_claim_amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = InsurancePool::unpack_unchecked(&pool_account.data.borrow())?;
        if pool.is_initialized {
            return Err(InsuranceError::AlreadyInitialized.into());
        }

        pool.is_initialized = true;
        pool.authority = *authority.key;
        pool.total_staked = 0;
        pool.total_claims_paid = 0;
        pool.min_stake_amount = min_stake_amount;
        pool.claim_delay_period = claim_delay_period;
        pool.max_claim_amount = max_claim_amount;
        pool.stake_count = 0;
        pool.claim_count = 0;

        InsurancePool::pack(pool, &mut pool_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_stake(
        accounts: &[AccountInfo],
        amount: u64,
        lock_period: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let stake_account = next_account_info(account_info_iter)?;
        let staker = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !staker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = InsurancePool::unpack(&pool_account.data.borrow())?;
        if amount < pool.min_stake_amount {
            return Err(InsuranceError::StakeBelowMinimum.into());
        }

        // Check staker has enough funds
        if staker.lamports() < amount {
            return Err(InsuranceError::InsufficientPoolFunds.into());
        }

        // Create stake record
        let mut stake = Stake::unpack_unchecked(&stake_account.data.borrow())?;
        if stake.is_initialized {
            return Err(InsuranceError::AlreadyInitialized.into());
        }

        // Transfer funds
        **staker.lamports.borrow_mut() -= amount;
        **pool_account.lamports.borrow_mut() += amount;

        // Update pool state
        pool.total_staked += amount;
        pool.stake_count += 1;
        InsurancePool::pack(pool, &mut pool_account.data.borrow_mut())?;

        // Initialize stake record
        stake.is_initialized = true;
        stake.staker = *staker.key;
        stake.amount = amount;
        stake.timestamp = clock.unix_timestamp;
        stake.locked_until = clock.unix_timestamp + lock_period;
        stake.rewards_claimed = 0;
        Stake::pack(stake, &mut stake_account.data.borrow_mut())?;

        Ok(())
    }

    // TODO: Implement process_file_claim, process_review_claim, and process_process_claim
} 