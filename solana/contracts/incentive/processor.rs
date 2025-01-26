use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::invoke_signed,
};

use crate::{
    state::AgentAccount,
    error::IncentiveError,
    instruction::IncentiveInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = IncentiveInstruction::unpack(instruction_data)?;

        match instruction {
            IncentiveInstruction::InitializeAgent => {
                Self::process_initialize_agent(program_id, accounts)
            }
            IncentiveInstruction::RewardAgent { amount, reputation_increase } => {
                Self::process_reward_agent(program_id, accounts, amount, reputation_increase)
            }
        }
    }

    fn process_initialize_agent(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        if agent_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut agent = AgentAccount::unpack_unchecked(&agent_account.data.borrow())?;
        if agent.is_initialized {
            return Err(IncentiveError::AlreadyInitialized.into());
        }

        agent.is_initialized = true;
        agent.owner = *authority.key;
        agent.reputation_score = 0;
        agent.total_rewards = 0;
        agent.completed_tasks = 0;

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_reward_agent(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        reputation_increase: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(IncentiveError::InvalidProgramAuthority.into());
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        
        // Transfer rewards
        if treasury.lamports() < amount {
            return Err(IncentiveError::InsufficientFunds.into());
        }

        **treasury.lamports.borrow_mut() -= amount;
        **agent_account.lamports.borrow_mut() += amount;

        // Update agent stats
        agent.total_rewards += amount;
        agent.reputation_score += reputation_increase;
        agent.completed_tasks += 1;

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }
} 