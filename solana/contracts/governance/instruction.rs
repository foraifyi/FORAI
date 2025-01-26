use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
    msg,
};
use std::convert::TryInto;
use std::mem::size_of;

use crate::state::VoteType;

#[derive(Debug)]
pub enum GovernanceInstruction {
    /// Initialize a new governance configuration
    /// 
    /// Accounts expected:
    /// 0. `[writable]` Governance config account
    /// 1. `[signer]` Admin account
    /// 2. `[]` System program
    InitializeGovernance {
        name: [u8; 32],
        voting_delay: u64,
        voting_period: u64,
        quorum_votes: u64,
        timelock_delay: u64,
        proposal_threshold: u64,
        vote_threshold: u8,
    },

    /// Create a new proposal
    ///
    /// Accounts expected:
    /// 0. `[writable]` Proposal account
    /// 1. `[signer]` Proposer account
    /// 2. `[]` Governance config account
    CreateProposal {
        title: [u8; 64],
        description: [u8; 256],
        vote_type: VoteType,
        choices: Vec<[u8; 32]>,
    },

    /// Cast a vote on a proposal
    ///
    /// Accounts expected:
    /// 0. `[writable]` Vote account
    /// 1. `[writable]` Proposal account
    /// 2. `[signer]` Voter account
    CastVote {
        vote_weight: u64,
        support: bool,
        choices: [u8; 8],
    },

    /// Cancel a proposal
    ///
    /// Accounts expected:
    /// 0. `[writable]` Proposal account
    /// 1. `[signer]` Proposer account
    CancelProposal,

    /// Queue a successful proposal for execution
    ///
    /// Accounts expected:
    /// 0. `[writable]` Proposal account
    /// 1. `[signer]` Admin account
    QueueProposal,

    /// Execute a queued proposal
    ///
    /// Accounts expected:
    /// 0. `[writable]` Proposal account
    /// 1. `[signer]` Admin account
    ExecuteProposal,

    /// Update governance parameters
    ///
    /// Accounts expected:
    /// 0. `[writable]` Governance config account
    /// 1. `[signer]` Admin account
    UpdateGovernance {
        voting_delay: Option<u64>,
        voting_period: Option<u64>,
        quorum_votes: Option<u64>,
        timelock_delay: Option<u64>,
        proposal_threshold: Option<u64>,
        vote_threshold: Option<u8>,
    },
}

impl GovernanceInstruction {
    /// Unpacks a byte buffer into a GovernanceInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let name: [u8; 32] = rest[..32].try_into().unwrap();
                let voting_delay = rest[32..40].try_into().map(u64::from_le_bytes).unwrap();
                let voting_period = rest[40..48].try_into().map(u64::from_le_bytes).unwrap();
                let quorum_votes = rest[48..56].try_into().map(u64::from_le_bytes).unwrap();
                let timelock_delay = rest[56..64].try_into().map(u64::from_le_bytes).unwrap();
                let proposal_threshold = rest[64..72].try_into().map(u64::from_le_bytes).unwrap();
                let vote_threshold = rest[72];

                Self::InitializeGovernance {
                    name,
                    voting_delay,
                    voting_period,
                    quorum_votes,
                    timelock_delay,
                    proposal_threshold,
                    vote_threshold,
                }
            }
            1 => {
                let title: [u8; 64] = rest[..64].try_into().unwrap();
                let description: [u8; 256] = rest[64..320].try_into().unwrap();
                let vote_type = VoteType::from_u8(rest[320]).ok_or(ProgramError::InvalidInstructionData)?;
                let choice_count = rest[321] as usize;
                let mut choices = Vec::with_capacity(choice_count);
                
                for i in 0..choice_count {
                    let start = 322 + i * 32;
                    let end = start + 32;
                    let choice: [u8; 32] = rest[start..end].try_into().unwrap();
                    choices.push(choice);
                }

                Self::CreateProposal {
                    title,
                    description,
                    vote_type,
                    choices,
                }
            }
            2 => {
                let vote_weight = rest[..8].try_into().map(u64::from_le_bytes).unwrap();
                let support = rest[8] != 0;
                let choices: [u8; 8] = rest[9..17].try_into().unwrap();

                Self::CastVote {
                    vote_weight,
                    support,
                    choices,
                }
            }
            3 => Self::CancelProposal,
            4 => Self::QueueProposal,
            5 => Self::ExecuteProposal,
            6 => {
                let mut pos = 0;
                let voting_delay = if rest[pos] == 1 {
                    pos += 1;
                    let delay = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    pos += 8;
                    Some(delay)
                } else {
                    pos += 1;
                    None
                };

                let voting_period = if rest[pos] == 1 {
                    pos += 1;
                    let period = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    pos += 8;
                    Some(period)
                } else {
                    pos += 1;
                    None
                };

                let quorum_votes = if rest[pos] == 1 {
                    pos += 1;
                    let votes = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    pos += 8;
                    Some(votes)
                } else {
                    pos += 1;
                    None
                };

                let timelock_delay = if rest[pos] == 1 {
                    pos += 1;
                    let delay = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    pos += 8;
                    Some(delay)
                } else {
                    pos += 1;
                    None
                };

                let proposal_threshold = if rest[pos] == 1 {
                    pos += 1;
                    let threshold = rest[pos..pos+8].try_into().map(u64::from_le_bytes).unwrap();
                    pos += 8;
                    Some(threshold)
                } else {
                    pos += 1;
                    None
                };

                let vote_threshold = if rest[pos] == 1 {
                    pos += 1;
                    Some(rest[pos])
                } else {
                    None
                };

                Self::UpdateGovernance {
                    voting_delay,
                    voting_period,
                    quorum_votes,
                    timelock_delay,
                    proposal_threshold,
                    vote_threshold,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitializeGovernance {
                name,
                voting_delay,
                voting_period,
                quorum_votes,
                timelock_delay,
                proposal_threshold,
                vote_threshold,
            } => {
                buf.push(0);
                buf.extend_from_slice(name);
                buf.extend_from_slice(&voting_delay.to_le_bytes());
                buf.extend_from_slice(&voting_period.to_le_bytes());
                buf.extend_from_slice(&quorum_votes.to_le_bytes());
                buf.extend_from_slice(&timelock_delay.to_le_bytes());
                buf.extend_from_slice(&proposal_threshold.to_le_bytes());
                buf.push(*vote_threshold);
            }
            Self::CreateProposal {
                title,
                description,
                vote_type,
                choices,
            } => {
                buf.push(1);
                buf.extend_from_slice(title);
                buf.extend_from_slice(description);
                buf.push(*vote_type as u8);
                buf.push(choices.len() as u8);
                for choice in choices {
                    buf.extend_from_slice(choice);
                }
            }
            Self::CastVote {
                vote_weight,
                support,
                choices,
            } => {
                buf.push(2);
                buf.extend_from_slice(&vote_weight.to_le_bytes());
                buf.push(*support as u8);
                buf.extend_from_slice(choices);
            }
            Self::CancelProposal => {
                buf.push(3);
            }
            Self::QueueProposal => {
                buf.push(4);
            }
            Self::ExecuteProposal => {
                buf.push(5);
            }
            Self::UpdateGovernance {
                voting_delay,
                voting_period,
                quorum_votes,
                timelock_delay,
                proposal_threshold,
                vote_threshold,
            } => {
                buf.push(6);
                
                // Pack voting_delay
                if let Some(delay) = voting_delay {
                    buf.push(1);
                    buf.extend_from_slice(&delay.to_le_bytes());
                } else {
                    buf.push(0);
                }

                // Pack voting_period
                if let Some(period) = voting_period {
                    buf.push(1);
                    buf.extend_from_slice(&period.to_le_bytes());
                } else {
                    buf.push(0);
                }

                // Pack quorum_votes
                if let Some(votes) = quorum_votes {
                    buf.push(1);
                    buf.extend_from_slice(&votes.to_le_bytes());
                } else {
                    buf.push(0);
                }

                // Pack timelock_delay
                if let Some(delay) = timelock_delay {
                    buf.push(1);
                    buf.extend_from_slice(&delay.to_le_bytes());
                } else {
                    buf.push(0);
                }

                // Pack proposal_threshold
                if let Some(threshold) = proposal_threshold {
                    buf.push(1);
                    buf.extend_from_slice(&threshold.to_le_bytes());
                } else {
                    buf.push(0);
                }

                // Pack vote_threshold
                if let Some(threshold) = vote_threshold {
                    buf.push(1);
                    buf.push(*threshold);
                } else {
                    buf.push(0);
                }
            }
        }
        buf
    }
} 