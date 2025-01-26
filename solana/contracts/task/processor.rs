use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};

use crate::{
    state::{TaskPool, Task, TaskStatus},
    error::TaskError,
    instruction::TaskInstruction,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = TaskInstruction::unpack(instruction_data)?;

        match instruction {
            TaskInstruction::InitializeTaskPool { 
                min_stake_required,
                min_performance_score,
                task_timeout,
                reward_amount,
                penalty_amount,
            } => {
                Self::process_initialize_pool(
                    accounts,
                    min_stake_required,
                    min_performance_score,
                    task_timeout,
                    reward_amount,
                    penalty_amount,
                )
            }
            TaskInstruction::CreateTask { 
                priority,
                reward_multiplier,
                specification_uri,
            } => {
                Self::process_create_task(
                    accounts,
                    priority,
                    reward_multiplier,
                    specification_uri,
                )
            }
            TaskInstruction::AcceptTask => {
                Self::process_accept_task(accounts)
            }
            TaskInstruction::SubmitResult { result_uri } => {
                Self::process_submit_result(accounts, result_uri)
            }
            TaskInstruction::ReviewTask { approved, feedback } => {
                Self::process_review_task(accounts, approved, feedback)
            }
        }
    }

    fn process_initialize_pool(
        accounts: &[AccountInfo],
        min_stake_required: u64,
        min_performance_score: u32,
        task_timeout: i64,
        reward_amount: u64,
        penalty_amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let authority = next_account_info(account_info_iter)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = TaskPool::unpack_unchecked(&pool_account.data.borrow())?;
        if pool.is_initialized {
            return Err(TaskError::AlreadyInitialized.into());
        }

        if min_performance_score > 10000 {
            return Err(TaskError::InvalidPerformanceScore.into());
        }

        pool.is_initialized = true;
        pool.authority = *authority.key;
        pool.total_tasks = 0;
        pool.min_stake_required = min_stake_required;
        pool.min_performance_score = min_performance_score;
        pool.task_timeout = task_timeout;
        pool.reward_amount = reward_amount;
        pool.penalty_amount = penalty_amount;

        TaskPool::pack(pool, &mut pool_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_create_task(
        accounts: &[AccountInfo],
        priority: u8,
        reward_multiplier: u16,
        specification_uri: [u8; 128],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let pool_account = next_account_info(account_info_iter)?;
        let task_account = next_account_info(account_info_iter)?;
        let creator = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        if !creator.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut pool = TaskPool::unpack(&pool_account.data.borrow())?;
        let mut task = Task::unpack_unchecked(&task_account.data.borrow())?;

        if task.is_initialized {
            return Err(TaskError::AlreadyInitialized.into());
        }

        if reward_multiplier < 100 {
            return Err(TaskError::InvalidRewardMultiplier.into());
        }

        task.is_initialized = true;
        task.creator = *creator.key;
        task.assigned_agent = None;
        task.status = TaskStatus::Open;
        task.priority = priority;
        task.reward_multiplier = reward_multiplier;
        task.created_at = clock.unix_timestamp;
        task.assigned_at = None;
        task.completed_at = None;
        task.specification_uri = specification_uri;
        task.result_uri = None;

        pool.total_tasks += 1;

        Task::pack(task, &mut task_account.data.borrow_mut())?;
        TaskPool::pack(pool, &mut pool_account.data.borrow_mut())?;

        Ok(())
    }

    // TODO: Implement process_accept_task, process_submit_result, and process_review_task
} 