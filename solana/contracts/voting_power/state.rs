use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct VotingPowerConfig {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub governance_token: Pubkey,
    pub delegation_enabled: bool,
    pub checkpoint_enabled: bool,
    pub checkpoint_interval: i64,  // Minimum time between checkpoints
    pub last_checkpoint: i64,      // Timestamp of last checkpoint
}

#[derive(Debug)]
pub struct VotingPower {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub amount: u64,
    pub delegated_amount: u64,
    pub delegate: Option<Pubkey>,
    pub checkpoint_number: u64,
    pub last_update: i64,
}

#[derive(Debug)]
pub struct Checkpoint {
    pub is_initialized: bool,
    pub number: u64,
    pub timestamp: i64,
    pub total_supply: u64,
    pub account_snapshot: Vec<(Pubkey, u64)>, // (account, voting power)
}

impl Sealed for VotingPowerConfig {}
impl IsInitialized for VotingPowerConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for VotingPowerConfig {
    const LEN: usize = 83; // 1 + 32 + 32 + 1 + 1 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, VotingPowerConfig::LEN];
        let (
            is_initialized,
            authority,
            governance_token,
            delegation_enabled,
            checkpoint_enabled,
            checkpoint_interval,
            last_checkpoint,
        ) = array_refs![src, 1, 32, 32, 1, 1, 8, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let delegation_enabled = match delegation_enabled[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let checkpoint_enabled = match checkpoint_enabled[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(VotingPowerConfig {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            governance_token: Pubkey::new_from_array(*governance_token),
            delegation_enabled,
            checkpoint_enabled,
            checkpoint_interval: i64::from_le_bytes(*checkpoint_interval),
            last_checkpoint: i64::from_le_bytes(*last_checkpoint),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, VotingPowerConfig::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            governance_token_dst,
            delegation_enabled_dst,
            checkpoint_enabled_dst,
            checkpoint_interval_dst,
            last_checkpoint_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 1, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        governance_token_dst.copy_from_slice(self.governance_token.as_ref());
        delegation_enabled_dst[0] = self.delegation_enabled as u8;
        checkpoint_enabled_dst[0] = self.checkpoint_enabled as u8;
        *checkpoint_interval_dst = self.checkpoint_interval.to_le_bytes();
        *last_checkpoint_dst = self.last_checkpoint.to_le_bytes();
    }
}

impl Sealed for VotingPower {}
impl IsInitialized for VotingPower {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for VotingPower {
    const LEN: usize = 91; // 1 + 32 + 8 + 8 + 33 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, VotingPower::LEN];
        let (
            is_initialized,
            owner,
            amount,
            delegated_amount,
            delegate_data,
            checkpoint_number,
            last_update,
        ) = array_refs![src, 1, 32, 8, 8, 33, 8, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let delegate = if delegate_data[0] == 0 {
            None
        } else {
            Some(Pubkey::new_from_array(*array_ref![delegate_data, 1, 32]))
        };

        Ok(VotingPower {
            is_initialized,
            owner: Pubkey::new_from_array(*owner),
            amount: u64::from_le_bytes(*amount),
            delegated_amount: u64::from_le_bytes(*delegated_amount),
            delegate,
            checkpoint_number: u64::from_le_bytes(*checkpoint_number),
            last_update: i64::from_le_bytes(*last_update),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, VotingPower::LEN];
        let (
            is_initialized_dst,
            owner_dst,
            amount_dst,
            delegated_amount_dst,
            delegate_dst,
            checkpoint_number_dst,
            last_update_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 33, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        owner_dst.copy_from_slice(self.owner.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *delegated_amount_dst = self.delegated_amount.to_le_bytes();
        
        if let Some(delegate) = self.delegate {
            delegate_dst[0] = 1;
            delegate_dst[1..].copy_from_slice(delegate.as_ref());
        } else {
            delegate_dst[0] = 0;
        }

        *checkpoint_number_dst = self.checkpoint_number.to_le_bytes();
        *last_update_dst = self.last_update.to_le_bytes();
    }
}

impl Sealed for Checkpoint {}
impl IsInitialized for Checkpoint {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Checkpoint {
    const LEN: usize = 1024; // Fixed size buffer for account snapshots

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Checkpoint::LEN];
        let (
            is_initialized,
            number,
            timestamp,
            total_supply,
            accounts_len,
            accounts_data,
        ) = array_refs![src, 1, 8, 8, 8, 2, 997];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let accounts_count = u16::from_le_bytes(*accounts_len) as usize;
        let mut account_snapshot = Vec::with_capacity(accounts_count);
        let account_size = 40; // 32 + 8

        for i in 0..accounts_count {
            let start = i * account_size;
            let account_data = array_ref![accounts_data, start, account_size];
            let (pubkey_data, power) = array_refs![account_data, 32, 8];

            account_snapshot.push((
                Pubkey::new_from_array(*pubkey_data),
                u64::from_le_bytes(*power),
            ));
        }

        Ok(Checkpoint {
            is_initialized,
            number: u64::from_le_bytes(*number),
            timestamp: i64::from_le_bytes(*timestamp),
            total_supply: u64::from_le_bytes(*total_supply),
            account_snapshot,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Checkpoint::LEN];
        let (
            is_initialized_dst,
            number_dst,
            timestamp_dst,
            total_supply_dst,
            accounts_len_dst,
            accounts_data_dst,
        ) = mut_array_refs![dst, 1, 8, 8, 8, 2, 997];

        is_initialized_dst[0] = self.is_initialized as u8;
        *number_dst = self.number.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
        *total_supply_dst = self.total_supply.to_le_bytes();
        *accounts_len_dst = (self.account_snapshot.len() as u16).to_le_bytes();

        let account_size = 40;
        for (i, (pubkey, power)) in self.account_snapshot.iter().enumerate() {
            let start = i * account_size;
            let account_dst = array_mut_ref![accounts_data_dst, start, account_size];
            let (pubkey_dst, power_dst) = mut_array_refs![account_dst, 32, 8];

            pubkey_dst.copy_from_slice(pubkey.as_ref());
            *power_dst = power.to_le_bytes();
        }
    }
} 