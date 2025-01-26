use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Debug)]
pub struct AgentAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub reputation_score: u64,
    pub total_rewards: u64,
    pub completed_tasks: u32,
}

impl Sealed for AgentAccount {}

impl IsInitialized for AgentAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for AgentAccount {
    const LEN: usize = 65; // 1 + 32 + 8 + 8 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, AgentAccount::LEN];
        let (
            is_initialized,
            owner,
            reputation_score,
            total_rewards,
            completed_tasks,
        ) = array_refs![src, 1, 32, 8, 8, 4];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(AgentAccount {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            reputation_score: u64::from_le_bytes(*reputation_score),
            total_rewards: u64::from_le_bytes(*total_rewards),
            completed_tasks: u32::from_le_bytes(*completed_tasks),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, AgentAccount::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            reputation_score_dst,
            total_rewards_dst,
            completed_tasks_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 4];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        *reputation_score_dst = self.reputation_score.to_le_bytes();
        *total_rewards_dst = self.total_rewards.to_le_bytes();
        *completed_tasks_dst = self.completed_tasks.to_le_bytes();
    }
} 