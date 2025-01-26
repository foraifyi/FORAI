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
    state::{DistributionConfig, Recipient, DistributionRound},
    error::DistributionError,
    instruction::DistributionInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DistributionInstruction::unpack(instruction_data)?;

        match instruction {
            DistributionInstruction::InitializeConfig { 
                distribution_rate,
                min_epoch_duration,
            } => {
                Self::process_initialize_config(
                    accounts,
                    distribution_rate,
                    min_epoch_duration,
                )
            }
            DistributionInstruction::AddRecipient { allocation } => {
                Self::process_add_recipient(accounts, allocation)
            }
            DistributionInstruction::StartRound { amount, duration } => {
                Self::process_start_round(accounts, amount, duration)
            }
            DistributionInstruction::ClaimDistribution => {
                Self::process_claim_distribution(accounts)
            }
            DistributionInstruction::FinalizeRound => {
                Self::process_finalize_round(accounts)
            }
        }
    }

    fn process_initialize_config(
        accounts: &[AccountInfo],
        distribution_rate: u64,
        min_epoch_duration: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;
        let token_vault = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut config = DistributionConfig::unpack_unchecked(&config_account.data.borrow())?;
        if config.is_initialized {
            return Err(DistributionError::AlreadyInitialized.into());
        }

        if distribution_rate == 0 || min_epoch_duration == 0 {
            return Err(DistributionError::InvalidDistributionRate.into());
        }

        config.is_initialized = true;
        config.authority = *authority.key;
        config.token_mint = *token_mint.key;
        config.token_vault = *token_vault.key;
        config.total_distributed = 0;
        config.distribution_rate = distribution_rate;
        config.min_epoch_duration = min_epoch_duration;
        config.last_distribution = clock.unix_timestamp;
        config.is_active = true;

        DistributionConfig::pack(config, &mut config_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_add_recipient(
        accounts: &[AccountInfo],
        allocation: u16,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let recipient_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let owner_token = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let config = DistributionConfig::unpack(&config_account.data.borrow())?;
        if !config.is_active {
            return Err(DistributionError::DistributionNotActive.into());
        }

        if allocation == 0 || allocation > 10000 {
            return Err(DistributionError::InvalidAllocation.into());
        }

        let mut recipient = Recipient::unpack_unchecked(&recipient_account.data.borrow())?;
        if recipient.is_initialized {
            return Err(DistributionError::AlreadyInitialized.into());
        }

        recipient.is_initialized = true;
        recipient.owner = *owner.key;
        recipient.token_account = *owner_token.key;
        recipient.allocation = allocation;
        recipient.total_received = 0;
        recipient.last_claim = clock.unix_timestamp;
        recipient.is_active = true;

        Recipient::pack(recipient, &mut recipient_account.data.borrow_mut())?;
        Ok(())
    }

    // TODO: Implement process_start_round, process_claim_distribution, and process_finalize_round
} 