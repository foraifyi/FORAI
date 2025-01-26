use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NFTStatus {
    Active = 0,
    Locked = 1,
    Burned = 2,
}

impl NFTStatus {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(NFTStatus::Active),
            1 => Some(NFTStatus::Locked),
            2 => Some(NFTStatus::Burned),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct NFTMetadata {
    pub is_initialized: bool,
    pub creator: Pubkey,
    pub owner: Pubkey,
    pub name: [u8; 32],
    pub symbol: [u8; 8],
    pub uri: [u8; 128],
    pub status: NFTStatus,
    pub royalty_percentage: u8,
    pub total_supply: u64,
    pub current_supply: u64,
    pub project: Pubkey,
    pub creation_time: i64,
    pub update_time: i64,
}

#[derive(Debug)]
pub struct NFTHolder {
    pub owner: Pubkey,
    pub nft_mint: Pubkey,
    pub amount: u64,
    pub locked_amount: u64,
    pub acquisition_time: i64,
}

impl Sealed for NFTMetadata {}
impl IsInitialized for NFTMetadata {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for NFTMetadata {
    const LEN: usize = 1 + 32 + 32 + 32 + 8 + 128 + 1 + 1 + 8 + 8 + 32 + 8 + 8; // 299 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, NFTMetadata::LEN];
        let (
            is_initialized,
            creator,
            owner,
            name,
            symbol,
            uri,
            status,
            royalty_percentage,
            total_supply,
            current_supply,
            project,
            creation_time,
            update_time,
        ) = array_refs![src, 1, 32, 32, 32, 8, 128, 1, 1, 8, 8, 32, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let status = NFTStatus::from_u8(status[0])
            .ok_or(ProgramError::InvalidAccountData)?;

        Ok(NFTMetadata {
            is_initialized,
            creator: Pubkey::new_from_array(*creator),
            owner: Pubkey::new_from_array(*owner),
            name: *name,
            symbol: *symbol,
            uri: *uri,
            status,
            royalty_percentage: royalty_percentage[0],
            total_supply: u64::from_le_bytes(*total_supply),
            current_supply: u64::from_le_bytes(*current_supply),
            project: Pubkey::new_from_array(*project),
            creation_time: i64::from_le_bytes(*creation_time),
            update_time: i64::from_le_bytes(*update_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, NFTMetadata::LEN];
        let (
            is_initialized_dst,
            creator_dst,
            owner_dst,
            name_dst,
            symbol_dst,
            uri_dst,
            status_dst,
            royalty_percentage_dst,
            total_supply_dst,
            current_supply_dst,
            project_dst,
            creation_time_dst,
            update_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 128, 1, 1, 8, 8, 32, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        creator_dst.copy_from_slice(self.creator.as_ref());
        owner_dst.copy_from_slice(self.owner.as_ref());
        name_dst.copy_from_slice(&self.name);
        symbol_dst.copy_from_slice(&self.symbol);
        uri_dst.copy_from_slice(&self.uri);
        status_dst[0] = self.status as u8;
        royalty_percentage_dst[0] = self.royalty_percentage;
        *total_supply_dst = self.total_supply.to_le_bytes();
        *current_supply_dst = self.current_supply.to_le_bytes();
        project_dst.copy_from_slice(self.project.as_ref());
        *creation_time_dst = self.creation_time.to_le_bytes();
        *update_time_dst = self.update_time.to_le_bytes();
    }
}

impl Sealed for NFTHolder {}
impl IsInitialized for NFTHolder {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for NFTHolder {
    const LEN: usize = 32 + 32 + 8 + 8 + 8; // 88 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, NFTHolder::LEN];
        let (owner, nft_mint, amount, locked_amount, acquisition_time) = 
            array_refs![src, 32, 32, 8, 8, 8];

        Ok(NFTHolder {
            owner: Pubkey::new_from_array(*owner),
            nft_mint: Pubkey::new_from_array(*nft_mint),
            amount: u64::from_le_bytes(*amount),
            locked_amount: u64::from_le_bytes(*locked_amount),
            acquisition_time: i64::from_le_bytes(*acquisition_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, NFTHolder::LEN];
        let (owner_dst, nft_mint_dst, amount_dst, locked_amount_dst, acquisition_time_dst) = 
            mut_array_refs![dst, 32, 32, 8, 8, 8];

        owner_dst.copy_from_slice(self.owner.as_ref());
        nft_mint_dst.copy_from_slice(self.nft_mint.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *locked_amount_dst = self.locked_amount.to_le_bytes();
        *acquisition_time_dst = self.acquisition_time.to_le_bytes();
    }
} 