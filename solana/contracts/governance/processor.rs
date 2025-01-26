use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
    program::{invoke, invoke_signed},
};

use crate::{
    state::{GovernanceConfig, Action, Queue},
    error::GovernanceError,
    instruction::GovernanceInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = GovernanceInstruction::unpack(instruction_data)?;

        match instruction {
            GovernanceInstruction::InitializeConfig { 
                voting_delay,
                voting_period,
                quorum_votes,
                timelock_delay,
                guardian,
            } => {
                Self::process_initialize_config(
                    accounts,
                    voting_delay,
                    voting_period,
                    quorum_votes,
                    timelock_delay,
                    guardian,
                )
            }
            GovernanceInstruction::QueueActions { 
                execution_time,
                actions,
            } => {
                Self::process_queue_actions(
                    accounts,
                    execution_time,
                    actions,
                )
            }
            GovernanceInstruction::ExecuteActions => {
                Self::process_execute_actions(accounts)
            }
            GovernanceInstruction::CancelActions => {
                Self::process_cancel_actions(accounts)
            }
            GovernanceInstruction::SetGuardian { new_guardian } => {
                Self::process_set_guardian(accounts, new_guardian)
            }
        }
    }

    fn process_initialize_config(
        accounts: &[AccountInfo],
        voting_delay: i64,
        voting_period: i64,
        quorum_votes: u64,
        timelock_delay: i64,
        guardian: Option<Pubkey>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let governance_token = next_account_info(account_info_iter)?;
        let proposal_program = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut config = GovernanceConfig::unpack_unchecked(&config_account.data.borrow())?;
        if config.is_initialized {
            return Err(GovernanceError::AlreadyInitialized.into());
        }

        if voting_delay < 0 || voting_period < 0 || timelock_delay < 0 {
            return Err(GovernanceError::InvalidTimelockDelay.into());
        }

        config.is_initialized = true;
        config.authority = *authority.key;
        config.governance_token = *governance_token.key;
        config.proposal_program = *proposal_program.key;
        config.voting_delay = voting_delay;
        config.voting_period = voting_period;
        config.quorum_votes = quorum_votes;
        config.timelock_delay = timelock_delay;
        config.guardian = guardian;
        config.is_active = true;

        GovernanceConfig::pack(config, &mut config_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_queue_actions(
        accounts: &[AccountInfo],
        execution_time: i64,
        actions: Vec<Pubkey>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let queue_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let proposal_account = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let config = GovernanceConfig::unpack(&config_account.data.borrow())?;
        if !config.is_active {
            return Err(GovernanceError::GovernanceNotActive.into());
        }

        let mut queue = Queue::unpack_unchecked(&queue_account.data.borrow())?;
        if queue.is_initialized {
            return Err(GovernanceError::AlreadyInitialized.into());
        }

        if execution_time <= clock.unix_timestamp + config.timelock_delay {
            return Err(GovernanceError::InvalidExecutionTime.into());
        }

        if actions.len() > 10 {
            return Err(GovernanceError::TooManyActions.into());
        }

        queue.is_initialized = true;
        queue.proposal = *proposal_account.key;
        queue.execution_time = execution_time;
        queue.actions = actions;
        queue.executed = false;
        queue.canceled = false;

        Queue::pack(queue, &mut queue_account.data.borrow_mut())?;
        Ok(())
    }

    // TODO: Implement process_execute_actions, process_cancel_actions, and process_set_guardian
} 