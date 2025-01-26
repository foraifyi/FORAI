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
    state::{LockConfig, TokenLock, UnlockRequest},
    error::TokenLockError,
    instruction::TokenLockInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TokenLockInstruction::unpack(instruction_data)?;

        match instruction {
            TokenLockInstruction::InitializeConfig { 
                min_lock_duration,
                max_lock_duration,
                unlock_fee,
                early_unlock_penalty,
            } => {
                Self::process_initialize_config(
                    accounts,
                    min_lock_duration,
                    max_lock_duration,
                    unlock_fee,
                    early_unlock_penalty,
                )
            }
            TokenLockInstruction::LockTokens { amount, duration } => {
                Self::process_lock_tokens(accounts, amount, duration)
            }
            TokenLockInstruction::RequestUnlock { amount, is_early } => {
                Self::process_request_unlock(accounts, amount, is_early)
            }
            TokenLockInstruction::ProcessUnlock => {
                Self::process_process_unlock(accounts)
            }
            TokenLockInstruction::CancelUnlock => {
                Self::process_cancel_unlock(accounts)
            }
        }
    }

    fn process_initialize_config(
        accounts: &[AccountInfo],
        min_lock_duration: i64,
        max_lock_duration: i64,
        unlock_fee: u16,
        early_unlock_penalty: u16,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut config = LockConfig::unpack_unchecked(&config_account.data.borrow())?;
        if config.is_initialized {
            return Err(TokenLockError::AlreadyInitialized.into());
        }

        if min_lock_duration <= 0 || max_lock_duration <= min_lock_duration {
            return Err(TokenLockError::InvalidLockAmount.into());
        }

        if unlock_fee > 10000 || early_unlock_penalty > 10000 {
            return Err(TokenLockError::InvalidFeeCalculation.into());
        }

        config.is_initialized = true;
        config.authority = *authority.key;
        config.token_mint = *token_mint.key;
        config.total_locked = 0;
        config.min_lock_duration = min_lock_duration;
        config.max_lock_duration = max_lock_duration;
        config.unlock_fee = unlock_fee;
        config.early_unlock_penalty = early_unlock_penalty;
        config.lock_count = 0;

        LockConfig::pack(config, &mut config_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_lock_tokens(
        accounts: &[AccountInfo],
        amount: u64,
        duration: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let lock_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let owner_token = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let config = LockConfig::unpack(&config_account.data.borrow())?;
        if duration < config.min_lock_duration || duration > config.max_lock_duration {
            return Err(TokenLockError::InvalidLockAmount.into());
        }

        let mut lock = TokenLock::unpack_unchecked(&lock_account.data.borrow())?;
        if lock.is_initialized {
            return Err(TokenLockError::AlreadyInitialized.into());
        }

        lock.is_initialized = true;
        lock.owner = *owner.key;
        lock.token_account = *owner_token.key;
        lock.amount = amount;
        lock.start_time = clock.unix_timestamp;
        lock.end_time = clock.unix_timestamp.checked_add(duration)
            .ok_or(TokenLockError::MathOverflow)?;
        lock.unlock_available = lock.end_time;
        lock.is_early_unlock = false;
        lock.is_active = true;

        TokenLock::pack(lock, &mut lock_account.data.borrow_mut())?;
        Ok(())
    }

    // TODO: Implement process_request_unlock, process_process_unlock, and process_cancel_unlock
} 