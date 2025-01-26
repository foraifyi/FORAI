use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum GovernanceInstruction {
    /// Initialize governance config
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New config account
    /// 2. `[signer]` Authority
    /// 3. `[]` Governance token
    /// 4. `[]` Proposal program
    InitializeConfig {
        voting_delay: i64,
        voting_period: i64,
        quorum_votes: u64,
        timelock_delay: i64,
        guardian: Option<Pubkey>,
    },

    /// Queue proposal actions
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New queue account
    /// 3. `[signer]` Authority
    /// 4. `[]` Proposal account
    QueueActions {
        execution_time: i64,
        actions: Vec<Pubkey>,
    },

    /// Execute queued actions
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Queue account
    /// 2. `[writable]` Action accounts (variable)
    /// 3. `[signer]` Authority
    ExecuteActions,

    /// Cancel queued actions
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Queue account
    /// 2. `[signer]` Authority or Guardian
    CancelActions,

    /// Set guardian
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[signer]` Authority
    SetGuardian {
        new_guardian: Option<Pubkey>,
    },
}

impl GovernanceInstruction {
    /// Unpacks a byte buffer into a GovernanceInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (voting_delay, rest) = Self::unpack_i64(rest)?;
                let (voting_period, rest) = Self::unpack_i64(rest)?;
                let (quorum_votes, rest) = Self::unpack_u64(rest)?;
                let (timelock_delay, rest) = Self::unpack_i64(rest)?;
                let (has_guardian, guardian_rest) = Self::unpack_bool(rest)?;
                let guardian = if has_guardian {
                    let (guardian, _) = Self::unpack_pubkey(guardian_rest)?;
                    Some(guardian)
                } else {
                    None
                };
                Self::InitializeConfig {
                    voting_delay,
                    voting_period,
                    quorum_votes,
                    timelock_delay,
                    guardian,
                }
            }
            1 => {
                let (execution_time, rest) = Self::unpack_i64(rest)?;
                let (actions_len, mut actions_data) = Self::unpack_u16(rest)?;
                let mut actions = Vec::with_capacity(actions_len as usize);
                for _ in 0..actions_len {
                    let (action, rest) = Self::unpack_pubkey(actions_data)?;
                    actions.push(action);
                    actions_data = rest;
                }
                Self::QueueActions {
                    execution_time,
                    actions,
                }
            }
            2 => Self::ExecuteActions,
            3 => Self::CancelActions,
            4 => {
                let (has_guardian, guardian_rest) = Self::unpack_bool(rest)?;
                let new_guardian = if has_guardian {
                    let (guardian, _) = Self::unpack_pubkey(guardian_rest)?;
                    Some(guardian)
                } else {
                    None
                };
                Self::SetGuardian { new_guardian }
            }
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

    fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if input.len() < 2 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (amount, rest) = input.split_at(2);
        let amount = amount
            .try_into()
            .map(u16::from_le_bytes)
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

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < 32 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (key, rest) = input.split_at(32);
        let pubkey = Pubkey::new(key);
        Ok((pubkey, rest))
    }
} 