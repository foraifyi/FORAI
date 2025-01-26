use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};

use crate::{
    state::{AgentRanking, Agent, Performance},
    error::RankingError,
    instruction::RankingInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = RankingInstruction::unpack(instruction_data)?;

        match instruction {
            RankingInstruction::InitializeRanking { 
                min_stake_amount, 
                performance_period, 
                reward_rate, 
                penalty_rate 
            } => {
                Self::process_initialize_ranking(
                    accounts,
                    min_stake_amount,
                    performance_period,
                    reward_rate,
                    penalty_rate,
                )
            }
            RankingInstruction::RegisterAgent { stake_amount } => {
                Self::process_register_agent(accounts, stake_amount)
            }
            RankingInstruction::SubmitPerformance { score, feedback_uri } => {
                Self::process_submit_performance(accounts, score, feedback_uri)
            }
            RankingInstruction::UpdateRanking { new_score } => {
                Self::process_update_ranking(accounts, new_score)
            }
            RankingInstruction::ClaimRewards => {
                Self::process_claim_rewards(accounts)
            }
        }
    }

    fn process_initialize_ranking(
        accounts: &[AccountInfo],
        min_stake_amount: u64,
        performance_period: i64,
        reward_rate: u16,
        penalty_rate: u16,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let ranking_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut ranking = AgentRanking::unpack_unchecked(&ranking_account.data.borrow())?;
        if ranking.is_initialized {
            return Err(RankingError::AlreadyInitialized.into());
        }

        if reward_rate > 10000 || penalty_rate > 10000 {
            return Err(RankingError::InvalidRewardCalculation.into());
        }

        ranking.is_initialized = true;
        ranking.authority = *authority.key;
        ranking.total_agents = 0;
        ranking.min_stake_amount = min_stake_amount;
        ranking.performance_period = performance_period;
        ranking.reward_rate = reward_rate;
        ranking.penalty_rate = penalty_rate;

        AgentRanking::pack(ranking, &mut ranking_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_register_agent(
        accounts: &[AccountInfo],
        stake_amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let ranking_account = next_account_info(account_info_iter)?;
        let agent_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut ranking = AgentRanking::unpack(&ranking_account.data.borrow())?;
        let mut agent = Agent::unpack_unchecked(&agent_account.data.borrow())?;

        if agent.is_initialized {
            return Err(RankingError::AgentAlreadyRegistered.into());
        }

        if stake_amount < ranking.min_stake_amount {
            return Err(RankingError::StakeBelowMinimum.into());
        }

        // Transfer stake amount
        **owner.lamports.borrow_mut() -= stake_amount;
        **agent_account.lamports.borrow_mut() += stake_amount;

        // Initialize agent
        agent.is_initialized = true;
        agent.owner = *owner.key;
        agent.stake_amount = stake_amount;
        agent.performance_score = 5000; // Start with neutral score (50%)
        agent.total_tasks = 0;
        agent.successful_tasks = 0;
        agent.rewards_earned = 0;
        agent.penalties_incurred = 0;
        agent.last_update = clock.unix_timestamp;
        agent.is_active = true;

        ranking.total_agents += 1;

        Agent::pack(agent, &mut agent_account.data.borrow_mut())?;
        AgentRanking::pack(ranking, &mut ranking_account.data.borrow_mut())?;

        Ok(())
    }

    // TODO: Implement process_submit_performance, process_update_ranking, and process_claim_rewards
} 