use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum TokenLockInstruction {
    /// Initialize lock config
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New config account
    /// 2. `[signer]` Authority
    /// 3. `[]` Token mint
    /// 4. `[]` System program
    InitializeConfig {
        min_lock_duration: i64,
        max_lock_duration: i64,
        unlock_fee: u16,
        early_unlock_penalty: u16,
    },

    /// Lock tokens
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New lock account
    /// 3. `[signer]` Owner
    /// 4. `[writable]` Owner token account
    /// 5. `[]` Token program
    LockTokens {
        amount: u64,
        duration: i64,
    },

    /// Request unlock
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Lock account
    /// 2. `[writable]` New unlock request account
    /// 3. `[signer]` Owner
    /// 4. `[]` Config account
    RequestUnlock {
        amount: u64,
        is_early: bool,
    },

    /// Process unlock
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Lock account
    /// 2. `[writable]` Unlock request account
    /// 3. `[signer]` Authority
    /// 4. `[writable]` Owner token account
    /// 5. `[]` Token program
    ProcessUnlock,

    /// Cancel unlock request
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Lock account
    /// 2. `[writable]` Unlock request account
    /// 3. `[signer]` Owner
    CancelUnlock,
}

impl TokenLockInstruction {
    /// Unpacks a byte buffer into a TokenLockInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (min_lock_duration, rest) = Self::unpack_i64(rest)?;
                let (max_lock_duration, rest) = Self::unpack_i64(rest)?;
                let (unlock_fee, rest) = Self::unpack_u16(rest)?;
                let (early_unlock_penalty, _) = Self::unpack_u16(rest)?;
                Self::InitializeConfig {
                    min_lock_duration,
                    max_lock_duration,
                    unlock_fee,
                    early_unlock_penalty,
                }
            }
            1 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (duration, _) = Self::unpack_i64(rest)?;
                Self::LockTokens { amount, duration }
            }
            2 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (is_early, _) = Self::unpack_bool(rest)?;
                Self::RequestUnlock { amount, is_early }
            }
            3 => Self::ProcessUnlock,
            4 => Self::CancelUnlock,
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
} 