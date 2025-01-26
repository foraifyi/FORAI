use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum IncentiveInstruction {
    /// Initialize a new agent account
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New agent account
    /// 2. `[signer]` Agent authority
    /// 3. `[]` System program
    InitializeAgent,

    /// Reward an agent for completed task
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Agent account to reward
    /// 2. `[signer]` Program authority
    /// 3. `[writable]` Treasury account
    RewardAgent {
        amount: u64,
        reputation_increase: u64,
    },

    /// Update reputation score
    UpdateReputation {
        new_score: u64,
    },

    /// Penalize agent
    PenalizeAgent {
        amount: u64,
    },

    /// Added: Query reputation history
    GetReputationHistory {
        start_time: i64,
        end_time: i64,
    },

    /// Added: Set agent permissions
    SetAgentAuthority {
        new_authority: Pubkey,
    },

    /// Added: Suspend agent
    SuspendAgent {
        duration: i64,
    },

    /// Added: Reactivate agent
    ReactivateAgent,

    /// Added: Batch update reputation
    BatchUpdateReputation {
        agents: Vec<Pubkey>,
        scores: Vec<u64>,
    },

    /// Added: Set performance multiplier
    SetPerformanceMultiplier {
        multiplier: u16,
    },

    /// Added: Claim accumulated rewards
    ClaimAccumulatedRewards,

    /// Added: Apply for level upgrade
    RequestLevelUpgrade {
        target_level: u8,
    },
}

impl IncentiveInstruction {
    /// Unpack instruction from byte data
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => Self::InitializeAgent,
            1 => {
                let new_score = Self::unpack_u64(rest)?;
                Self::UpdateReputation { new_score }
            }
            2 => {
                let amount = Self::unpack_u64(rest)?;
                Self::RewardAgent { amount, reputation_increase: 0 }
            }
            3 => {
                let amount = Self::unpack_u64(rest)?;
                Self::PenalizeAgent { amount }
            }
            4 => {
                let (start_time, rest) = Self::unpack_i64(rest)?;
                let end_time = Self::unpack_i64(rest)?;
                Self::GetReputationHistory { start_time, end_time }
            }
            5 => {
                let new_authority = Self::unpack_pubkey(rest)?;
                Self::SetAgentAuthority { new_authority }
            }
            6 => {
                let duration = Self::unpack_i64(rest)?;
                Self::SuspendAgent { duration }
            }
            7 => Self::ReactivateAgent,
            8 => {
                let (agents, rest) = Self::unpack_pubkey_vec(rest)?;
                let scores = Self::unpack_u64_vec(rest)?;
                Self::BatchUpdateReputation { agents, scores }
            }
            9 => {
                let multiplier = Self::unpack_u16(rest)?;
                Self::SetPerformanceMultiplier { multiplier }
            }
            10 => Self::ClaimAccumulatedRewards,
            11 => {
                let target_level = Self::unpack_u8(rest)?;
                Self::RequestLevelUpgrade { target_level }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<u64, ProgramError> {
        let value = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(value)
    }

    fn unpack_i64(input: &[u8]) -> Result<(i64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (value_slice, rest) = input.split_at(8);
        let value = i64::from_le_bytes(value_slice.try_into().unwrap());
        Ok((value, rest))
    }

    fn unpack_u16(input: &[u8]) -> Result<u16, ProgramError> {
        let value = input
            .get(..2)
            .and_then(|slice| slice.try_into().ok())
            .map(u16::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(value)
    }

    fn unpack_u8(input: &[u8]) -> Result<u8, ProgramError> {
        let value = input
            .first()
            .copied()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(value)
    }

    fn unpack_pubkey(input: &[u8]) -> Result<Pubkey, ProgramError> {
        if input.len() < 32 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let value = Pubkey::new(&input[..32]);
        Ok(value)
    }

    fn unpack_pubkey_vec(input: &[u8]) -> Result<(Vec<Pubkey>, &[u8]), ProgramError> {
        let (len_slice, rest) = input.split_at(2);
        let len = u16::from_le_bytes(len_slice.try_into().unwrap()) as usize;
        
        let mut pubkeys = Vec::with_capacity(len);
        let mut current = rest;
        
        for _ in 0..len {
            if current.len() < 32 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let (key_slice, rest_slice) = current.split_at(32);
            pubkeys.push(Pubkey::new(key_slice));
            current = rest_slice;
        }
        
        Ok((pubkeys, current))
    }

    fn unpack_u64_vec(input: &[u8]) -> Result<Vec<u64>, ProgramError> {
        let (len_slice, rest) = input.split_at(2);
        let len = u16::from_le_bytes(len_slice.try_into().unwrap()) as usize;
        
        let mut values = Vec::with_capacity(len);
        let mut current = rest;
        
        for _ in 0..len {
            if current.len() < 8 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let (value_slice, rest_slice) = current.split_at(8);
            values.push(u64::from_le_bytes(value_slice.try_into().unwrap()));
            current = rest_slice;
        }
        
        Ok(values)
    }
} 