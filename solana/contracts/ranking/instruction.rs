use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum RankingInstruction {
    /// Initialize ranking system
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New ranking config account
    /// 2. `[signer]` Authority
    /// 3. `[]` System program
    InitializeRanking {
        min_stake_amount: u64,
        performance_period: i64,
        reward_rate: u16,
        penalty_rate: u16,
    },

    /// Register new AI agent
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Ranking config
    /// 2. `[writable]` New agent account
    /// 3. `[signer]` Agent owner
    RegisterAgent {
        stake_amount: u64,
    },

    /// Submit performance score
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Agent account
    /// 2. `[writable]` Performance record
    /// 3. `[signer]` Reviewer
    /// 4. `[]` Task account
    SubmitPerformance {
        score: u32,
        feedback_uri: [u8; 128],
    },

    /// Update agent ranking
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Agent account
    /// 2. `[signer]` Authority
    UpdateRanking {
        new_score: u32,
    },

    /// Claim rewards
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Agent account
    /// 2. `[writable]` Owner account
    /// 3. `[signer]` Owner
    ClaimRewards,
}

impl RankingInstruction {
    /// Unpacks a byte buffer into a RankingInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (min_stake_amount, rest) = Self::unpack_u64(rest)?;
                let (performance_period, rest) = Self::unpack_i64(rest)?;
                let (reward_rate, rest) = Self::unpack_u16(rest)?;
                let (penalty_rate, _) = Self::unpack_u16(rest)?;
                Self::InitializeRanking {
                    min_stake_amount,
                    performance_period,
                    reward_rate,
                    penalty_rate,
                }
            }
            1 => {
                let (stake_amount, _) = Self::unpack_u64(rest)?;
                Self::RegisterAgent { stake_amount }
            }
            2 => {
                let (score, rest) = Self::unpack_u32(rest)?;
                let feedback_uri = Self::unpack_fixed_bytes(rest)?;
                Self::SubmitPerformance {
                    score,
                    feedback_uri,
                }
            }
            3 => {
                let (new_score, _) = Self::unpack_u32(rest)?;
                Self::UpdateRanking { new_score }
            }
            4 => Self::ClaimRewards,
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

    fn unpack_u32(input: &[u8]) -> Result<(u32, &[u8]), ProgramError> {
        if input.len() < 4 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (amount, rest) = input.split_at(4);
        let amount = amount
            .try_into()
            .map(u32::from_le_bytes)
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

    fn unpack_fixed_bytes(input: &[u8]) -> Result<[u8; 128], ProgramError> {
        if input.len() < 128 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let bytes: [u8; 128] = input[..128].try_into().unwrap();
        Ok(bytes)
    }
} 