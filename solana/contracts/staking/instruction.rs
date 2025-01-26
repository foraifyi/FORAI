use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum StakingInstruction {
    /// Initialize stake pool
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New stake pool account
    /// 2. `[signer]` Authority
    /// 3. `[]` Token mint
    /// 4. `[writable]` Token vault
    /// 5. `[]` System program
    /// 6. `[]` Token program
    InitializePool {
        min_stake_duration: i64,
        reward_rate: u16,
        early_unstake_penalty: u16,
    },

    /// Stake tokens
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Stake pool
    /// 2. `[writable]` New stake account
    /// 3. `[signer]` Owner
    /// 4. `[writable]` Owner token account
    /// 5. `[writable]` Pool token vault
    /// 6. `[]` Token program
    Stake {
        amount: u64,
        lock_duration: i64,
    },

    /// Unstake tokens
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Stake pool
    /// 2. `[writable]` Stake account
    /// 3. `[signer]` Owner
    /// 4. `[writable]` Owner token account
    /// 5. `[writable]` Pool token vault
    /// 6. `[]` Token program
    Unstake,

    /// Claim rewards
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Stake account
    /// 2. `[signer]` Owner
    /// 3. `[writable]` Owner token account
    /// 4. `[writable]` Reward distribution account
    /// 5. `[]` Token program
    ClaimRewards,

    /// Start reward distribution
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Stake pool
    /// 2. `[writable]` New reward distribution account
    /// 3. `[signer]` Authority
    /// 4. `[writable]` Authority token account
    /// 5. `[writable]` Pool token vault
    /// 6. `[]` Token program
    StartDistribution {
        total_rewards: u64,
        duration: i64,
    },
}

impl StakingInstruction {
    /// Unpacks a byte buffer into a StakingInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (min_stake_duration, rest) = Self::unpack_i64(rest)?;
                let (reward_rate, rest) = Self::unpack_u16(rest)?;
                let (early_unstake_penalty, _) = Self::unpack_u16(rest)?;
                Self::InitializePool {
                    min_stake_duration,
                    reward_rate,
                    early_unstake_penalty,
                }
            }
            1 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (lock_duration, _) = Self::unpack_i64(rest)?;
                Self::Stake { amount, lock_duration }
            }
            2 => Self::Unstake,
            3 => Self::ClaimRewards,
            4 => {
                let (total_rewards, rest) = Self::unpack_u64(rest)?;
                let (duration, _) = Self::unpack_i64(rest)?;
                Self::StartDistribution {
                    total_rewards,
                    duration,
                }
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
} 