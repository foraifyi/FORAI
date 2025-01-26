use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

#[derive(Debug)]
pub struct GovernanceConfig {
    pub is_initialized: bool,
    pub authority: Pubkey,
    pub governance_token: Pubkey,
    pub proposal_program: Pubkey,
    pub voting_delay: i64,          // Delay before voting starts
    pub voting_period: i64,         // Duration of voting period
    pub quorum_votes: u64,          // Minimum votes required
    pub timelock_delay: i64,        // Delay before execution
    pub guardian: Option<Pubkey>,   // Optional emergency guardian
    pub is_active: bool,
}

#[derive(Debug)]
pub struct Proposal {
    pub is_initialized: bool,
    pub proposer: Pubkey,
    pub start_block: u64,
    pub end_block: u64,
    pub description_uri: [u8; 128], // IPFS URI for proposal details
    pub executable_uri: [u8; 128],  // IPFS URI for executable code
    pub for_votes: u64,
    pub against_votes: u64,
    pub executed: bool,
    pub canceled: bool,
    pub execution_time: u64,
}

#[derive(Debug)]
pub struct Vote {
    pub is_initialized: bool,
    pub voter: Pubkey,
    pub proposal: Pubkey,
    pub support: bool,
    pub votes: u64,
    pub timestamp: i64,
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
    const LEN: usize = 122; // 1 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 33 + 1

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, GovernanceConfig::LEN];
        let (
            is_initialized,
            authority,
            governance_token,
            proposal_program,
            voting_delay,
            voting_period,
            quorum_votes,
            timelock_delay,
            guardian_data,
            is_active,
        ) = array_refs![src, 1, 32, 32, 32, 8, 8, 8, 8, 33, 1];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let is_active = match is_active[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let guardian = if guardian_data[0] == 0 {
            None
        } else {
            Some(Pubkey::new_from_array(*array_ref![guardian_data, 1, 32]))
        };

        Ok(GovernanceConfig {
            is_initialized,
            authority: Pubkey::new_from_array(*authority),
            governance_token: Pubkey::new_from_array(*governance_token),
            proposal_program: Pubkey::new_from_array(*proposal_program),
            voting_delay: i64::from_le_bytes(*voting_delay),
            voting_period: i64::from_le_bytes(*voting_period),
            quorum_votes: u64::from_le_bytes(*quorum_votes),
            timelock_delay: i64::from_le_bytes(*timelock_delay),
            guardian,
            is_active,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GovernanceConfig::LEN];
        let (
            is_initialized_dst,
            authority_dst,
            governance_token_dst,
            proposal_program_dst,
            voting_delay_dst,
            voting_period_dst,
            quorum_votes_dst,
            timelock_delay_dst,
            guardian_dst,
            is_active_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 8, 8, 8, 33, 1];

        is_initialized_dst[0] = self.is_initialized as u8;
        authority_dst.copy_from_slice(self.authority.as_ref());
        governance_token_dst.copy_from_slice(self.governance_token.as_ref());
        proposal_program_dst.copy_from_slice(self.proposal_program.as_ref());
        *voting_delay_dst = self.voting_delay.to_le_bytes();
        *voting_period_dst = self.voting_period.to_le_bytes();
        *quorum_votes_dst = self.quorum_votes.to_le_bytes();
        *timelock_delay_dst = self.timelock_delay.to_le_bytes();
        
        if let Some(guardian) = self.guardian {
            guardian_dst[0] = 1;
            guardian_dst[1..].copy_from_slice(guardian.as_ref());
        } else {
            guardian_dst[0] = 0;
        }

        is_active_dst[0] = self.is_active as u8;
    }
}

impl Sealed for Proposal {}
impl IsInitialized for Proposal {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Proposal {
    const LEN: usize = 315; // 1 + 32 + 8 + 8 + 128 + 128 + 8 + 8 + 1 + 1 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Proposal::LEN];
        let (
            is_initialized,
            proposer,
            start_block,
            end_block,
            description_uri,
            executable_uri,
            for_votes,
            against_votes,
            executed,
            canceled,
            execution_time,
        ) = array_refs![src, 1, 32, 8, 8, 128, 128, 8, 8, 1, 1, 8];

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

        Ok(Proposal {
            is_initialized,
            proposer: Pubkey::new_from_array(*proposer),
            start_block: u64::from_le_bytes(*start_block),
            end_block: u64::from_le_bytes(*end_block),
            description_uri: *description_uri,
            executable_uri: *executable_uri,
            for_votes: u64::from_le_bytes(*for_votes),
            against_votes: u64::from_le_bytes(*against_votes),
            executed,
            canceled,
            execution_time: u64::from_le_bytes(*execution_time),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Proposal::LEN];
        let (
            is_initialized_dst,
            proposer_dst,
            start_block_dst,
            end_block_dst,
            description_uri_dst,
            executable_uri_dst,
            for_votes_dst,
            against_votes_dst,
            executed_dst,
            canceled_dst,
            execution_time_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8, 128, 128, 8, 8, 1, 1, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        proposer_dst.copy_from_slice(self.proposer.as_ref());
        *start_block_dst = self.start_block.to_le_bytes();
        *end_block_dst = self.end_block.to_le_bytes();
        description_uri_dst.copy_from_slice(&self.description_uri);
        executable_uri_dst.copy_from_slice(&self.executable_uri);
        *for_votes_dst = self.for_votes.to_le_bytes();
        *against_votes_dst = self.against_votes.to_le_bytes();
        executed_dst[0] = self.executed as u8;
        canceled_dst[0] = self.canceled as u8;
        *execution_time_dst = self.execution_time.to_le_bytes();
    }
}

impl Sealed for Vote {}
impl IsInitialized for Vote {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Vote {
    const LEN: usize = 52; // 1 + 32 + 32 + 1 + 8 + 8

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Vote::LEN];
        let (
            is_initialized,
            voter,
            proposal,
            support,
            votes,
            timestamp,
        ) = array_refs![src, 1, 32, 32, 1, 8, 8];

        let is_initialized = match is_initialized[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let support = match support[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Vote {
            is_initialized,
            voter: Pubkey::new_from_array(*voter),
            proposal: Pubkey::new_from_array(*proposal),
            support,
            votes: u64::from_le_bytes(*votes),
            timestamp: i64::from_le_bytes(*timestamp),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Vote::LEN];
        let (
            is_initialized_dst,
            voter_dst,
            proposal_dst,
            support_dst,
            votes_dst,
            timestamp_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 1, 8, 8];

        is_initialized_dst[0] = self.is_initialized as u8;
        voter_dst.copy_from_slice(self.voter.as_ref());
        proposal_dst.copy_from_slice(self.proposal.as_ref());
        support_dst[0] = self.support as u8;
        *votes_dst = self.votes.to_le_bytes();
        *timestamp_dst = self.timestamp.to_le_bytes();
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