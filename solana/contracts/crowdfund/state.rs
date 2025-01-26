use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProjectStatus {
    Active = 0,
    Funded = 1,
    Completed = 2,
    Failed = 3,
    Cancelled = 4,
}

impl ProjectStatus {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ProjectStatus::Active),
            1 => Some(ProjectStatus::Funded),
            2 => Some(ProjectStatus::Completed),
            3 => Some(ProjectStatus::Failed),
            4 => Some(ProjectStatus::Cancelled),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub title: [u8; 32],
    pub description: [u8; 64],
    pub target_amount: u64,
    pub current_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub current_milestone: u8,
    pub status: ProjectStatus,
    pub treasury: Pubkey,
}

#[derive(Debug, Clone)]
pub struct Milestone {
    pub description: [u8; 32],
    pub target_amount: u64,
    pub completion_time: i64,
    pub is_completed: bool,
    pub is_funds_released: bool,
}

#[derive(Debug)]
pub struct Investment {
    pub investor: Pubkey,
    pub project: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub is_refunded: bool,
}

impl Sealed for Project {}
impl IsInitialized for Project {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Project {
    const LEN: usize = 1 + 32 + 32 + 64 + 8 + 8 + 8 + 8 + 1 + 1 + 32; // 195 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Project::LEN];
        let (
            is_initialized,
            owner,
            title,
            description,
            target_amount,
            current_amount,
            start_time,
            end_time,
            current_milestone,
            status,
            treasury,
        ) = array_refs![src, 1, 32, 32, 64, 8, 8, 8, 8, 1, 1, 32];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let status = ProjectStatus::from_u8(status[0])
            .ok_or(ProgramError::InvalidAccountData)?;

        Ok(Project {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            title: *title,
            description: *description,
            target_amount: u64::from_le_bytes(*target_amount),
            current_amount: u64::from_le_bytes(*current_amount),
            start_time: i64::from_le_bytes(*start_time),
            end_time: i64::from_le_bytes(*end_time),
            current_milestone: current_milestone[0],
            status,
            treasury: Pubkey::new_from_array(*treasury),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Project::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            title_dst,
            description_dst,
            target_amount_dst,
            current_amount_dst,
            start_time_dst,
            end_time_dst,
            current_milestone_dst,
            status_dst,
            treasury_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 64, 8, 8, 8, 8, 1, 1, 32];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        title_dst.copy_from_slice(&self.title);
        description_dst.copy_from_slice(&self.description);
        *target_amount_dst = self.target_amount.to_le_bytes();
        *current_amount_dst = self.current_amount.to_le_bytes();
        *start_time_dst = self.start_time.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
        current_milestone_dst[0] = self.current_milestone;
        status_dst[0] = self.status as u8;
        treasury_dst.copy_from_slice(self.treasury.as_ref());
    }
}

impl Sealed for Investment {}
impl IsInitialized for Investment {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for Investment {
    const LEN: usize = 32 + 32 + 8 + 8 + 1; // 81 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Investment::LEN];
        let (investor, project, amount, timestamp, is_refunded) = 
            array_refs![src, 32, 32, 8, 8, 1];

        Ok(Investment {
            investor: Pubkey::new_from_array(*investor),
            project: Pubkey::new_from_array(*project),
            amount: u64::from_le_bytes(*amount),
            timestamp: i64::from_le_bytes(*timestamp),
            is_refunded: is_refunded[0] != 0,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Investment::LEN];
        let (investor_dst, project_dst, amount_dst, timestamp_dst, is_refunded_dst) = 
            mut_array_refs![dst, 32, 32, 8, 8, 1];

        investor_dst.copy_from_slice(self.investor.as_ref());
        project_dst.copy_from_slice(self.project.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
        is_refunded_dst[0] = self.is_refunded as u8;
    }
} 