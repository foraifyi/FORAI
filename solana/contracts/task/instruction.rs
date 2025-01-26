use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum TaskInstruction {
    /// Initialize task pool
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New task pool account
    /// 2. `[signer]` Authority
    /// 3. `[]` System program
    InitializeTaskPool {
        min_stake_required: u64,
        min_performance_score: u32,
        task_timeout: i64,
        reward_amount: u64,
        penalty_amount: u64,
    },

    /// Create new task
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Task pool account
    /// 2. `[writable]` New task account
    /// 3. `[signer]` Task creator
    CreateTask {
        priority: u8,
        reward_multiplier: u16,
        specification_uri: [u8; 128],
    },

    /// Accept task assignment
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Task account
    /// 2. `[signer]` Agent account
    /// 3. `[]` Agent ranking account
    AcceptTask,

    /// Submit task result
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Task account
    /// 2. `[signer]` Agent account
    /// 3. `[writable]` Agent ranking account
    SubmitResult {
        result_uri: [u8; 128],
    },

    /// Review task result
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Task account
    /// 2. `[writable]` Agent account
    /// 3. `[signer]` Task creator or Authority
    ReviewTask {
        approved: bool,
        feedback: [u8; 128],
    },
}

impl TaskInstruction {
    /// Unpacks a byte buffer into a TaskInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (min_stake_required, rest) = Self::unpack_u64(rest)?;
                let (min_performance_score, rest) = Self::unpack_u32(rest)?;
                let (task_timeout, rest) = Self::unpack_i64(rest)?;
                let (reward_amount, rest) = Self::unpack_u64(rest)?;
                let (penalty_amount, _) = Self::unpack_u64(rest)?;
                Self::InitializeTaskPool {
                    min_stake_required,
                    min_performance_score,
                    task_timeout,
                    reward_amount,
                    penalty_amount,
                }
            }
            1 => {
                let (priority, rest) = Self::unpack_u8(rest)?;
                let (reward_multiplier, rest) = Self::unpack_u16(rest)?;
                let specification_uri = Self::unpack_fixed_bytes(rest)?;
                Self::CreateTask {
                    priority,
                    reward_multiplier,
                    specification_uri,
                }
            }
            2 => Self::AcceptTask,
            3 => {
                let result_uri = Self::unpack_fixed_bytes(rest)?;
                Self::SubmitResult { result_uri }
            }
            4 => {
                let (approved, rest) = Self::unpack_bool(rest)?;
                let feedback = Self::unpack_fixed_bytes(rest)?;
                Self::ReviewTask { approved, feedback }
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

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok((input[0], &input[1..]))
    }

    fn unpack_bool(input: &[u8]) -> Result<(bool, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        match input[0] {
            0 => Ok((false, &input[1..])),
            1 => Ok((true, &input[1..])),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    fn unpack_fixed_bytes(input: &[u8]) -> Result<[u8; 128], ProgramError> {
        if input.len() < 128 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let bytes: [u8; 128] = input[..128].try_into().unwrap();
        Ok(bytes)
    }
} 