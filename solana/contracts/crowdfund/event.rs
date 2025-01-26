use solana_program::{
    account_info::AccountInfo,
    msg,
    pubkey::Pubkey,
};

pub enum CrowdfundEvent<'a> {
    // Project lifecycle events
    ProjectCreated {
        project: &'a Pubkey,
        owner: &'a Pubkey,
        target_amount: u64,
        milestone_count: u8,
    },
    ProjectFunded {
        project: &'a Pubkey,
        total_amount: u64,
    },
    ProjectCompleted {
        project: &'a Pubkey,
        completion_time: i64,
    },
    ProjectCancelled {
        project: &'a Pubkey,
        cancellation_time: i64,
    },
    ProjectFailed {
        project: &'a Pubkey,
        failure_time: i64,
        current_amount: u64,
    },

    // Investment events
    InvestmentMade {
        project: &'a Pubkey,
        investor: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    InvestmentRefunded {
        project: &'a Pubkey,
        investor: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },

    // Milestone events
    MilestoneCreated {
        project: &'a Pubkey,
        milestone_index: u8,
        target_amount: u64,
    },
    MilestoneCompleted {
        project: &'a Pubkey,
        milestone_index: u8,
        amount: u64,
    },
    MilestoneFundsReleased {
        project: &'a Pubkey,
        milestone_index: u8,
        amount: u64,
        recipient: &'a Pubkey,
    },

    // Treasury events
    TreasuryInitialized {
        treasury: &'a Pubkey,
        admin: &'a Pubkey,
    },
    TreasuryFundsDeposited {
        treasury: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    TreasuryFundsWithdrawn {
        treasury: &'a Pubkey,
        amount: u64,
        recipient: &'a Pubkey,
        timestamp: i64,
    },

    // Security events
    SecurityCheckFailed {
        project: &'a Pubkey,
        check_type: &'a str,
        reason: &'a str,
    },
    UnauthorizedAccess {
        account: &'a Pubkey,
        required_authority: &'a Pubkey,
    },
}

impl<'a> CrowdfundEvent<'a> {
    pub fn emit(&self) {
        match self {
            // Project lifecycle events
            Self::ProjectCreated { project, owner, target_amount, milestone_count } => {
                msg!("Project Created: project={}, owner={}, target_amount={}, milestones={}", 
                    project, owner, target_amount, milestone_count);
            }
            Self::ProjectFunded { project, total_amount } => {
                msg!("Project Funded: project={}, total_amount={}", 
                    project, total_amount);
            }
            Self::ProjectCompleted { project, completion_time } => {
                msg!("Project Completed: project={}, time={}", 
                    project, completion_time);
            }
            Self::ProjectCancelled { project, cancellation_time } => {
                msg!("Project Cancelled: project={}, time={}", 
                    project, cancellation_time);
            }
            Self::ProjectFailed { project, failure_time, current_amount } => {
                msg!("Project Failed: project={}, time={}, raised_amount={}", 
                    project, failure_time, current_amount);
            }

            // Investment events
            Self::InvestmentMade { project, investor, amount, timestamp } => {
                msg!("Investment Made: project={}, investor={}, amount={}, time={}", 
                    project, investor, amount, timestamp);
            }
            Self::InvestmentRefunded { project, investor, amount, timestamp } => {
                msg!("Investment Refunded: project={}, investor={}, amount={}, time={}", 
                    project, investor, amount, timestamp);
            }

            // Milestone events
            Self::MilestoneCreated { project, milestone_index, target_amount } => {
                msg!("Milestone Created: project={}, index={}, target_amount={}", 
                    project, milestone_index, target_amount);
            }
            Self::MilestoneCompleted { project, milestone_index, amount } => {
                msg!("Milestone Completed: project={}, index={}, amount={}", 
                    project, milestone_index, amount);
            }
            Self::MilestoneFundsReleased { project, milestone_index, amount, recipient } => {
                msg!("Milestone Funds Released: project={}, index={}, amount={}, recipient={}", 
                    project, milestone_index, amount, recipient);
            }

            // Treasury events
            Self::TreasuryInitialized { treasury, admin } => {
                msg!("Treasury Initialized: treasury={}, admin={}", 
                    treasury, admin);
            }
            Self::TreasuryFundsDeposited { treasury, amount, timestamp } => {
                msg!("Treasury Funds Deposited: treasury={}, amount={}, time={}", 
                    treasury, amount, timestamp);
            }
            Self::TreasuryFundsWithdrawn { treasury, amount, recipient, timestamp } => {
                msg!("Treasury Funds Withdrawn: treasury={}, amount={}, recipient={}, time={}", 
                    treasury, amount, recipient, timestamp);
            }

            // Security events
            Self::SecurityCheckFailed { project, check_type, reason } => {
                msg!("Security Check Failed: project={}, check={}, reason={}", 
                    project, check_type, reason);
            }
            Self::UnauthorizedAccess { account, required_authority } => {
                msg!("Unauthorized Access: account={}, required_authority={}", 
                    account, required_authority);
            }
        }
    }
} 