use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};
use spl_token::state::Account as TokenAccount;

use crate::{
    state::{VotingPowerConfig, VotingPower, Checkpoint},
    error::VotingPowerError,
    instruction::VotingPowerInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = VotingPowerInstruction::unpack(instruction_data)?;

        match instruction {
            VotingPowerInstruction::InitializeConfig { 
                delegation_enabled,
                checkpoint_enabled,
                checkpoint_interval,
            } => {
                Self::process_initialize_config(
                    accounts,
                    delegation_enabled,
                    checkpoint_enabled,
                    checkpoint_interval,
                )
            }
            VotingPowerInstruction::UpdateVotingPower { amount } => {
                Self::process_update_voting_power(accounts, amount)
            }
            VotingPowerInstruction::DelegateVotingPower { amount } => {
                Self::process_delegate_voting_power(accounts, amount)
            }
            VotingPowerInstruction::CreateCheckpoint => {
                Self::process_create_checkpoint(accounts)
            }
            VotingPowerInstruction::GetVotingPowerAtCheckpoint => {
                Self::process_get_voting_power_at_checkpoint(accounts)
            }
        }
    }

    fn process_initialize_config(
        accounts: &[AccountInfo],
        delegation_enabled: bool,
        checkpoint_enabled: bool,
        checkpoint_interval: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let governance_token = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut config = VotingPowerConfig::unpack_unchecked(&config_account.data.borrow())?;
        if config.is_initialized {
            return Err(VotingPowerError::AlreadyInitialized.into());
        }

        if checkpoint_interval <= 0 {
            return Err(VotingPowerError::InvalidCheckpointInterval.into());
        }

        config.is_initialized = true;
        config.authority = *authority.key;
        config.governance_token = *governance_token.key;
        config.delegation_enabled = delegation_enabled;
        config.checkpoint_enabled = checkpoint_enabled;
        config.checkpoint_interval = checkpoint_interval;
        config.last_checkpoint = 0;

        VotingPowerConfig::pack(config, &mut config_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_update_voting_power(
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let voting_power_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let token_account = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let config = VotingPowerConfig::unpack(&config_account.data.borrow())?;
        let token_data = TokenAccount::unpack(&token_account.data.borrow())?;

        if token_data.owner != *owner.key {
            return Err(VotingPowerError::InvalidTokenOwner.into());
        }

        if token_data.mint != config.governance_token {
            return Err(VotingPowerError::InvalidGovernanceToken.into());
        }

        let mut voting_power = VotingPower::unpack_unchecked(&voting_power_account.data.borrow())?;
        if voting_power.is_initialized && voting_power.owner != *owner.key {
            return Err(VotingPowerError::InvalidOwner.into());
        }

        voting_power.is_initialized = true;
        voting_power.owner = *owner.key;
        voting_power.amount = amount;
        voting_power.last_update = clock.unix_timestamp;

        VotingPower::pack(voting_power, &mut voting_power_account.data.borrow_mut())?;
        Ok(())
    }

    // TODO: Implement process_delegate_voting_power, process_create_checkpoint, and process_get_voting_power_at_checkpoint
} 