use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
    program_error::ProgramError,
};

#[derive(Debug)]
pub struct InsurancePool {
    pub is_initialized: bool,
    pub total_staked: u64,
    pub total_claims: u64,
    pub stakers: Vec<Stake>,
    pub claims: Vec<Claim>,
    pub risk_parameters: RiskParameters,
}

#[derive(Debug)]
pub struct Stake {
    pub staker: Pubkey,
    pub amount: u64,
    pub locked_until: i64,
}

#[derive(Debug)]
pub struct Claim {
    pub claimer: Pubkey,
    pub amount: u64,
    pub reason: [u8; 64],
    pub status: ClaimStatus,
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
        true
    }
}

impl Pack for Stake {
    const LEN: usize = 61; // 1 + 32 + 8 + 8 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Stake::LEN];
        let (
            staker,
            amount,
            locked_until,
        ) = array_refs![src, 32, 8, 8];

        Ok(Stake {
            staker: Pubkey::new_from_array(*staker),
            amount: u64::from_le_bytes(*amount),
            locked_until: i64::from_le_bytes(*locked_until),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Stake::LEN];
        let (
            staker_dst,
            amount_dst,
            locked_until_dst,
        ) = mut_array_refs![dst, 32, 8, 8];

        staker_dst.copy_from_slice(self.staker.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        *locked_until_dst = self.locked_until.to_le_bytes();
    }
}

impl Sealed for Claim {}
impl IsInitialized for Claim {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Pack for Claim {
    const LEN: usize = 184; // 1 + 32 + 32 + 8 + 8 + 128 + 1 + 1 + 32

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Claim::LEN];
        let (
            claimer,
            amount,
            reason,
            status,
        ) = array_refs![src, 32, 8, 64, 1];

        Ok(Claim {
            claimer: Pubkey::new_from_array(*claimer),
            amount: u64::from_le_bytes(*amount),
            reason: *reason,
            status: ClaimStatus::from_le_bytes(*status),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Claim::LEN];
        let (
            claimer_dst,
            amount_dst,
            reason_dst,
            status_dst,
        ) = mut_array_refs![dst, 32, 8, 64, 1];

        claimer_dst.copy_from_slice(self.claimer.as_ref());
        *amount_dst = self.amount.to_le_bytes();
        reason_dst.copy_from_slice(&self.reason);
        *status_dst = self.status.to_le_bytes();
    }
} 