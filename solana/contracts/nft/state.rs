use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug)]
pub struct SoftwareNFT {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub metadata: Metadata,
    pub usage_rights: UsageRights,
    pub version_history: Vec<Version>,
    pub license_type: LicenseType,
}

#[derive(Debug)]
pub struct Metadata {
    pub name: [u8; 32],
    pub description: [u8; 64],
    pub repository: [u8; 64],
    pub created_at: i64,
}

pub struct UsageRights {
    pub can_modify: bool,
    pub can_redistribute: bool,
    pub can_sublicense: bool,
}

impl Sealed for SoftwareNFT {}
impl IsInitialized for SoftwareNFT {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for SoftwareNFT {
    const LEN: usize = 177; // 1 + 32 + 32 + 128 + 4 + 1 + 1 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        // TODO: Implement unpacking
        unimplemented!()
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {
        // TODO: Implement packing
        unimplemented!()
    }
} 