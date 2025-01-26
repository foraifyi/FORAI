use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProposalStatus {
    Draft = 0,
    Active = 1,
    Canceled = 2,
    Defeated = 3,
    Succeeded = 4,
    Queued = 5,
    Expired = 6,
    Executed = 7,
}

impl ProposalStatus {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ProposalStatus::Draft),
            1 => Some(ProposalStatus::Active),
            2 => Some(ProposalStatus::Canceled),
            3 => Some(ProposalStatus::Defeated),
            4 => Some(ProposalStatus::Succeeded),
            5 => Some(ProposalStatus::Queued),
            6 => Some(ProposalStatus::Expired),
            7 => Some(ProposalStatus::Executed),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VoteType {
    SingleChoice = 0,
    MultiChoice = 1,
    Ranked = 2,
}

impl VoteType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(VoteType::SingleChoice),
            1 => Some(VoteType::MultiChoice),
            2 => Some(VoteType::Ranked),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct GovernanceConfig {
    pub is_initialized: bool,
    pub admin: Pubkey,
    pub name: [u8; 32],
    pub voting_delay: u64,
    pub voting_period: u64,
    pub quorum_votes: u64,
    pub timelock_delay: u64,
    pub proposal_threshold: u64,
    pub vote_threshold: u8,
    pub creation_time: i64,
    pub update_time: i64,
}

#[derive(Debug)]
pub struct Proposal {
    pub is_initialized: bool,
    pub governance: Pubkey,
    pub proposer: Pubkey,
    pub title: [u8; 64],
    pub description: [u8; 256],
    pub vote_type: VoteType,
    pub choices: [[u8; 32]; 8],
    pub choice_count: u8,
    pub status: ProposalStatus,
    pub start_time: i64,
    pub end_time: i64,
    pub execute_time: i64,
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
    pub quorum_reached: bool,
    pub creation_time: i64,
    pub update_time: i64,
}

#[derive(Debug)]
pub struct Vote {
    pub is_initialized: bool,
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub vote_weight: u64,
    pub support: bool,
    pub choices: [u8; 8],
    pub voting_time: i64,
}

#[derive(Debug)]
pub struct Action {
    pub is_initialized: bool,
    pub proposal: Pubkey,
    pub program_id: Pubkey,
    pub accounts: Vec<ActionAccount>,
    pub data: Vec<u8>,
    pub executed: bool,
}

#[derive(Debug)]
pub struct ActionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug)]
pub struct Queue {
    pub is_initialized: bool,
    pub proposal: Pubkey,
    pub execution_time: i64,
    pub actions: Vec<Pubkey>,
    pub executed: bool,
    pub canceled: bool,
}

impl Sealed for GovernanceConfig {}
impl IsInitialized for GovernanceConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for GovernanceConfig {
    const LEN: usize = 1 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 1 + 8 + 8; // 122 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, GovernanceConfig::LEN];
        let (
            is_initialized,
            admin,
            name,
            voting_delay,
            voting_period,
            quorum_votes,
            timelock_delay,
            proposal_threshold,
            vote_threshold,
            creation_time,
            update_time,
        ) = array_refs![src, 1, 32, 32, 8, 8, 8, 8, 8, 1, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(GovernanceConfig {
            is_initialized,
            admin: Pubkey::new_from_array(*admin),
            name: *name,
            voting_delay: u64::from_le_bytes(*voting_delay),
            voting_period: u64::from_le_bytes(*voting_period),
            quorum_votes: u64::from_le_bytes(*quorum_votes),
            timelock_delay: u64::from_le_bytes(*timelock_delay),
            proposal_threshold: u64::from_le_bytes(*proposal_threshold),
            vote_threshold: vote_threshold[0],
            creation_time: i64::from_le_bytes(*creation_time),
            update_time: i64::from_le_bytes(*update_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GovernanceConfig::LEN];
        let (
            is_initialized_dst,
            admin_dst,
            name_dst,
            voting_delay_dst,
            voting_period_dst,
            quorum_votes_dst,
            timelock_delay_dst,
            proposal_threshold_dst,
            vote_threshold_dst,
            creation_time_dst,
            update_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 8, 8, 8, 8, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        admin_dst.copy_from_slice(self.admin.as_ref());
        name_dst.copy_from_slice(&self.name);
        *voting_delay_dst = self.voting_delay.to_le_bytes();
        *voting_period_dst = self.voting_period.to_le_bytes();
        *quorum_votes_dst = self.quorum_votes.to_le_bytes();
        *timelock_delay_dst = self.timelock_delay.to_le_bytes();
        *proposal_threshold_dst = self.proposal_threshold.to_le_bytes();
        vote_threshold_dst[0] = self.vote_threshold;
        *creation_time_dst = self.creation_time.to_le_bytes();
        *update_time_dst = self.update_time.to_le_bytes();
    }
}

impl Sealed for Proposal {}
impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Proposal {
    const LEN: usize = 1 + 32 + 32 + 64 + 256 + 1 + 256 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 8 + 8; // 709 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Proposal::LEN];
        let (
            is_initialized,
            governance,
            proposer,
            title,
            description,
            vote_type,
            choices,
            choice_count,
            status,
            start_time,
            end_time,
            execute_time,
            for_votes,
            against_votes,
            abstain_votes,
            quorum_reached,
            creation_time,
            update_time,
        ) = array_refs![src, 1, 32, 32, 64, 256, 1, 256, 1, 1, 8, 8, 8, 8, 8, 8, 1, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let vote_type = VoteType::from_u8(vote_type[0])
            .ok_or(ProgramError::InvalidAccountData)?;

        let status = ProposalStatus::from_u8(status[0])
            .ok_or(ProgramError::InvalidAccountData)?;

        let quorum_reached = match quorum_reached {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let mut choices_array = [[0u8; 32]; 8];
        for (i, chunk) in choices.chunks(32).enumerate() {
            if i < 8 {
                choices_array[i].copy_from_slice(chunk);
            }
        }

        Ok(Proposal {
            is_initialized,
            governance: Pubkey::new_from_array(*governance),
            proposer: Pubkey::new_from_array(*proposer),
            title: *title,
            description: *description,
            vote_type,
            choices: choices_array,
            choice_count: choice_count[0],
            status,
            start_time: i64::from_le_bytes(*start_time),
            end_time: i64::from_le_bytes(*end_time),
            execute_time: i64::from_le_bytes(*execute_time),
            for_votes: u64::from_le_bytes(*for_votes),
            against_votes: u64::from_le_bytes(*against_votes),
            abstain_votes: u64::from_le_bytes(*abstain_votes),
            quorum_reached,
            creation_time: i64::from_le_bytes(*creation_time),
            update_time: i64::from_le_bytes(*update_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Proposal::LEN];
        let (
            is_initialized_dst,
            governance_dst,
            proposer_dst,
            title_dst,
            description_dst,
            vote_type_dst,
            choices_dst,
            choice_count_dst,
            status_dst,
            start_time_dst,
            end_time_dst,
            execute_time_dst,
            for_votes_dst,
            against_votes_dst,
            abstain_votes_dst,
            quorum_reached_dst,
            creation_time_dst,
            update_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 64, 256, 1, 256, 1, 1, 8, 8, 8, 8, 8, 8, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        governance_dst.copy_from_slice(self.governance.as_ref());
        proposer_dst.copy_from_slice(self.proposer.as_ref());
        title_dst.copy_from_slice(&self.title);
        description_dst.copy_from_slice(&self.description);
        vote_type_dst[0] = self.vote_type as u8;
        
        let mut choices_flat = [0u8; 256];
        for (i, choice) in self.choices.iter().enumerate() {
            if i < 8 {
                choices_flat[i*32..(i+1)*32].copy_from_slice(choice);
            }
        }
        choices_dst.copy_from_slice(&choices_flat);
        
        choice_count_dst[0] = self.choice_count;
        status_dst[0] = self.status as u8;
        *start_time_dst = self.start_time.to_le_bytes();
        *end_time_dst = self.end_time.to_le_bytes();
        *execute_time_dst = self.execute_time.to_le_bytes();
        *for_votes_dst = self.for_votes.to_le_bytes();
        *against_votes_dst = self.against_votes.to_le_bytes();
        *abstain_votes_dst = self.abstain_votes.to_le_bytes();
        quorum_reached_dst[0] = self.quorum_reached as u8;
        *creation_time_dst = self.creation_time.to_le_bytes();
        *update_time_dst = self.update_time.to_le_bytes();
    }
}

impl Sealed for Vote {}
impl IsInitialized for Vote {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Vote {
    const LEN: usize = 1 + 32 + 32 + 8 + 1 + 8 + 8; // 90 bytes

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Vote::LEN];
        let (
            is_initialized,
            proposal,
            voter,
            vote_weight,
            support,
            choices,
            voting_time,
        ) = array_refs![src, 1, 32, 32, 8, 1, 8, 8];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let support = match support {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Vote {
            is_initialized,
            proposal: Pubkey::new_from_array(*proposal),
            voter: Pubkey::new_from_array(*voter),
            vote_weight: u64::from_le_bytes(*vote_weight),
            support,
            choices: *choices,
            voting_time: i64::from_le_bytes(*voting_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Vote::LEN];
        let (
            is_initialized_dst,
            proposal_dst,
            voter_dst,
            vote_weight_dst,
            support_dst,
            choices_dst,
            voting_time_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 8, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        proposal_dst.copy_from_slice(self.proposal.as_ref());
        voter_dst.copy_from_slice(self.voter.as_ref());
        *vote_weight_dst = self.vote_weight.to_le_bytes();
        support_dst[0] = self.support as u8;
        choices_dst.copy_from_slice(&self.choices);
        *voting_time_dst = self.voting_time.to_le_bytes();
    }
}

impl Sealed for Action {}
impl IsInitialized for Action {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Action {
    const LEN: usize = 1024; // Fixed size buffer for accounts and data

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Action::LEN];
        let (
            is_initialized,
            proposal,
            program_id,
            accounts_len,
            accounts_data,
            data_len,
            data_buffer,
            executed,
        ) = array_refs![src, 1, 32, 32, 2, 512, 2, 442, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let executed = match executed[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let accounts_count = u16::from_le_bytes(*accounts_len) as usize;
        let mut accounts = Vec::with_capacity(accounts_count);
        let account_size = 34; // 32 + 1 + 1

        for i in 0..accounts_count {
            let start = i * account_size;
            let account_data = array_ref![accounts_data, start, account_size];
            let (pubkey_data, is_signer, is_writable) = array_refs![account_data, 32, 1, 1];

            accounts.push(ActionAccount {
                pubkey: Pubkey::new_from_array(*pubkey_data),
                is_signer: is_signer[0] != 0,
                is_writable: is_writable[0] != 0,
            });
        }

        let data_size = u16::from_le_bytes(*data_len) as usize;
        let data = data_buffer[..data_size].to_vec();

        Ok(Action {
            is_initialized,
            proposal: Pubkey::new_from_array(*proposal),
            program_id: Pubkey::new_from_array(*program_id),
            accounts,
            data,
            executed,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Action::LEN];
        let (
            is_initialized_dst,
            proposal_dst,
            program_id_dst,
            accounts_len_dst,
            accounts_data_dst,
            data_len_dst,
            data_buffer_dst,
            executed_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 2, 512, 2, 442, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        proposal_dst.copy_from_slice(self.proposal.as_ref());
        program_id_dst.copy_from_slice(self.program_id.as_ref());
        *accounts_len_dst = (self.accounts.len() as u16).to_le_bytes();

        let account_size = 34;
        for (i, account) in self.accounts.iter().enumerate() {
            let start = i * account_size;
            let account_dst = array_mut_ref![accounts_data_dst, start, account_size];
            let (pubkey_dst, is_signer_dst, is_writable_dst) = 
                mut_array_refs![account_dst, 32, 1, 1];

            pubkey_dst.copy_from_slice(account.pubkey.as_ref());
            is_signer_dst[0] = account.is_signer as u8;
            is_writable_dst[0] = account.is_writable as u8;
        }

        *data_len_dst = (self.data.len() as u16).to_le_bytes();
        data_buffer_dst[..self.data.len()].copy_from_slice(&self.data);
        executed_dst[0] = self.executed as u8;
    }
}

impl Sealed for Queue {}
impl IsInitialized for Queue {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Queue {
    const LEN: usize = 512; // Fixed size buffer for actions

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Queue::LEN];
        let (
            is_initialized,
            proposal,
            execution_time,
            actions_len,
            actions_data,
            executed,
            canceled,
        ) = array_refs![src, 1, 32, 8, 2, 467, 1, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let executed = match executed[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let canceled = match canceled[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let actions_count = u16::from_le_bytes(*actions_len) as usize;
        let mut actions = Vec::with_capacity(actions_count);

        for i in 0..actions_count {
            let start = i * 32;
            let action_data = array_ref![actions_data, start, 32];
            actions.push(Pubkey::new_from_array(*action_data));
        }

        Ok(Queue {
            is_initialized,
            proposal: Pubkey::new_from_array(*proposal),
            execution_time: i64::from_le_bytes(*execution_time),
            actions,
            executed,
            canceled,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Queue::LEN];
        let (
            is_initialized_dst,
            proposal_dst,
            execution_time_dst,
            actions_len_dst,
            actions_data_dst,
            executed_dst,
            canceled_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 2, 467, 1, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        proposal_dst.copy_from_slice(self.proposal.as_ref());
        *execution_time_dst = self.execution_time.to_le_bytes();
        *actions_len_dst = (self.actions.len() as u16).to_le_bytes();

        for (i, action) in self.actions.iter().enumerate() {
            let start = i * 32;
            let action_dst = array_mut_ref![actions_data_dst, start, 32];
            action_dst.copy_from_slice(action.as_ref());
        }

        executed_dst[0] = self.executed as u8;
        canceled_dst[0] = self.canceled as u8;
    }
} 