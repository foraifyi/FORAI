use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq)]
pub enum ProjectStatus {
    Active,
    Funded,
    Completed,
    Failed,
    Cancelled,
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
    pub milestones: Vec<Milestone>,
    pub current_milestone: u8,
    pub status: ProjectStatus,
    pub treasury: Pubkey,
}

#[derive(Debug)]
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
    const LEN: usize = 233; // Calculate exact size

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Project::LEN];
        // ... implement unpacking logic
        unimplemented!()
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Project::LEN];
        // ... implement packing logic
        unimplemented!()
    }
}

impl Sealed for Investment {}
impl IsInitialized for Investment {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for Investment {
    const LEN: usize = 82; // 32 + 32 + 8 + 8 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Investment::LEN];
        // ... implement unpacking logic
        unimplemented!()
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Investment::LEN];
        // ... implement packing logic
        unimplemented!()
    }
} 