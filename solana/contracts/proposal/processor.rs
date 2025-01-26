use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};

use crate::{
    state::{ProposalConfig, Proposal, Vote},
    error::ProposalError,
    instruction::ProposalInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ProposalInstruction::unpack(instruction_data)?;

        match instruction {
            ProposalInstruction::InitializeConfig { 
                min_voting_power,
                voting_delay,
                voting_period,
                quorum_votes,
            } => {
                Self::process_initialize_config(
                    accounts,
                    min_voting_power,
                    voting_delay,
                    voting_period,
                    quorum_votes,
                )
            }
            ProposalInstruction::CreateProposal { 
                title,
                description_url,
                execution_time,
            } => {
                Self::process_create_proposal(
                    accounts,
                    title,
                    description_url,
                    execution_time,
                )
            }
            ProposalInstruction::CastVote { support } => {
                Self::process_cast_vote(accounts, support)
            }
            ProposalInstruction::CancelProposal => {
                Self::process_cancel_proposal(accounts)
            }
            ProposalInstruction::ExecuteProposal => {
                Self::process_execute_proposal(accounts)
            }
        }
    }

    fn process_initialize_config(
        accounts: &[AccountInfo],
        min_voting_power: u64,
        voting_delay: i64,
        voting_period: i64,
        quorum_votes: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let voting_power_program = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut config = ProposalConfig::unpack_unchecked(&config_account.data.borrow())?;
        if config.is_initialized {
            return Err(ProposalError::AlreadyInitialized.into());
        }

        if voting_delay < 0 || voting_period <= 0 {
            return Err(ProposalError::InvalidVotingPeriod.into());
        }

        config.is_initialized = true;
        config.authority = *authority.key;
        config.voting_power_program = *voting_power_program.key;
        config.min_voting_power = min_voting_power;
        config.voting_delay = voting_delay;
        config.voting_period = voting_period;
        config.quorum_votes = quorum_votes;
        config.proposal_count = 0;

        ProposalConfig::pack(config, &mut config_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_create_proposal(
        accounts: &[AccountInfo],
        title: [u8; 32],
        description_url: [u8; 64],
        execution_time: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let config_account = next_account_info(account_info_iter)?;
        let proposal_account = next_account_info(account_info_iter)?;
        let proposer = next_account_info(account_info_iter)?;
        let voting_power_account = next_account_info(account_info_iter)?;
        let voting_checkpoint = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !proposer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let config = ProposalConfig::unpack(&config_account.data.borrow())?;
        let mut proposal = Proposal::unpack_unchecked(&proposal_account.data.borrow())?;

        if proposal.is_initialized {
            return Err(ProposalError::AlreadyInitialized.into());
        }

        // TODO: Verify proposer's voting power meets minimum requirement

        let voting_starts = clock.unix_timestamp.checked_add(config.voting_delay)
            .ok_or(ProposalError::MathOverflow)?;
        let voting_ends = voting_starts.checked_add(config.voting_period)
            .ok_or(ProposalError::MathOverflow)?;

        if execution_time <= voting_ends {
            return Err(ProposalError::InvalidExecutionTime.into());
        }

        proposal.is_initialized = true;
        proposal.proposer = *proposer.key;
        proposal.title = title;
        proposal.description_url = description_url;
        proposal.voting_starts = voting_starts;
        proposal.voting_ends = voting_ends;
        proposal.execution_time = execution_time;
        proposal.for_votes = 0;
        proposal.against_votes = 0;
        proposal.abstain_votes = 0;
        proposal.canceled = false;
        proposal.executed = false;
        proposal.voting_checkpoint = *voting_checkpoint.key;

        Proposal::pack(proposal, &mut proposal_account.data.borrow_mut())?;
        Ok(())
    }

    // TODO: Implement process_cast_vote, process_cancel_proposal, and process_execute_proposal
} 