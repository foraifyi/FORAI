use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct AgentRanking {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub total_agents: u32,
    pub min_stake_amount: u64,
    pub performance_period: i64,  // Period for performance calculation
    pub reward_rate: u16,        // Basis points (0-10000)
    pub penalty_rate: u16,       // Basis points (0-10000)
}

#[derive(Debug)]
pub struct Agent {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub stake_amount: u64,
    pub performance_score: u32,   // 0-10000
    pub total_tasks: u32,
    pub successful_tasks: u32,
    pub rewards_earned: u64,
    pub penalties_incurred: u64,
    pub last_update: i64,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct Performance {
    pub is_initialized: bool,
    pub agent: Pubkey,
    pub task_id: Pubkey,
    pub score: u32,              // 0-10000
    pub timestamp: i64,
    pub reviewer: Pubkey,
    pub feedback_uri: [u8; 128], // IPFS URI for detailed feedback
}

impl Sealed for AgentRanking {}
impl IsInitialized for AgentRanking {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for AgentRanking {
    const LEN: usize = 57; // 1 + 32 + 4 + 8 + 8 + 2 + 2

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, AgentRanking::LEN];
        let (
            is_initialized,
            authority,
            total_agents,
            min_stake_amount,
            performance_period,
            reward_rate,
            penalty_rate,
        ) = array_refs![src, 1, 32, 4, 8, 8, 2, 2];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(AgentRanking {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            total_agents: u32::from_le_bytes(*total_agents),
            min_stake_amount: u64::from_le_bytes(*min_stake_amount),
            performance_period: i64::from_le_bytes(*performance_period),
            reward_rate: u16::from_le_bytes(*reward_rate),
            penalty_rate: u16::from_le_bytes(*penalty_rate),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, AgentRanking::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            total_agents_dst,
            min_stake_amount_dst,
            performance_period_dst,
            reward_rate_dst,
            penalty_rate_dst,
        ) = mut_array_refs![dst, 1, 32, 4, 8, 8, 2, 2];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        *total_agents_dst = self.total_agents.to_le_bytes();
        *min_stake_amount_dst = self.min_stake_amount.to_le_bytes();
        *performance_period_dst = self.performance_period.to_le_bytes();
        *reward_rate_dst = self.reward_rate.to_le_bytes();
        *penalty_rate_dst = self.penalty_rate.to_le_bytes();
    }
}

impl Sealed for Agent {}
impl IsInitialized for Agent {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Agent {
    const LEN: usize = 70; // 1 + 32 + 8 + 4 + 4 + 4 + 8 + 8 + 8 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Agent::LEN];
        let (
            is_initialized,
            owner,
            stake_amount,
            performance_score,
            total_tasks,
            successful_tasks,
            rewards_earned,
            penalties_incurred,
            last_update,
            is_active,
        ) = array_refs![src, 1, 32, 8, 4, 4, 4, 8, 8, 8, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_active = match is_active[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Agent {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            stake_amount: u64::from_le_bytes(*stake_amount),
            performance_score: u32::from_le_bytes(*performance_score),
            total_tasks: u32::from_le_bytes(*total_tasks),
            successful_tasks: u32::from_le_bytes(*successful_tasks),
            rewards_earned: u64::from_le_bytes(*rewards_earned),
            penalties_incurred: u64::from_le_bytes(*penalties_incurred),
            last_update: i64::from_le_bytes(*last_update),
            is_active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Agent::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            stake_amount_dst,
            performance_score_dst,
            total_tasks_dst,
            successful_tasks_dst,
            rewards_earned_dst,
            penalties_incurred_dst,
            last_update_dst,
            is_active_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 4, 4, 4, 8, 8, 8, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        *stake_amount_dst = self.stake_amount.to_le_bytes();
        *performance_score_dst = self.performance_score.to_le_bytes();
        *total_tasks_dst = self.total_tasks.to_le_bytes();
        *successful_tasks_dst = self.successful_tasks.to_le_bytes();
        *rewards_earned_dst = self.rewards_earned.to_le_bytes();
        *penalties_incurred_dst = self.penalties_incurred.to_le_bytes();
        *last_update_dst = self.last_update.to_le_bytes();
        is_active_dst[0] = self.is_active as u8;
    }
}

impl Sealed for Performance {}
impl IsInitialized for Performance {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Performance {
    const LEN: usize = 214; // 1 + 32 + 32 + 4 + 8 + 32 + 128

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Performance::LEN];
        let (
            is_initialized,
            agent,
            task_id,
            score,
            timestamp,
            reviewer,
            feedback_uri,
        ) = array_refs![src, 1, 32, 32, 4, 8, 32, 128];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Performance {
            is_initialized,
            agent: Pubkey::new_from_array(*agent),
            task_id: Pubkey::new_from_array(*task_id),
            score: u32::from_le_bytes(*score),
            timestamp: i64::from_le_bytes(*timestamp),
            reviewer: Pubkey::new_from_array(*reviewer),
            feedback_uri: *feedback_uri,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Performance::LEN];
        let (
            is_initialized_dst,
            agent_dst,
            task_id_dst,
            score_dst,
            timestamp_dst,
            reviewer_dst,
            feedback_uri_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 4, 8, 32, 128];

        is_initialized_dst[0] = self.is_initialized as u8;
        agent_dst.copy_from_slice(self.agent.as_ref());
        task_id_dst.copy_from_slice(self.task_id.as_ref());
        *score_dst = self.score.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
        reviewer_dst.copy_from_slice(self.reviewer.as_ref());
        feedback_uri_dst.copy_from_slice(&self.feedback_uri);
    }
} 