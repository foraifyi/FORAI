use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};

use crate::{
    state::{AgentAccount, ReputationHistory},
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
                Self::process_initialize_agent(accounts, program_id)
            }
            IncentiveInstruction::UpdateReputation { new_score } => {
                Self::process_update_reputation(accounts, new_score)
            }
            IncentiveInstruction::RewardAgent { amount } => {
                Self::process_reward_agent(accounts, amount)
            }
            IncentiveInstruction::PenalizeAgent { amount } => {
                Self::process_penalize_agent(accounts, amount)
            }
            IncentiveInstruction::GetReputationHistory { start_time, end_time } => {
                Self::process_get_reputation_history(accounts, start_time, end_time)
            }
            IncentiveInstruction::SetAgentAuthority { new_authority } => {
                Self::process_set_agent_authority(accounts, new_authority)
            }
            IncentiveInstruction::SuspendAgent { duration } => {
                Self::process_suspend_agent(accounts, duration)
            }
            IncentiveInstruction::ReactivateAgent => {
                Self::process_reactivate_agent(accounts)
            }
            IncentiveInstruction::BatchUpdateReputation { agents, scores } => {
                Self::process_batch_update_reputation(accounts, agents, scores)
            }
            IncentiveInstruction::SetPerformanceMultiplier { multiplier } => {
                Self::process_set_performance_multiplier(accounts, multiplier)
            }
            IncentiveInstruction::ClaimAccumulatedRewards => {
                Self::process_claim_accumulated_rewards(accounts)
            }
            IncentiveInstruction::RequestLevelUpgrade { target_level } => {
                Self::process_request_level_upgrade(accounts, target_level)
            }
        }
    }

    fn process_initialize_agent(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if agent_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut agent = AgentAccount::unpack_unchecked(&agent_account.data.borrow())?;
        if agent.is_initialized {
            return Err(IncentiveError::AlreadyInitialized.into());
        }

        agent.is_initialized = true;
        agent.owner = *owner.key;
        agent.authority = *authority.key;
        agent.reputation_score = 5000;  // Initial medium reputation
        agent.total_rewards = 0;
        agent.completed_tasks = 0;
        agent.failed_tasks = 0;
        agent.total_tasks = 0;
        agent.level = 1;  // Initial level
        agent.last_task_time = clock.unix_timestamp;
        agent.performance_multiplier = 100;  // Base multiplier (1x)
        agent.consecutive_successes = 0;

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_update_reputation(
        accounts: &[AccountInfo],
        new_score: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let history_account = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.authority != *authority.key {
            return Err(IncentiveError::InvalidAuthority.into());
        }

        // Validate score range
        if new_score > 10000 {
            return Err(IncentiveError::InvalidReputationScore.into());
        }

        // Record history
        let mut history = ReputationHistory::unpack_unchecked(&history_account.data.borrow())?;
        if history.is_initialized {
            return Err(IncentiveError::AlreadyInitialized.into());
        }

        history.is_initialized = true;
        history.agent = *agent_account.key;
        history.timestamp = clock.unix_timestamp;
        history.old_score = agent.reputation_score;
        history.new_score = new_score;
        history.authority = *authority.key;

        // Update agent level
        agent.level = match new_score {
            0..=2000 => 1,
            2001..=4000 => 2,
            4001..=6000 => 3,
            6001..=8000 => 4,
            _ => 5,
        };

        // Update performance multiplier
        agent.performance_multiplier = 100 + ((new_score as u16) / 100);
        agent.reputation_score = new_score;

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        ReputationHistory::pack(history, &mut history_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_reward_agent(
        accounts: &[AccountInfo],
        base_amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        
        // Calculate time bonus
        let time_since_last = clock.unix_timestamp - agent.last_task_time;
        let time_bonus = if time_since_last < 86400 { // Within 24 hours
            110 // 10% bonus
        } else {
            100
        };

        // Calculate consecutive success bonus
        let streak_bonus = 100 + (agent.consecutive_successes.min(10) * 5) as u16;

        // Calculate level bonus
        let level_bonus = 100 + ((agent.level as u16 - 1) * 20);

        // Calculate final reward
        let reputation_multiplier = (agent.reputation_score as f64) / 5000.0;
        let performance_multiplier = (agent.performance_multiplier as f64) / 100.0;
        let time_multiplier = (time_bonus as f64) / 100.0;
        let streak_multiplier = (streak_bonus as f64) / 100.0;
        let level_multiplier = (level_bonus as f64) / 100.0;

        let total_reward = (base_amount as f64 
            * reputation_multiplier 
            * performance_multiplier 
            * time_multiplier 
            * streak_multiplier 
            * level_multiplier) as u64;

        // Ensure reward doesn't exceed treasury balance
        if treasury.lamports() < total_reward {
            return Err(IncentiveError::InsufficientFunds.into());
        }

        // Transfer reward
        **treasury.try_borrow_mut_lamports()? -= total_reward;
        **agent_account.try_borrow_mut_lamports()? += total_reward;

        // Update agent status
        agent.total_rewards += total_reward;
        agent.completed_tasks += 1;
        agent.total_tasks += 1;
        agent.consecutive_successes += 1;
        agent.last_task_time = clock.unix_timestamp;

        // Adjust reputation based on completion rate
        let completion_rate = (agent.completed_tasks as f64 / agent.total_tasks as f64) * 100.0;
        if completion_rate >= 90.0 && agent.reputation_score < 9900 {
            agent.reputation_score += 100;
        }

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_penalize_agent(
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;

        // Calculate penalty amount
        let penalty_multiplier = (agent.reputation_score as f64) / 10000.0;
        let level_factor = agent.level as f64 * 0.2; // Higher level means heavier penalty
        let total_penalty = (amount as f64 * penalty_multiplier * (1.0 + level_factor)) as u64;

        if agent_account.lamports() < total_penalty {
            return Err(IncentiveError::InsufficientFunds.into());
        }

        // Transfer penalty amount
        **agent_account.try_borrow_mut_lamports()? -= total_penalty;
        **treasury.try_borrow_mut_lamports()? += total_penalty;

        // Update agent status
        agent.failed_tasks += 1;
        agent.total_tasks += 1;
        agent.consecutive_successes = 0;
        agent.last_task_time = clock.unix_timestamp;

        // Reduce reputation score
        let reputation_penalty = 1000 + ((agent.level as u64 - 1) * 200);
        if agent.reputation_score > reputation_penalty {
            agent.reputation_score -= reputation_penalty;
        } else {
            agent.reputation_score = 0;
        }

        // Possible downgrade
        if agent.level > 1 && agent.reputation_score < (agent.level as u64 - 1) * 2000 {
            agent.level -= 1;
        }

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_get_reputation_history(
        accounts: &[AccountInfo],
        start_time: i64,
        end_time: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Implementation of process_get_reputation_history
        Ok(())
    }

    fn process_set_agent_authority(
        accounts: &[AccountInfo],
        new_authority: Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let current_authority = next_account_info(account_info_iter)?;

        if !current_authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.authority != *current_authority.key {
            return Err(IncentiveError::InvalidAuthority.into());
        }

        agent.authority = new_authority;
        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_suspend_agent(
        accounts: &[AccountInfo],
        duration: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.authority != *authority.key {
            return Err(IncentiveError::InvalidAuthority.into());
        }

        // Set suspension status
        agent.is_active = false;
        agent.last_task_time = clock.unix_timestamp + duration;

        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_reactivate_agent(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.authority != *authority.key {
            return Err(IncentiveError::InvalidAuthority.into());
        }

        // Check if suspension period has ended
        if clock.unix_timestamp < agent.last_task_time {
            return Err(IncentiveError::SuspensionPeriodNotEnded.into());
        }

        agent.is_active = true;
        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_batch_update_reputation(
        accounts: &[AccountInfo],
        agents: Vec<Pubkey>,
        scores: Vec<u64>,
    ) -> ProgramResult {
        if agents.len() != scores.len() {
            return Err(IncentiveError::BatchUpdateMismatch.into());
        }

        let account_info_iter = &mut accounts.iter();
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        for (agent_pubkey, score) in agents.iter().zip(scores.iter()) {
            let agent_account = next_account_info(account_info_iter)?;
            if agent_account.key != agent_pubkey {
                return Err(IncentiveError::InvalidAgentAccount.into());
            }

            let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
            if !agent.is_active {
                continue;
            }

            // Update reputation score
            agent.reputation_score = *score;
            
            // Update level
            agent.level = match score {
                0..=2000 => 1,
                2001..=4000 => 2,
                4001..=6000 => 3,
                6001..=8000 => 4,
                _ => 5,
            };

            AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        }

        Ok(())
    }

    fn process_set_performance_multiplier(
        accounts: &[AccountInfo],
        multiplier: u16,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.authority != *authority.key {
            return Err(IncentiveError::InvalidAuthority.into());
        }

        // Validate multiplier range (50-500, i.e., 0.5x-5x)
        if multiplier < 50 || multiplier > 500 {
            return Err(IncentiveError::InvalidMultiplier.into());
        }

        agent.performance_multiplier = multiplier;
        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_claim_accumulated_rewards(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.owner != *owner.key {
            return Err(IncentiveError::InvalidOwner.into());
        }

        // Calculate claimable rewards
        let claimable_rewards = agent.total_rewards;
        if claimable_rewards == 0 {
            return Err(IncentiveError::NoRewardsTolaim.into());
        }

        // Transfer rewards
        **treasury.try_borrow_mut_lamports()? -= claimable_rewards;
        **owner.try_borrow_mut_lamports()? += claimable_rewards;

        // Reset reward counter
        agent.total_rewards = 0;
        AgentAccount::pack(agent, &mut agent_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_request_level_upgrade(
        accounts: &[AccountInfo],
        target_level: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let agent_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let agent = AgentAccount::unpack(&agent_account.data.borrow())?;
        if agent.owner != *owner.key {
            return Err(IncentiveError::InvalidOwner.into());
        }

        // Validate target level
        if target_level <= agent.level || target_level > 5 {
            return Err(IncentiveError::InvalidTargetLevel.into());
        }

        // Validate reputation requirements
        let required_reputation = match target_level {
            2 => 2000,
            3 => 4000,
            4 => 6000,
            5 => 8000,
            _ => return Err(IncentiveError::InvalidTargetLevel.into()),
        };

        if agent.reputation_score < required_reputation {
            return Err(IncentiveError::InsufficientReputation.into());
        }

        // Validate task completion requirements
        let required_tasks = (target_level as u32 - 1) * 10;
        if agent.completed_tasks < required_tasks {
            return Err(IncentiveError::InsufficientCompletedTasks.into());
        }

        Ok(())
    }
} 