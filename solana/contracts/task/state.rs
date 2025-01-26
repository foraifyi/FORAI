use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct TaskPool {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub total_tasks: u32,
    pub min_stake_required: u64,
    pub min_performance_score: u32,  // Minimum score required (0-10000)
    pub task_timeout: i64,           // Time before task can be reassigned
    pub reward_amount: u64,          // Base reward for task completion
    pub penalty_amount: u64,         // Penalty for task failure
}

#[derive(Debug)]
pub struct Task {
    pub is_initialized: bool,
    pub creator: Pubkey,
    pub assigned_agent: Option<Pubkey>,
    pub status: TaskStatus,
    pub priority: u8,                // 0-255, higher is more important
    pub reward_multiplier: u16,      // Basis points (100 = 1x, 200 = 2x, etc)
    pub created_at: i64,
    pub assigned_at: Option<i64>,
    pub completed_at: Option<i64>,
    pub specification_uri: [u8; 128], // IPFS URI for task details
    pub result_uri: Option<[u8; 128]>, // IPFS URI for task result
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Open,
    Assigned,
    Completed,
    Failed,
    Disputed,
    Cancelled,
}

impl Sealed for TaskPool {}
impl IsInitialized for TaskPool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for TaskPool {
    const LEN: usize = 61; // 1 + 32 + 4 + 8 + 4 + 8 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, TaskPool::LEN];
        let (
            is_initialized,
            authority,
            total_tasks,
            min_stake_required,
            min_performance_score,
            task_timeout,
            reward_amount,
            penalty_amount,
        ) = array_refs![src, 1, 32, 4, 8, 4, 8, 8, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(TaskPool {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            total_tasks: u32::from_le_bytes(*total_tasks),
            min_stake_required: u64::from_le_bytes(*min_stake_required),
            min_performance_score: u32::from_le_bytes(*min_performance_score),
            task_timeout: i64::from_le_bytes(*task_timeout),
            reward_amount: u64::from_le_bytes(*reward_amount),
            penalty_amount: u64::from_le_bytes(*penalty_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, TaskPool::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            total_tasks_dst,
            min_stake_required_dst,
            min_performance_score_dst,
            task_timeout_dst,
            reward_amount_dst,
            penalty_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 4, 8, 4, 8, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        *total_tasks_dst = self.total_tasks.to_le_bytes();
        *min_stake_required_dst = self.min_stake_required.to_le_bytes();
        *min_performance_score_dst = self.min_performance_score.to_le_bytes();
        *task_timeout_dst = self.task_timeout.to_le_bytes();
        *reward_amount_dst = self.reward_amount.to_le_bytes();
        *penalty_amount_dst = self.penalty_amount.to_le_bytes();
    }
}

impl Sealed for Task {}
impl IsInitialized for Task {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl TaskStatus {
    fn to_u8(&self) -> u8 {
        match self {
            TaskStatus::Open => 0,
            TaskStatus::Assigned => 1,
            TaskStatus::Completed => 2,
            TaskStatus::Failed => 3,
            TaskStatus::Disputed => 4,
            TaskStatus::Cancelled => 5,
        }
    }

    fn from_u8(value: u8) -> Result<Self, ProgramError> {
        match value {
            0 => Ok(TaskStatus::Open),
            1 => Ok(TaskStatus::Assigned),
            2 => Ok(TaskStatus::Completed),
            3 => Ok(TaskStatus::Failed),
            4 => Ok(TaskStatus::Disputed),
            5 => Ok(TaskStatus::Cancelled),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl Pack for Task {
    const LEN: usize = 326; // 1 + 32 + 33 + 1 + 1 + 2 + 8 + 9 + 9 + 128 + 129

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Task::LEN];
        let (
            is_initialized,
            creator,
            assigned_agent_opt,
            status,
            priority,
            reward_multiplier,
            created_at,
            assigned_at_opt,
            completed_at_opt,
            specification_uri,
            result_uri_opt,
        ) = array_refs![src, 1, 32, 33, 1, 1, 2, 8, 9, 9, 128, 129];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let assigned_agent = if assigned_agent_opt[0] == 1 {
            Some(Pubkey::new_from_array(*array_ref![assigned_agent_opt, 1, 32]))
        } else {
            None
        };

        let assigned_at = if assigned_at_opt[0] == 1 {
            Some(i64::from_le_bytes(*array_ref![assigned_at_opt, 1, 8]))
        } else {
            None
        };

        let completed_at = if completed_at_opt[0] == 1 {
            Some(i64::from_le_bytes(*array_ref![completed_at_opt, 1, 8]))
        } else {
            None
        };

        let result_uri = if result_uri_opt[0] == 1 {
            Some(*array_ref![result_uri_opt, 1, 128])
        } else {
            None
        };

        Ok(Task {
            is_initialized,
            creator: Pubkey::new_from_array(*creator),
            assigned_agent,
            status: TaskStatus::from_u8(status[0])?,
            priority: priority[0],
            reward_multiplier: u16::from_le_bytes(*reward_multiplier),
            created_at: i64::from_le_bytes(*created_at),
            assigned_at,
            completed_at,
            specification_uri: *specification_uri,
            result_uri,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Task::LEN];
        let (
            is_initialized_dst,
            creator_dst,
            assigned_agent_opt_dst,
            status_dst,
            priority_dst,
            reward_multiplier_dst,
            created_at_dst,
            assigned_at_opt_dst,
            completed_at_opt_dst,
            specification_uri_dst,
            result_uri_opt_dst,
        ) = mut_array_refs![dst, 1, 32, 33, 1, 1, 2, 8, 9, 9, 128, 129];

        is_initialized_dst[0] = self.is_initialized as u8;
        creator_dst.copy_from_slice(self.creator.as_ref());

        if let Some(agent) = self.assigned_agent {
            assigned_agent_opt_dst[0] = 1;
            assigned_agent_opt_dst[1..].copy_from_slice(agent.as_ref());
        } else {
            assigned_agent_opt_dst[0] = 0;
        }

        status_dst[0] = self.status.to_u8();
        priority_dst[0] = self.priority;
        *reward_multiplier_dst = self.reward_multiplier.to_le_bytes();
        *created_at_dst = self.created_at.to_le_bytes();

        if let Some(time) = self.assigned_at {
            assigned_at_opt_dst[0] = 1;
            assigned_at_opt_dst[1..].copy_from_slice(&time.to_le_bytes());
        } else {
            assigned_at_opt_dst[0] = 0;
        }

        if let Some(time) = self.completed_at {
            completed_at_opt_dst[0] = 1;
            completed_at_opt_dst[1..].copy_from_slice(&time.to_le_bytes());
        } else {
            completed_at_opt_dst[0] = 0;
        }

        specification_uri_dst.copy_from_slice(&self.specification_uri);

        if let Some(uri) = &self.result_uri {
            result_uri_opt_dst[0] = 1;
            result_uri_opt_dst[1..].copy_from_slice(uri);
        } else {
            result_uri_opt_dst[0] = 0;
        }
    }
} 