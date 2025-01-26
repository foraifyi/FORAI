use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
    rent::Rent,
    msg,
};

use crate::{
    error::CrowdfundError,
    instruction::CrowdfundInstruction,
    state::{Project, ProjectStatus, Investment, Milestone},
    event::CrowdfundEvent,
    security::SecurityChecks,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CrowdfundInstruction::unpack(instruction_data)?;

        match instruction {
            CrowdfundInstruction::InitializeProject {
                title,
                description,
                target_amount,
                start_time,
                end_time,
                milestones,
            } => {
                Self::process_initialize_project(
                    accounts,
                    title,
                    description,
                    target_amount,
                    start_time,
                    end_time,
                    milestones,
                    program_id,
                )
            }
            CrowdfundInstruction::Invest { amount } => {
                Self::process_invest(accounts, amount)
            }
            CrowdfundInstruction::CompleteMilestone { milestone_index } => {
                Self::process_complete_milestone(accounts, milestone_index)
            }
            CrowdfundInstruction::ClaimRefund => {
                Self::process_claim_refund(accounts)
            }
            CrowdfundInstruction::CancelProject => {
                Self::process_cancel_project(accounts)
            }
        }
    }

    fn process_initialize_project(
        accounts: &[AccountInfo],
        title: [u8; 32],
        description: [u8; 64],
        target_amount: u64,
        start_time: i64,
        end_time: i64,
        milestones: Vec<Milestone>,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if project_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut project = Project::unpack_unchecked(&project_account.data.borrow())?;
        if project.is_initialized {
            return Err(CrowdfundError::AlreadyInitialized.into());
        }

        let clock = Clock::get()?;
        if start_time < clock.unix_timestamp {
            return Err(CrowdfundError::InvalidStatusTransition.into());
        }

        if end_time <= start_time {
            return Err(CrowdfundError::InvalidStatusTransition.into());
        }

        project.is_initialized = true;
        project.owner = *owner.key;
        project.title = title;
        project.description = description;
        project.target_amount = target_amount;
        project.current_amount = 0;
        project.start_time = start_time;
        project.end_time = end_time;
        project.milestones = milestones;
        project.current_milestone = 0;
        project.status = ProjectStatus::Active;
        project.treasury = *treasury.key;

        SecurityChecks::verify_account_ownership(project_account, program_id)?;
        SecurityChecks::verify_signer(owner)?;
        SecurityChecks::verify_rent_exempt(project_account, &Rent::get()?)?;
        SecurityChecks::verify_account_data_len(project_account, Project::LEN)?;
        SecurityChecks::verify_time_constraints(Clock::get()?.unix_timestamp, start_time, end_time)?;

        CrowdfundEvent::ProjectCreated {
            project: project_account.key,
            owner: owner.key,
            target_amount,
            milestone_count: milestones.len() as u8,
        }.emit();

        Project::pack(project, &mut project_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_invest(
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let investment_account = next_account_info(account_info_iter)?;
        let investor = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !investor.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut project = Project::unpack(&project_account.data.borrow())?;
        if project.status != ProjectStatus::Active {
            return Err(CrowdfundError::ProjectNotActive.into());
        }

        let clock = Clock::get()?;
        if clock.unix_timestamp >= project.end_time {
            return Err(CrowdfundError::FundingPeriodEnded.into());
        }

        SecurityChecks::verify_sufficient_funds(investor, amount)?;
        SecurityChecks::verify_unique_accounts(&[project_account, investment_account, investor, treasury])?;

        // Transfer investment amount
        **investor.try_borrow_mut_lamports()? -= amount;
        **treasury.try_borrow_mut_lamports()? += amount;

        // Record investment
        let investment = Investment {
            investor: *investor.key,
            project: *project_account.key,
            amount,
            timestamp: clock.unix_timestamp,
            is_refunded: false,
        };
        Investment::pack(investment, &mut investment_account.data.borrow_mut())?;

        // Update project state
        project.current_amount += amount;
        if project.current_amount >= project.target_amount {
            project.status = ProjectStatus::Funded;
            
            CrowdfundEvent::ProjectFunded {
                project: project_account.key,
                total_amount: project.current_amount,
            }.emit();
        }

        Project::pack(project, &mut project_account.data.borrow_mut())?;

        CrowdfundEvent::InvestmentMade {
            project: project_account.key,
            investor: investor.key,
            amount,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }

    fn process_complete_milestone(
        accounts: &[AccountInfo],
        milestone_index: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut project = Project::unpack(&project_account.data.borrow())?;
        if project.owner != *owner.key {
            return Err(CrowdfundError::InvalidAuthority.into());
        }

        if project.status != ProjectStatus::Funded {
            return Err(CrowdfundError::ProjectNotActive.into());
        }

        if milestone_index as usize >= project.milestones.len() {
            return Err(CrowdfundError::InvalidMilestoneIndex.into());
        }

        let milestone = &mut project.milestones[milestone_index as usize];
        if milestone.is_completed {
            return Err(CrowdfundError::MilestoneAlreadyCompleted.into());
        }

        let clock = Clock::get()?;
        milestone.completion_time = clock.unix_timestamp;
        milestone.is_completed = true;

        // Release milestone funds
        if !milestone.is_funds_released {
            **treasury.try_borrow_mut_lamports()? -= milestone.target_amount;
            **owner.try_borrow_mut_lamports()? += milestone.target_amount;
            milestone.is_funds_released = true;
        }

        // Check if project is completed
        if project.milestones.iter().all(|m| m.is_completed) {
            project.status = ProjectStatus::Completed;
            
            CrowdfundEvent::ProjectCompleted {
                project: project_account.key,
                completion_time: Clock::get()?.unix_timestamp,
            }.emit();
        }

        Project::pack(project, &mut project_account.data.borrow_mut())?;

        CrowdfundEvent::MilestoneCompleted {
            project: project_account.key,
            milestone_index,
            amount: milestone.target_amount,
        }.emit();

        Ok(())
    }

    fn process_claim_refund(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let investment_account = next_account_info(account_info_iter)?;
        let investor = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !investor.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let project = Project::unpack(&project_account.data.borrow())?;
        let mut investment = Investment::unpack(&investment_account.data.borrow())?;

        // Verify investment belongs to the investor
        if investment.investor != *investor.key {
            return Err(CrowdfundError::InvalidAuthority.into());
        }

        // Verify investment belongs to the project
        if investment.project != *project_account.key {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // Check if refund is allowed
        let clock = Clock::get()?;
        if clock.unix_timestamp < project.end_time {
            return Err(CrowdfundError::FundingPeriodEnded.into());
        }

        if project.status != ProjectStatus::Failed && project.status != ProjectStatus::Cancelled {
            return Err(CrowdfundError::ProjectNotActive.into());
        }

        if investment.is_refunded {
            return Err(CrowdfundError::RefundAlreadyClaimed.into());
        }

        // Process refund
        **treasury.try_borrow_mut_lamports()? -= investment.amount;
        **investor.try_borrow_mut_lamports()? += investment.amount;
        investment.is_refunded = true;

        Investment::pack(investment, &mut investment_account.data.borrow_mut())?;

        CrowdfundEvent::RefundClaimed {
            project: project_account.key,
            investor: investor.key,
            amount: investment.amount,
        }.emit();

        Ok(())
    }

    fn process_cancel_project(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut project = Project::unpack(&project_account.data.borrow())?;
        
        // Verify owner
        if project.owner != *owner.key {
            return Err(CrowdfundError::InvalidAuthority.into());
        }

        // Can only cancel active projects
        if project.status != ProjectStatus::Active {
            return Err(CrowdfundError::ProjectNotActive.into());
        }

        // Update project status
        project.status = ProjectStatus::Cancelled;
        Project::pack(project, &mut project_account.data.borrow_mut())?;

        CrowdfundEvent::ProjectCancelled {
            project: project_account.key,
            cancellation_time: Clock::get()?.unix_timestamp,
        }.emit();

        Ok(())
    }
} 