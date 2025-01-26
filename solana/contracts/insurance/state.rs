use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    program_error::ProgramError,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InsurancePoolStatus {
    Active = 0,
    Paused = 1,
    Liquidated = 2,
}

impl InsurancePoolStatus {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(InsurancePoolStatus::Active),
            1 => Some(InsurancePoolStatus::Paused),
            2 => Some(InsurancePoolStatus::Liquidated),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct InsurancePool {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub name: [u8; 32],
    pub status: InsurancePoolStatus,
    pub total_capital: u64,
    pub available_capital: u64,
    pub locked_capital: u64,
    pub min_capital_requirement: u64,
    pub coverage_ratio: u8,
    pub premium_rate: u8,
    pub claim_period: u64,
    pub creation_time: i64,
    pub update_time: i64,
}

#[derive(Debug)]
pub struct InsurancePolicy {
    pub is_initialized: bool,
    pub pool: Pubkey,
    pub insured: Pubkey,
    pub coverage_amount: u64,
    pub premium_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub claim_deadline: i64,
    pub is_claimed: bool,
    pub claim_amount: u64,
    pub claim_time: i64,
}

#[derive(Debug)]
pub struct Claim {
    pub is_initialized: bool,
    pub policy: Pubkey,
    pub claimant: Pubkey,
    pub amount: u64,
    pub evidence: [u8; 128],
    pub status: ClaimStatus,
    pub submission_time: i64,
    pub processing_time: i64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ClaimStatus {
    Pending = 0,
    Approved = 1,
    Rejected = 2,
}

impl ClaimStatus {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ClaimStatus::Pending),
            1 => Some(ClaimStatus::Approved),
            2 => Some(ClaimStatus::Rejected),
            _ => None,
        }
    }
}

impl Sealed for InsurancePool {}
impl IsInitialized for InsurancePool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for InsurancePool {
    const LEN: usize = 1 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 8 + 1 + 1 + 8 + 8 + 8; // 156 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, InsurancePool::LEN];
        let (
            is_initialized,
            admin,
            treasury,
            name,
            status,
            total_capital,
            available_capital,
            locked_capital,
            min_capital_requirement,
            coverage_ratio,
            premium_rate,
            claim_period,
            creation_time,
            update_time,
        ) = array_refs![src, 1, 32, 32, 32, 1, 8, 8, 8, 8, 1, 1, 8, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let status = InsurancePoolStatus::from_u8(status[0])
            .ok_or(ProgramError::InvalidAccountData)?;

        Ok(InsurancePool {
            is_initialized,
            admin: Pubkey::new_from_array(*admin),
            treasury: Pubkey::new_from_array(*treasury),
            name: *name,
            status,
            total_capital: u64::from_le_bytes(*total_capital),
            available_capital: u64::from_le_bytes(*available_capital),
            locked_capital: u64::from_le_bytes(*locked_capital),
            min_capital_requirement: u64::from_le_bytes(*min_capital_requirement),
            coverage_ratio: coverage_ratio[0],
            premium_rate: premium_rate[0],
            claim_period: u64::from_le_bytes(*claim_period),
            creation_time: i64::from_le_bytes(*creation_time),
            update_time: i64::from_le_bytes(*update_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, InsurancePool::LEN];
        let (
            is_initialized_dst,
            admin_dst,
            treasury_dst,
            name_dst,
            status_dst,
            total_capital_dst,
            available_capital_dst,
            locked_capital_dst,
            min_capital_requirement_dst,
            coverage_ratio_dst,
            premium_rate_dst,
            claim_period_dst,
            creation_time_dst,
            update_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 1, 8, 8, 8, 8, 1, 1, 8, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        admin_dst.copy_from_slice(self.admin.as_ref());
        treasury_dst.copy_from_slice(self.treasury.as_ref());
        name_dst.copy_from_slice(&self.name);
        status_dst[0] = self.status as u8;
        *total_capital_dst = self.total_capital.to_le_bytes();
        *available_capital_dst = self.available_capital.to_le_bytes();
        *locked_capital_dst = self.locked_capital.to_le_bytes();
        *min_capital_requirement_dst = self.min_capital_requirement.to_le_bytes();
        coverage_ratio_dst[0] = self.coverage_ratio;
        premium_rate_dst[0] = self.premium_rate;
        *claim_period_dst = self.claim_period.to_le_bytes();
        *creation_time_dst = self.creation_time.to_le_bytes();
        *update_time_dst = self.update_time.to_le_bytes();
    }
}

impl Sealed for InsurancePolicy {}
impl IsInitialized for InsurancePolicy {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for InsurancePolicy {
    const LEN: usize = 1 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 8 + 8; // 122 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, InsurancePolicy::LEN];
        let (
            is_initialized,
            pool,
            insured,
            coverage_amount,
            premium_amount,
            start_time,
            end_time,
            claim_deadline,
            is_claimed,
            claim_amount,
            claim_time,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 8, 8, 1, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_claimed = match is_claimed {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(InsurancePolicy {
            is_initialized,
            pool: Pubkey::new_from_array(*pool),
            insured: Pubkey::new_from_array(*insured),
            coverage_amount: u64::from_le_bytes(*coverage_amount),
            premium_amount: u64::from_le_bytes(*premium_amount),
            start_time: i64::from_le_bytes(*start_time),
            end_time: i64::from_le_bytes(*end_time),
            claim_deadline: i64::from_le_bytes(*claim_deadline),
            is_claimed,
            claim_amount: u64::from_le_bytes(*claim_amount),
            claim_time: i64::from_le_bytes(*claim_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, InsurancePolicy::LEN];
        let (
            is_initialized_dst,
            pool_dst,
            insured_dst,
            coverage_amount_dst,
            premium_amount_dst,
            start_time_dst,
            end_time_dst,
            claim_deadline_dst,
            is_claimed_dst,
            claim_amount_dst,
            claim_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 8, 8, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        pool_dst.copy_from_slice(self.pool.as_ref());
        insured_dst.copy_from_slice(self.insured.as_ref());
        *coverage_amount_dst = self.coverage_amount.to_le_bytes();
        *premium_amount_dst = self.premium_amount.to_le_bytes();
        *start_time_dst = self.start_time.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
        *claim_deadline_dst = self.claim_deadline.to_le_bytes();
        is_claimed_dst[0] = self.is_claimed as u8;
        *claim_amount_dst = self.claim_amount.to_le_bytes();
        *claim_time_dst = self.claim_time.to_le_bytes();
    }
}

impl Sealed for Claim {}
impl IsInitialized for Claim {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Claim {
    const LEN: usize = 1 + 32 + 32 + 8 + 128 + 1 + 8 + 8; // 218 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Claim::LEN];
        let (
            is_initialized,
            policy,
            claimant,
            amount,
            evidence,
            status,
            submission_time,
            processing_time,
        ) = array_refs![src, 1, 32, 32, 8, 128, 1, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let status = ClaimStatus::from_u8(status[0])
            .ok_or(ProgramError::InvalidAccountData)?;

        Ok(Claim {
            is_initialized,
            policy: Pubkey::new_from_array(*policy),
            claimant: Pubkey::new_from_array(*claimant),
            amount: u64::from_le_bytes(*amount),
            evidence: *evidence,
            status,
            submission_time: i64::from_le_bytes(*submission_time),
            processing_time: i64::from_le_bytes(*processing_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Claim::LEN];
        let (
            is_initialized_dst,
            policy_dst,
            claimant_dst,
            amount_dst,
            evidence_dst,
            status_dst,
            submission_time_dst,
            processing_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 128, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        policy_dst.copy_from_slice(self.policy.as_ref());
        claimant_dst.copy_from_slice(self.claimant.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        evidence_dst.copy_from_slice(&self.evidence);
        status_dst[0] = self.status as u8;
        *submission_time_dst = self.submission_time.to_le_bytes();
        *processing_time_dst = self.processing_time.to_le_bytes();
    }
} 