use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Debug)]
pub struct SoftwareNFT {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub project: Pubkey,
    pub metadata_uri: [u8; 128],  // IPFS URI for software metadata
    pub version: u32,
    pub is_transferable: bool,
    pub can_modify: bool,
    pub royalty_percentage: u8,    // in basis points (0-10000)
}

#[derive(Debug)]
pub struct Metadata {
    pub name: [u8; 32],
    pub symbol: [u8; 8],
    pub code_uri: [u8; 128],      // IPFS URI for actual code
    pub license: [u8; 32],
    pub description: [u8; 256],
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