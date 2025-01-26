use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::Pack,
    sysvar::{clock::Clock, Sysvar},
};

use crate::{
    state::{Project, Investment},
    error::CrowdfundError,
    instruction::CrowdfundInstruction,
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
            CrowdfundInstruction::InitializeProject { target_amount, deadline, milestone_count } => {
                Self::process_initialize_project(program_id, accounts, target_amount, deadline, milestone_count)
            }
            CrowdfundInstruction::Invest { amount } => {
                Self::process_invest(program_id, accounts, amount)
            }
            CrowdfundInstruction::ReleaseMilestone => {
                Self::process_release_milestone(program_id, accounts)
            }
        }
    }

    fn process_initialize_project(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        target_amount: u64,
        deadline: i64,
        milestone_count: u8,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;

        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut project = Project::unpack_unchecked(&project_account.data.borrow())?;
        if project.is_initialized {
            return Err(CrowdfundError::AlreadyInitialized.into());
        }

        project.is_initialized = true;
        project.owner = *owner.key;
        project.treasury = *treasury.key;
        project.target_amount = target_amount;
        project.current_amount = 0;
        project.deadline = deadline;
        project.is_completed = false;
        project.milestone_count = milestone_count;
        project.current_milestone = 0;

        Project::pack(project, &mut project_account.data.borrow_mut())?;
        Ok(())
    }

    fn process_invest(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let investor = next_account_info(account_info_iter)?;
        let investment_account = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        // Verify investor signature
        if !investor.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load project data
        let mut project = Project::unpack(&project_account.data.borrow())?;
        
        // Check project is still active
        if project.is_completed {
            return Err(CrowdfundError::ProjectCompleted.into());
        }
        if clock.unix_timestamp >= project.deadline {
            return Err(CrowdfundError::DeadlinePassed.into());
        }

        // Verify treasury account
        if project.treasury != *treasury.key {
            return Err(CrowdfundError::TreasuryMismatch.into());
        }

        // Check investment amount
        if amount == 0 {
            return Err(CrowdfundError::InvalidInvestmentAmount.into());
        }

        // Check investor has enough funds
        if investor.lamports() < amount {
            return Err(CrowdfundError::InsufficientFunds.into());
        }

        // Create investment record
        let mut investment = Investment::unpack_unchecked(&investment_account.data.borrow())?;
        if investment.is_initialized {
            return Err(CrowdfundError::InvestmentExists.into());
        }

        // Transfer funds
        **investor.lamports.borrow_mut() -= amount;
        **treasury.lamports.borrow_mut() += amount;

        // Update project state
        project.current_amount += amount;
        Project::pack(project, &mut project_account.data.borrow_mut())?;

        // Initialize investment record
        investment.is_initialized = true;
        investment.investor = *investor.key;
        investment.project = *project_account.key;
        investment.amount = amount;
        investment.timestamp = clock.unix_timestamp;
        Investment::pack(investment, &mut investment_account.data.borrow_mut())?;

        Ok(())
    }

    fn process_release_milestone(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let project_account = next_account_info(account_info_iter)?;
        let owner = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;
        let clock = Clock::get()?;

        // Verify project owner
        if !owner.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load project data
        let mut project = Project::unpack(&project_account.data.borrow())?;
        
        // Verify owner
        if project.owner != *owner.key {
            return Err(CrowdfundError::InvalidProjectOwner.into());
        }

        // Verify treasury
        if project.treasury != *treasury.key {
            return Err(CrowdfundError::TreasuryMismatch.into());
        }

        // Check project status
        if project.is_completed {
            return Err(CrowdfundError::ProjectCompleted.into());
        }
        if project.current_amount < project.target_amount {
            return Err(CrowdfundError::NotFullyFunded.into());
        }

        // Calculate milestone payment
        let milestone_payment = project.target_amount / (project.milestone_count as u64);
        if treasury.lamports() < milestone_payment {
            return Err(CrowdfundError::InsufficientFunds.into());
        }

        // Transfer milestone payment
        **treasury.lamports.borrow_mut() -= milestone_payment;
        **owner.lamports.borrow_mut() += milestone_payment;

        // Update project state
        project.current_milestone += 1;
        if project.current_milestone >= project.milestone_count {
            project.is_completed = true;
        }

        Project::pack(project, &mut project_account.data.borrow_mut())?;
        Ok(())
    }
} 