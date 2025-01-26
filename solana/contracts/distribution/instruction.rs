use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum DistributionInstruction {
    /// Initialize distribution config
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New config account
    /// 2. `[signer]` Authority
    /// 3. `[]` Token mint
    /// 4. `[writable]` Token vault
    /// 5. `[]` Token program
    InitializeConfig {
        distribution_rate: u64,
        min_epoch_duration: i64,
    },

    /// Add recipient
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New recipient account
    /// 3. `[signer]` Authority
    /// 4. `[]` Owner
    /// 5. `[]` Owner token account
    AddRecipient {
        allocation: u16,
    },

    /// Start distribution round
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New round account
    /// 3. `[signer]` Authority
    /// 4. `[writable]` Token vault
    StartRound {
        amount: u64,
        duration: i64,
    },

    /// Claim distribution
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` Round account
    /// 3. `[writable]` Recipient account
    /// 4. `[signer]` Owner
    /// 5. `[writable]` Owner token account
    /// 6. `[writable]` Token vault
    /// 7. `[]` Token program
    ClaimDistribution,

    /// Finalize round
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` Round account
    /// 3. `[signer]` Authority
    FinalizeRound,
}

impl DistributionInstruction {
    /// Unpacks a byte buffer into a DistributionInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (distribution_rate, rest) = Self::unpack_u64(rest)?;
                let (min_epoch_duration, _) = Self::unpack_i64(rest)?;
                Self::InitializeConfig {
                    distribution_rate,
                    min_epoch_duration,
                }
            }
            1 => {
                let (allocation, _) = Self::unpack_u16(rest)?;
                Self::AddRecipient { allocation }
            }
            2 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (duration, _) = Self::unpack_i64(rest)?;
                Self::StartRound { amount, duration }
            }
            3 => Self::ClaimDistribution,
            4 => Self::FinalizeRound,
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
} 