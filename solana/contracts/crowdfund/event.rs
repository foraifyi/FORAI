use solana_program::{
    account_info::AccountInfo,
    msg,
    pubkey::Pubkey,
};

pub enum CrowdfundEvent<'a> {
    ProjectCreated {
        project: &'a Pubkey,
        owner: &'a Pubkey,
        target_amount: u64,
        milestone_count: u8,
    },
    InvestmentMade {
        project: &'a Pubkey,
        investor: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    MilestoneCompleted {
        project: &'a Pubkey,
        milestone_index: u8,
        amount_released: u64,
        timestamp: i64,
    },
    RefundClaimed {
        project: &'a Pubkey,
        investor: &'a Pubkey,
        amount: u64,
    },
    ProjectCancelled {
        project: &'a Pubkey,
        current_amount: u64,
        timestamp: i64,
    },
    ProjectCompleted {
        project: &'a Pubkey,
        total_raised: u64,
        completion_time: i64,
    },
}

impl<'a> CrowdfundEvent<'a> {
    pub fn emit(&self) {
        match self {
            Self::ProjectCreated { project, owner, target_amount, milestone_count } => {
                msg!("Project Created: project={}, owner={}, target_amount={}, milestones={}", 
                    project, owner, target_amount, milestone_count);
            }
            Self::InvestmentMade { project, investor, amount, timestamp } => {
                msg!("Investment Made: project={}, investor={}, amount={}, time={}", 
                    project, investor, amount, timestamp);
            }
            Self::MilestoneCompleted { project, milestone_index, amount_released, timestamp } => {
                msg!("Milestone Completed: project={}, index={}, amount={}, time={}", 
                    project, milestone_index, amount_released, timestamp);
            }
            Self::RefundClaimed { project, investor, amount } => {
                msg!("Refund Claimed: project={}, investor={}, amount={}", 
                    project, investor, amount);
            }
            Self::ProjectCancelled { project, current_amount, timestamp } => {
                msg!("Project Cancelled: project={}, raised={}, time={}", 
                    project, current_amount, timestamp);
            }
            Self::ProjectCompleted { project, total_raised, completion_time } => {
                msg!("Project Completed: project={}, total_raised={}, time={}", 
                    project, total_raised, completion_time);
            }
        }
    }
} 