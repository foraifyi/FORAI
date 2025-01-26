use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum VotingPowerInstruction {
    /// Initialize voting power config
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New config account
    /// 2. `[signer]` Authority
    /// 3. `[]` Governance token
    InitializeConfig {
        delegation_enabled: bool,
        checkpoint_enabled: bool,
        checkpoint_interval: i64,
    },

    /// Update voting power
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` Voting power account
    /// 3. `[signer]` Authority
    /// 4. `[]` Owner
    /// 5. `[]` Token account
    UpdateVotingPower {
        amount: u64,
    },

    /// Delegate voting power
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` Delegator voting power account
    /// 3. `[writable]` Delegate voting power account
    /// 4. `[signer]` Delegator
    DelegateVotingPower {
        amount: u64,
    },

    /// Create checkpoint
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New checkpoint account
    /// 3. `[signer]` Authority
    CreateCheckpoint,

    /// Get voting power at checkpoint
    /// 
    /// Accounts expected:
    /// 1. `[]` Config account
    /// 2. `[]` Checkpoint account
    /// 3. `[]` Account to query
    GetVotingPowerAtCheckpoint,
}

impl VotingPowerInstruction {
    /// Unpacks a byte buffer into a VotingPowerInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (delegation_enabled, rest) = Self::unpack_bool(rest)?;
                let (checkpoint_enabled, rest) = Self::unpack_bool(rest)?;
                let (checkpoint_interval, _) = Self::unpack_i64(rest)?;
                Self::InitializeConfig {
                    delegation_enabled,
                    checkpoint_enabled,
                    checkpoint_interval,
                }
            }
            1 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::UpdateVotingPower { amount }
            }
            2 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                Self::DelegateVotingPower { amount }
            }
            3 => Self::CreateCheckpoint,
            4 => Self::GetVotingPowerAtCheckpoint,
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

    fn unpack_bool(input: &[u8]) -> Result<(bool, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bool_byte, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok((*bool_byte != 0, rest))
    }
} 