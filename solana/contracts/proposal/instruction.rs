use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum ProposalInstruction {
    /// Initialize proposal config
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New config account
    /// 2. `[signer]` Authority
    /// 3. `[]` Voting power program
    /// 4. `[]` System program
    InitializeConfig {
        min_voting_power: u64,
        voting_delay: i64,
        voting_period: i64,
        quorum_votes: u64,
    },

    /// Create proposal
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New proposal account
    /// 3. `[signer]` Proposer
    /// 4. `[]` Voting power account
    /// 5. `[writable]` Voting checkpoint account
    CreateProposal {
        title: [u8; 32],
        description_url: [u8; 64],
        execution_time: i64,
    },

    /// Cast vote
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Proposal account
    /// 2. `[writable]` New vote account
    /// 3. `[signer]` Voter
    /// 4. `[]` Voter power account
    /// 5. `[]` Voting checkpoint
    CastVote {
        support: u8,
    },

    /// Cancel proposal
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Proposal account
    /// 2. `[signer]` Proposer
    CancelProposal,

    /// Execute proposal
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Proposal account
    /// 2. `[signer]` Authority
    ExecuteProposal,
}

impl ProposalInstruction {
    /// Unpacks a byte buffer into a ProposalInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (min_voting_power, rest) = Self::unpack_u64(rest)?;
                let (voting_delay, rest) = Self::unpack_i64(rest)?;
                let (voting_period, rest) = Self::unpack_i64(rest)?;
                let (quorum_votes, _) = Self::unpack_u64(rest)?;
                Self::InitializeConfig {
                    min_voting_power,
                    voting_delay,
                    voting_period,
                    quorum_votes,
                }
            }
            1 => {
                let (title, rest) = Self::unpack_fixed_bytes::<32>(rest)?;
                let (description_url, rest) = Self::unpack_fixed_bytes::<64>(rest)?;
                let (execution_time, _) = Self::unpack_i64(rest)?;
                Self::CreateProposal {
                    title,
                    description_url,
                    execution_time,
                }
            }
            2 => {
                let (support, _) = Self::unpack_u8(rest)?;
                Self::CastVote { support }
            }
            3 => Self::CancelProposal,
            4 => Self::ExecuteProposal,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .try_into()
            .map(u64::from_le_bytes)
            .or(Err(ProgramError::InvalidInstructionData))?;
        Ok((amount, rest))
    }

    fn unpack_i64(input: &[u8]) -> Result<(i64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .try_into()
            .map(i64::from_le_bytes)
            .or(Err(ProgramError::InvalidInstructionData))?;
        Ok((amount, rest))
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (value, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok((*value, rest))
    }

    fn unpack_fixed_bytes<const N: usize>(input: &[u8]) -> Result<([u8; N], &[u8]), ProgramError> {
        if input.len() < N {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = input.split_at(N);
        let array = bytes
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok((array, rest))
    }
} 