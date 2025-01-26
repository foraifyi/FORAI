use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct ProposalConfig {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub voting_power_program: Pubkey,
    pub min_voting_power: u64,      // Minimum power required to create proposal
    pub voting_delay: i64,          // Delay before voting starts
    pub voting_period: i64,         // Duration of voting period
    pub quorum_votes: u64,          // Minimum votes required
    pub proposal_count: u32,
}

#[derive(Debug)]
pub struct Proposal {
    pub is_initialized: bool,
    pub proposer: Pubkey,
    pub title: [u8; 32],           // Fixed size title
    pub description_url: [u8; 64],  // IPFS hash or other URL
    pub voting_starts: i64,
    pub voting_ends: i64,
    pub execution_time: i64,        // When proposal can be executed
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
    pub canceled: bool,
    pub executed: bool,
    pub voting_checkpoint: Pubkey,  // Reference to voting power snapshot
}

#[derive(Debug)]
pub struct Vote {
    pub is_initialized: bool,
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub voter_power: u64,
    pub support: u8,               // 0 = Against, 1 = For, 2 = Abstain
    pub vote_time: i64,
}

impl Sealed for ProposalConfig {}
impl IsInitialized for ProposalConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for ProposalConfig {
    const LEN: usize = 89; // 1 + 32 + 32 + 8 + 8 + 8 + 8 + 4

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ProposalConfig::LEN];
        let (
            is_initialized,
            authority,
            voting_power_program,
            min_voting_power,
            voting_delay,
            voting_period,
            quorum_votes,
            proposal_count,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 8, 4];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(ProposalConfig {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            voting_power_program: Pubkey::new_from_array(*voting_power_program),
            min_voting_power: u64::from_le_bytes(*min_voting_power),
            voting_delay: i64::from_le_bytes(*voting_delay),
            voting_period: i64::from_le_bytes(*voting_period),
            quorum_votes: u64::from_le_bytes(*quorum_votes),
            proposal_count: u32::from_le_bytes(*proposal_count),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ProposalConfig::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            voting_power_program_dst,
            min_voting_power_dst,
            voting_delay_dst,
            voting_period_dst,
            quorum_votes_dst,
            proposal_count_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 8, 4];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        voting_power_program_dst.copy_from_slice(self.voting_power_program.as_ref());
        *min_voting_power_dst = self.min_voting_power.to_le_bytes();
        *voting_delay_dst = self.voting_delay.to_le_bytes();
        *voting_period_dst = self.voting_period.to_le_bytes();
        *quorum_votes_dst = self.quorum_votes.to_le_bytes();
        *proposal_count_dst = self.proposal_count.to_le_bytes();
    }
}

impl Sealed for Proposal {}
impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Proposal {
    const LEN: usize = 179; // 1 + 32 + 32 + 64 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 32

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Proposal::LEN];
        let (
            is_initialized,
            proposer,
            title,
            description_url,
            voting_starts,
            voting_ends,
            execution_time,
            for_votes,
            against_votes,
            abstain_votes,
            canceled,
            executed,
            voting_checkpoint,
        ) = array_refs![src, 1, 32, 32, 64, 8, 8, 8, 8, 8, 8, 1, 1, 32];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let canceled = match canceled[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let executed = match executed[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Proposal {
            is_initialized,
            proposer: Pubkey::new_from_array(*proposer),
            title: *title,
            description_url: *description_url,
            voting_starts: i64::from_le_bytes(*voting_starts),
            voting_ends: i64::from_le_bytes(*voting_ends),
            execution_time: i64::from_le_bytes(*execution_time),
            for_votes: u64::from_le_bytes(*for_votes),
            against_votes: u64::from_le_bytes(*against_votes),
            abstain_votes: u64::from_le_bytes(*abstain_votes),
            canceled,
            executed,
            voting_checkpoint: Pubkey::new_from_array(*voting_checkpoint),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Proposal::LEN];
        let (
            is_initialized_dst,
            proposer_dst,
            title_dst,
            description_url_dst,
            voting_starts_dst,
            voting_ends_dst,
            execution_time_dst,
            for_votes_dst,
            against_votes_dst,
            abstain_votes_dst,
            canceled_dst,
            executed_dst,
            voting_checkpoint_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 64, 8, 8, 8, 8, 8, 8, 1, 1, 32];

        is_initialized_dst[0] = self.is_initialized as u8;
        proposer_dst.copy_from_slice(self.proposer.as_ref());
        title_dst.copy_from_slice(&self.title);
        description_url_dst.copy_from_slice(&self.description_url);
        *voting_starts_dst = self.voting_starts.to_le_bytes();
        *voting_ends_dst = self.voting_ends.to_le_bytes();
        *execution_time_dst = self.execution_time.to_le_bytes();
        *for_votes_dst = self.for_votes.to_le_bytes();
        *against_votes_dst = self.against_votes.to_le_bytes();
        *abstain_votes_dst = self.abstain_votes.to_le_bytes();
        canceled_dst[0] = self.canceled as u8;
        executed_dst[0] = self.executed as u8;
        voting_checkpoint_dst.copy_from_slice(self.voting_checkpoint.as_ref());
    }
}

impl Sealed for Vote {}
impl IsInitialized for Vote {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Vote {
    const LEN: usize = 83; // 1 + 32 + 32 + 8 + 1 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Vote::LEN];
        let (
            is_initialized,
            proposal,
            voter,
            voter_power,
            support,
            vote_time,
        ) = array_refs![src, 1, 32, 32, 8, 1, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Vote {
            is_initialized,
            proposal: Pubkey::new_from_array(*proposal),
            voter: Pubkey::new_from_array(*voter),
            voter_power: u64::from_le_bytes(*voter_power),
            support: support[0],
            vote_time: i64::from_le_bytes(*vote_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Vote::LEN];
        let (
            is_initialized_dst,
            proposal_dst,
            voter_dst,
            voter_power_dst,
            support_dst,
            vote_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 1, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        proposal_dst.copy_from_slice(self.proposal.as_ref());
        voter_dst.copy_from_slice(self.voter.as_ref());
        *voter_power_dst = self.voter_power.to_le_bytes();
        support_dst[0] = self.support;
        *vote_time_dst = self.vote_time.to_le_bytes();
    }
} 