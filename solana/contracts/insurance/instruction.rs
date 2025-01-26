use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
    msg,
};
use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
pub enum InsuranceInstruction {
    /// Initialize a new insurance pool
    /// 
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[signer]` Admin account
    /// 2. `[]` Treasury account
    /// 3. `[]` System program
    InitializePool {
        name: [u8; 32],
        min_capital_requirement: u64,
        coverage_ratio: u8,
        premium_rate: u8,
        claim_period: u64,
    },

    /// Add capital to the insurance pool
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[writable]` Treasury account
    /// 2. `[signer]` Provider account
    AddCapital {
        amount: u64,
    },

    /// Create a new insurance policy
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[writable]` Policy account
    /// 2. `[writable]` Treasury account
    /// 3. `[signer]` Insured account
    CreatePolicy {
        coverage_amount: u64,
        duration: u64,
    },

    /// Submit an insurance claim
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[writable]` Policy account
    /// 2. `[writable]` Claim account
    /// 3. `[signer]` Claimant account
    SubmitClaim {
        amount: u64,
        evidence: [u8; 128],
    },

    /// Process an insurance claim
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[writable]` Policy account
    /// 2. `[writable]` Claim account
    /// 3. `[writable]` Treasury account
    /// 4. `[signer]` Admin account
    ProcessClaim {
        approved: bool,
    },

    /// Update pool parameters
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[signer]` Admin account
    UpdatePool {
        coverage_ratio: Option<u8>,
        premium_rate: Option<u8>,
        claim_period: Option<u64>,
        min_capital_requirement: Option<u64>,
    },

    /// Pause/unpause the insurance pool
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[signer]` Admin account
    SetPoolStatus {
        paused: bool,
    },

    /// Withdraw capital from the insurance pool
    ///
    /// Accounts expected:
    /// 0. `[writable]` Pool account
    /// 1. `[writable]` Treasury account
    /// 2. `[signer]` Admin account
    WithdrawCapital {
        amount: u64,
    },
}

impl InsuranceInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let name: [u8; 32] = rest[..32].try_into().unwrap();
                let min_capital_requirement = rest[32..40].try_into().map(u64::from_le_bytes).unwrap();
                let coverage_ratio = rest[40];
                let premium_rate = rest[41];
                let claim_period = rest[42..50].try_into().map(u64::from_le_bytes).unwrap();

                Self::InitializePool {
                    name,
                    min_capital_requirement,
                    coverage_ratio,
                    premium_rate,
                    claim_period,
                }
            }
            1 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::AddCapital { amount }
            }
            2 => {
                let coverage_amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                let duration = rest[8..16].try_into().map(u64::from_le_bytes).unwrap();
                Self::CreatePolicy {
                    coverage_amount,
                    duration,
                }
            }
            3 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                let evidence: [u8; 128] = rest[8..136].try_into().unwrap();
                Self::SubmitClaim {
                    amount,
                    evidence,
                }
            }
            4 => {
                let approved = rest[0] != 0;
                Self::ProcessClaim { approved }
            }
            5 => {
                let mut pos = 0;
                let coverage_ratio = if rest[pos] == 1 {
                    pos += 1;
                    Some(rest[pos])
                } else {
                    pos += 1;
                    None
                };
                pos += 1;

                let premium_rate = if rest[pos] == 1 {
                    pos += 1;
                    Some(rest[pos])
                } else {
                    pos += 1;
                    None
                };
                pos += 1;

                let claim_period = if rest[pos] == 1 {
                    pos += 1;
                    let period = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    pos += 8;
                    Some(period)
                } else {
                    pos += 1;
                    None
                };

                let min_capital_requirement = if rest[pos] == 1 {
                    pos += 1;
                    let requirement = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    Some(requirement)
                } else {
                    None
                };

                Self::UpdatePool {
                    coverage_ratio,
                    premium_rate,
                    claim_period,
                    min_capital_requirement,
                }
            }
            6 => {
                let paused = rest[0] != 0;
                Self::SetPoolStatus { paused }
            }
            7 => {
                let amount = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                Self::WithdrawCapital { amount }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitializePool {
                name,
                min_capital_requirement,
                coverage_ratio,
                premium_rate,
                claim_period,
            } => {
                buf.push(0);
                buf.extend_from_slice(name);
                buf.extend_from_slice(&min_capital_requirement.to_le_bytes());
                buf.push(*coverage_ratio);
                buf.push(*premium_rate);
                buf.extend_from_slice(&claim_period.to_le_bytes());
            }
            Self::AddCapital { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::CreatePolicy {
                coverage_amount,
                duration,
            } => {
                buf.push(2);
                buf.extend_from_slice(&coverage_amount.to_le_bytes());
                buf.extend_from_slice(&duration.to_le_bytes());
            }
            Self::SubmitClaim { amount, evidence } => {
                buf.push(3);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(evidence);
            }
            Self::ProcessClaim { approved } => {
                buf.push(4);
                buf.push(*approved as u8);
            }
            Self::UpdatePool {
                coverage_ratio,
                premium_rate,
                claim_period,
                min_capital_requirement,
            } => {
                buf.push(5);
                
                // Pack coverage_ratio
                if let Some(ratio) = coverage_ratio {
                    buf.push(1);
                    buf.push(*ratio);
                } else {
                    buf.push(0);
                }

                // Pack premium_rate
                if let Some(rate) = premium_rate {
                    buf.push(1);
                    buf.push(*rate);
                } else {
                    buf.push(0);
                }

                // Pack claim_period
                if let Some(period) = claim_period {
                    buf.push(1);
                    buf.extend_from_slice(&period.to_le_bytes());
                } else {
                    buf.push(0);
                }

                // Pack min_capital_requirement
                if let Some(requirement) = min_capital_requirement {
                    buf.push(1);
                    buf.extend_from_slice(&requirement.to_le_bytes());
                } else {
                    buf.push(0);
                }
            }
            Self::SetPoolStatus { paused } => {
                buf.push(6);
                buf.push(*paused as u8);
            }
            Self::WithdrawCapital { amount } => {
                buf.push(7);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
        }
        buf
    }
} 