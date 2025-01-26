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
    state::{TokenConfig, MintRequest, TransferLimit},
    error::GovernanceTokenError,
    instruction::GovernanceTokenInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = GovernanceTokenInstruction::unpack(instruction_data)?;

        match instruction {
            GovernanceTokenInstruction::InitializeConfig { 
                decimals,
                initial_supply,
            } => {
                Self::process_initialize_config(
                    accounts,
                    decimals,
                    initial_supply,
                )
            }
            GovernanceTokenInstruction::RequestMint { 
                amount,
                expiry_time,
            } => {
                Self::process_request_mint(
                    accounts,
                    amount,
                    expiry_time,
                )
            }
            GovernanceTokenInstruction::ExecuteMint => {
                Self::process_execute_mint(accounts)
            }
            GovernanceTokenInstruction::SetTransferLimit { 
                daily_limit,
                is_exempt,
            } => {
                Self::process_set_transfer_limit(
                    accounts,
                    daily_limit,
                    is_exempt,
                )
            }
            GovernanceTokenInstruction::UpdateTransferStatus { enable_transfers } => {
                Self::process_update_transfer_status(accounts, enable_transfers)
            }
        }
    }

    fn process_initialize_config(
        accounts: &[AccountInfo],
        decimals: u8,
        initial_supply: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut config = TokenConfig::unpack_unchecked(&config_account.data.borrow())?;
        if config.is_initialized {
            return Err(GovernanceTokenError::AlreadyInitialized.into());
        }

        if decimals > 9 {
            return Err(GovernanceTokenError::InvalidDecimals.into());
        }

        config.is_initialized = true;
        config.authority = *authority.key;
        config.token_mint = *token_mint.key;
        config.treasury = *treasury.key;
        config.total_supply = initial_supply;
        config.circulating_supply = 0;
        config.decimals = decimals;
        config.transfer_enabled = false;
        config.mint_enabled = true;

        TokenConfig::pack(config, &mut config_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_request_mint(
        accounts: &[AccountInfo],
        amount: u64,
        expiry_time: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let mint_request_account = next_account_info(account_info_iter)?;
        let requester = next_account_info(account_info_iter)?;
        let recipient_token = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !requester.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let config = TokenConfig::unpack(&config_account.data.borrow())?;
        if !config.mint_enabled {
            return Err(GovernanceTokenError::MintNotEnabled.into());
        }

        let mut request = MintRequest::unpack_unchecked(&mint_request_account.data.borrow())?;
        if request.is_initialized {
            return Err(GovernanceTokenError::AlreadyInitialized.into());
        }

        if expiry_time <= clock.unix_timestamp {
            return Err(GovernanceTokenError::InvalidExpiryTime.into());
        }

        request.is_initialized = true;
        request.requester = *requester.key;
        request.recipient = *recipient_token.key;
        request.amount = amount;
        request.request_time = clock.unix_timestamp;
        request.expiry_time = expiry_time;
        request.is_approved = false;
        request.is_executed = false;

        MintRequest::pack(request, &mut mint_request_account.data.borrow_mut())?;
        Ok(())
    }

    // TODO: Implement process_execute_mint, process_set_transfer_limit, and process_update_transfer_status
} 