use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use std::convert::TryInto;
use std::mem::size_of;
use crate::state::Milestone;

#[derive(Debug)]
pub enum CrowdfundInstruction {
    /// Initialize a new project
    /// 
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[signer]` Project owner
    /// 2. `[]` Treasury account
    /// 3. `[]` System program
    InitializeProject {
        title: [u8; 32],
        description: [u8; 64],
        target_amount: u64,
        start_time: i64,
        end_time: i64,
    },

    /// Invest in a project
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[writable]` Investment account
    /// 2. `[signer]` Investor
    /// 3. `[writable]` Treasury account
    Invest {
        amount: u64,
    },

    /// Complete a milestone
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[signer]` Project owner
    /// 2. `[writable]` Treasury account
    CompleteMilestone {
        milestone_index: u8,
    },

    /// Claim refund for failed project
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[writable]` Investment account
    /// 2. `[signer]` Investor
    /// 3. `[writable]` Treasury account
    ClaimRefund,

    /// Cancel project (only by owner before funding ends)
    ///
    /// Accounts expected:
    /// 0. `[writable]` Project account
    /// 1. `[signer]` Project owner
    CancelProject,
}

impl CrowdfundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let title: [u8; 32] = rest[..32].try_into().unwrap();
                let description: [u8; 64] = rest[32..96].try_into().unwrap();
                let target_amount = rest[96..104].try_into().map(u64::from_le_bytes).unwrap();
                let start_time = rest[104..112].try_into().map(i64::from_le_bytes).unwrap();
                let end_time = rest[112..120].try_into().map(i64::from_le_bytes).unwrap();

                Self::InitializeProject {
                    title,
                    description,
                    target_amount,
                    start_time,
                    end_time,
                }
            }
            1 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::Invest { amount }
            }
            2 => {
                let milestone_index = rest[0];
                Self::CompleteMilestone { milestone_index }
            }
            3 => Self::ClaimRefund,
            4 => Self::CancelProject,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitializeProject {
                title,
                description,
                target_amount,
                start_time,
                end_time,
            } => {
                buf.push(0);
                buf.extend_from_slice(title);
                buf.extend_from_slice(description);
                buf.extend_from_slice(&target_amount.to_le_bytes());
                buf.extend_from_slice(&start_time.to_le_bytes());
                buf.extend_from_slice(&end_time.to_le_bytes());
            }
            Self::Invest { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::CompleteMilestone { milestone_index } => {
                buf.push(2);
                buf.push(*milestone_index);
            }
            Self::ClaimRefund => buf.push(3),
            Self::CancelProject => buf.push(4),
        }
        buf
    }
} 