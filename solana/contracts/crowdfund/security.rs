use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    msg,
};
use crate::error::CrowdfundError;

pub struct SecurityChecks;

impl SecurityChecks {
    pub fn verify_account_ownership(
        account: &AccountInfo,
        expected_owner: &Pubkey,
    ) -> Result<(), ProgramError> {
        if account.owner != expected_owner {
            msg!("Invalid account owner");
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    pub fn verify_signer(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer {
            msg!("Missing required signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }

    pub fn verify_rent_exempt(
        account: &AccountInfo,
        rent: &Rent,
    ) -> Result<(), ProgramError> {
        if !rent.is_exempt(account.lamports(), account.data_len()) {
            msg!("Account not rent exempt");
            return Err(CrowdfundError::NotRentExempt.into());
        }
        Ok(())
    }

    pub fn verify_account_data_len(
        account: &AccountInfo,
        expected_len: usize,
    ) -> Result<(), ProgramError> {
        if account.data_len() != expected_len {
            msg!("Invalid account data length");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn verify_sufficient_funds(
        account: &AccountInfo,
        required_amount: u64,
    ) -> Result<(), ProgramError> {
        if account.lamports() < required_amount {
            msg!("Insufficient funds");
            return Err(CrowdfundError::InsufficientFunds.into());
        }
        Ok(())
    }

    pub fn verify_unique_accounts(accounts: &[&AccountInfo]) -> Result<(), ProgramError> {
        for (i, account) in accounts.iter().enumerate() {
            for other_account in accounts.iter().skip(i + 1) {
                if account.key == other_account.key {
                    msg!("Duplicate account detected");
                    return Err(CrowdfundError::DuplicateAccount.into());
                }
            }
        }
        Ok(())
    }

    pub fn verify_milestone_sequence(
        current_milestone: u8,
        requested_milestone: u8,
    ) -> Result<(), ProgramError> {
        if requested_milestone != current_milestone {
            msg!("Invalid milestone sequence");
            return Err(CrowdfundError::InvalidMilestoneSequence.into());
        }
        Ok(())
    }

    pub fn verify_milestone_amount(
        milestone_amount: u64,
        available_amount: u64,
    ) -> Result<(), ProgramError> {
        if milestone_amount > available_amount {
            msg!("Insufficient project funds for milestone");
            return Err(CrowdfundError::InsufficientProjectFunds.into());
        }
        Ok(())
    }

    pub fn verify_time_constraints(
        current_time: i64,
        start_time: i64,
        end_time: i64,
    ) -> Result<(), ProgramError> {
        if current_time < start_time {
            msg!("Project has not started");
            return Err(CrowdfundError::ProjectNotStarted.into());
        }
        if current_time > end_time {
            msg!("Project has ended");
            return Err(CrowdfundError::ProjectEnded.into());
        }
        Ok(())
    }
} 