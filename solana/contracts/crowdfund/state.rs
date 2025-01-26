use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct Project {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub treasury: Pubkey,
    pub target_amount: u64,
    pub current_amount: u64,
    pub deadline: i64,
    pub is_completed: bool,
    pub milestone_count: u8,
    pub current_milestone: u8,
}

#[derive(Debug)]
pub struct Investment {
    pub is_initialized: bool,
    pub investor: Pubkey,
    pub project: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

impl Sealed for Project {}
impl IsInitialized for Project {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Project {
    const LEN: usize = 82; // 1 + 32 + 32 + 8 + 8 + 8 + 1 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Project::LEN];
        let (
            is_initialized,
            owner,
            treasury,
            target_amount,
            current_amount,
            deadline,
            is_completed,
            milestone_count,
            current_milestone,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 1, 1, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_completed = match is_completed[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Project {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            treasury: Pubkey::new_from_array(*treasury),
            target_amount: u64::from_le_bytes(*target_amount),
            current_amount: u64::from_le_bytes(*current_amount),
            deadline: i64::from_le_bytes(*deadline),
            is_completed,
            milestone_count: milestone_count[0],
            current_milestone: current_milestone[0],
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Project::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            treasury_dst,
            target_amount_dst,
            current_amount_dst,
            deadline_dst,
            is_completed_dst,
            milestone_count_dst,
            current_milestone_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 1, 1, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        treasury_dst.copy_from_slice(self.treasury.as_ref());
        *target_amount_dst = self.target_amount.to_le_bytes();
        *current_amount_dst = self.current_amount.to_le_bytes();
        *deadline_dst = self.deadline.to_le_bytes();
        is_completed_dst[0] = self.is_completed as u8;
        milestone_count_dst[0] = self.milestone_count;
        current_milestone_dst[0] = self.current_milestone;
    }
}

impl Sealed for Investment {}
impl IsInitialized for Investment {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Investment {
    const LEN: usize = 82; // 1 + 32 + 32 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Investment::LEN];
        let (
            is_initialized,
            investor,
            project,
            amount,
            timestamp,
        ) = array_refs![src, 1, 32, 32, 8, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Investment {
            is_initialized,
            investor: Pubkey::new_from_array(*investor),
            project: Pubkey::new_from_array(*project),
            amount: u64::from_le_bytes(*amount),
            timestamp: i64::from_le_bytes(*timestamp),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Investment::LEN];
        let (
            is_initialized_dst,
            investor_dst,
            project_dst,
            amount_dst,
            timestamp_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        investor_dst.copy_from_slice(self.investor.as_ref());
        project_dst.copy_from_slice(self.project.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
    }
} 