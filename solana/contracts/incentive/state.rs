use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct AgentAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub authority: Pubkey,          // Admin authority
    pub reputation_score: u64,      // 0-10000
    pub total_rewards: u64,
    pub completed_tasks: u32,
    pub failed_tasks: u32,          // Added: Number of failed tasks
    pub total_tasks: u32,           // Added: Total number of tasks
    pub level: u8,                  // Added: Agent level (1-5)
    pub last_task_time: i64,        // Added: Last task time
    pub performance_multiplier: u16, // Added: Performance multiplier (basis points)
    pub consecutive_successes: u32,  // Added: Number of consecutive successes
}

// Added: Reputation history records
#[derive(Debug)]
pub struct ReputationHistory {
    pub is_initialized: bool,
    pub agent: Pubkey,
    pub timestamp: i64,
    pub old_score: u64,
    pub new_score: u64,
    pub reason: [u8; 32],          // Change reason
    pub authority: Pubkey,
}

impl Sealed for AgentAccount {}

impl IsInitialized for AgentAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for AgentAccount {
    const LEN: usize = 105; // 1 + 32 + 32 + 8 + 8 + 4 + 4 + 4 + 1 + 8 + 2 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, AgentAccount::LEN];
        let (
            is_initialized,
            owner,
            authority,
            reputation_score,
            total_rewards,
            completed_tasks,
            failed_tasks,
            total_tasks,
            level,
            last_task_time,
            performance_multiplier,
            consecutive_successes,
        ) = array_refs![src, 1, 32, 32, 8, 8, 4, 4, 4, 1, 8, 2, 4];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(AgentAccount {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            authority: Pubkey::new_from_array(*authority),
            reputation_score: u64::from_le_bytes(*reputation_score),
            total_rewards: u64::from_le_bytes(*total_rewards),
            completed_tasks: u32::from_le_bytes(*completed_tasks),
            failed_tasks: u32::from_le_bytes(*failed_tasks),
            total_tasks: u32::from_le_bytes(*total_tasks),
            level: level[0],
            last_task_time: i64::from_le_bytes(*last_task_time),
            performance_multiplier: u16::from_le_bytes(*performance_multiplier),
            consecutive_successes: u32::from_le_bytes(*consecutive_successes),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, AgentAccount::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            authority_dst,
            reputation_score_dst,
            total_rewards_dst,
            completed_tasks_dst,
            failed_tasks_dst,
            total_tasks_dst,
            level_dst,
            last_task_time_dst,
            performance_multiplier_dst,
            consecutive_successes_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 4, 4, 4, 1, 8, 2, 4];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        authority_dst.copy_from_slice(self.authority.as_ref());
        *reputation_score_dst = self.reputation_score.to_le_bytes();
        *total_rewards_dst = self.total_rewards.to_le_bytes();
        *completed_tasks_dst = self.completed_tasks.to_le_bytes();
        *failed_tasks_dst = self.failed_tasks.to_le_bytes();
        *total_tasks_dst = self.total_tasks.to_le_bytes();
        level_dst[0] = self.level;
        *last_task_time_dst = self.last_task_time.to_le_bytes();
        *performance_multiplier_dst = self.performance_multiplier.to_le_bytes();
        *consecutive_successes_dst = self.consecutive_successes.to_le_bytes();
    }
}

impl Sealed for ReputationHistory {}

impl IsInitialized for ReputationHistory {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ReputationHistory {
    const LEN: usize = 118; // 1 + 32 + 8 + 8 + 8 + 32 + 32

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ReputationHistory::LEN];
        let (
            is_initialized,
            agent,
            timestamp,
            old_score,
            new_score,
            reason,
            authority,
        ) = array_refs![src, 1, 32, 8, 8, 8, 32, 32];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(ReputationHistory {
            is_initialized,
            agent: Pubkey::new_from_array(*agent),
            timestamp: i64::from_le_bytes(*timestamp),
            old_score: u64::from_le_bytes(*old_score),
            new_score: u64::from_le_bytes(*new_score),
            reason: *reason,
            authority: Pubkey::new_from_array(*authority),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ReputationHistory::LEN];
        let (
            is_initialized_dst,
            agent_dst,
            timestamp_dst,
            old_score_dst,
            new_score_dst,
            reason_dst,
            authority_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 8, 32, 32];

        is_initialized_dst[0] = self.is_initialized as u8;
        agent_dst.copy_from_slice(self.agent.as_ref());
        *timestamp_dst = self.timestamp.to_le_bytes();
        *old_score_dst = self.old_score.to_le_bytes();
        *new_score_dst = self.new_score.to_le_bytes();
        reason_dst.copy_from_slice(&self.reason);
        authority_dst.copy_from_slice(self.authority.as_ref());
    }
} 