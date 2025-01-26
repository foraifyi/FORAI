use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    program_error::ProgramError,
};

#[derive(Debug)]
pub struct InsurancePool {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub total_staked: u64,
    pub total_claims_paid: u64,
    pub min_stake_amount: u64,
    pub claim_delay_period: i64,    // Time delay before claim can be processed
    pub max_claim_amount: u64,
    pub stake_count: u32,
    pub claim_count: u32,
}

#[derive(Debug)]
pub struct Stake {
    pub is_initialized: bool,
    pub staker: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub locked_until: i64,          // Staking lock period
    pub rewards_claimed: u64,
}

#[derive(Debug)]
pub struct Claim {
    pub is_initialized: bool,
    pub claimer: Pubkey,
    pub project: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
    pub evidence_uri: [u8; 128],    // IPFS URI for bug evidence
    pub is_approved: bool,
    pub is_paid: bool,
    pub reviewer: Pubkey,
}

impl Sealed for InsurancePool {}
impl IsInitialized for InsurancePool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for InsurancePool {
    const LEN: usize = 73; // 1 + 32 + 8 + 8 + 8 + 8 + 8 + 4 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        // TODO: Implement unpacking
        unimplemented!()
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {
        // TODO: Implement packing
        unimplemented!()
    }
}

impl Sealed for Stake {}
impl IsInitialized for Stake {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Stake {
    const LEN: usize = 61; // 1 + 32 + 8 + 8 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Stake::LEN];
        let (
            is_initialized,
            staker,
            amount,
            timestamp,
            locked_until,
            rewards_claimed,
        ) = array_refs![src, 1, 32, 8, 8, 8, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Stake {
            is_initialized,
            staker: Pubkey::new_from_array(*staker),
            amount: u64::from_le_bytes(*amount),
            timestamp: i64::from_le_bytes(*timestamp),
            locked_until: i64::from_le_bytes(*locked_until),
            rewards_claimed: u64::from_le_bytes(*rewards_claimed),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Stake::LEN];
        let (
            is_initialized_dst,
            staker_dst,
            amount_dst,
            timestamp_dst,
            locked_until_dst,
            rewards_claimed_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        staker_dst.copy_from_slice(self.staker.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
        *locked_until_dst = self.locked_until.to_le_bytes();
        *rewards_claimed_dst = self.rewards_claimed.to_le_bytes();
    }
}

impl Sealed for Claim {}
impl IsInitialized for Claim {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Claim {
    const LEN: usize = 184; // 1 + 32 + 32 + 8 + 8 + 128 + 1 + 1 + 32

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Claim::LEN];
        let (
            is_initialized,
            claimer,
            project,
            amount,
            timestamp,
            evidence_uri,
            is_approved,
            is_paid,
            reviewer,
        ) = array_refs![src, 1, 32, 32, 8, 8, 128, 1, 1, 32];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_approved = match is_approved[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_paid = match is_paid[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Claim {
            is_initialized,
            claimer: Pubkey::new_from_array(*claimer),
            project: Pubkey::new_from_array(*project),
            amount: u64::from_le_bytes(*amount),
            timestamp: i64::from_le_bytes(*timestamp),
            evidence_uri: *evidence_uri,
            is_approved,
            is_paid,
            reviewer: Pubkey::new_from_array(*reviewer),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Claim::LEN];
        let (
            is_initialized_dst,
            claimer_dst,
            project_dst,
            amount_dst,
            timestamp_dst,
            evidence_uri_dst,
            is_approved_dst,
            is_paid_dst,
            reviewer_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 128, 1, 1, 32];

        is_initialized_dst[0] = self.is_initialized as u8;
        claimer_dst.copy_from_slice(self.claimer.as_ref());
        project_dst.copy_from_slice(self.project.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
        evidence_uri_dst.copy_from_slice(&self.evidence_uri);
        is_approved_dst[0] = self.is_approved as u8;
        is_paid_dst[0] = self.is_paid as u8;
        reviewer_dst.copy_from_slice(self.reviewer.as_ref());
    }
} 