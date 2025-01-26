use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use crate::error::CrowdfundError;

pub struct SecurityChecks;

impl SecurityChecks {
    pub fn verify_account_ownership(
        account: &AccountInfo,
        expected_owner: &Pubkey,
    ) -> ProgramResult {
        if account.owner != expected_owner {
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    pub fn verify_signer(account: &AccountInfo) -> ProgramResult {
        if !account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }

    pub fn verify_rent_exempt(
        account: &AccountInfo,
        rent: &Rent,
    ) -> ProgramResult {
        if !rent.is_exempt(account.lamports(), account.data_len()) {
            return Err(CrowdfundError::NotRentExempt.into());
        }
        Ok(())
    }

    pub fn verify_account_data_len(
        account: &AccountInfo,
        expected_len: usize,
    ) -> ProgramResult {
        if account.data_len() != expected_len {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn verify_sufficient_funds(
        account: &AccountInfo,
        required_amount: u64,
    ) -> ProgramResult {
        if account.lamports() < required_amount {
            return Err(CrowdfundError::InsufficientFunds.into());
        }
        Ok(())
    }

    pub fn verify_unique_accounts(accounts: &[&AccountInfo]) -> ProgramResult {
        for (i, account1) in accounts.iter().enumerate() {
            for account2 in accounts.iter().skip(i + 1) {
                if account1.key == account2.key {
                    return Err(CrowdfundError::DuplicateAccount.into());
                }
            }
        }
        Ok(())
    }

    pub fn verify_milestone_sequence(
        milestone_index: u8,
        current_milestone: u8,
    ) -> ProgramResult {
        if milestone_index != current_milestone {
            return Err(CrowdfundError::InvalidMilestoneSequence.into());
        }
        Ok(())
    }

    pub fn verify_milestone_amount(
        milestone_amount: u64,
        total_raised: u64,
    ) -> ProgramResult {
        if milestone_amount > total_raised {
            return Err(CrowdfundError::InsufficientProjectFunds.into());
        }
        Ok(())
    }

    pub fn verify_time_constraints(
        current_time: i64,
        start_time: i64,
        end_time: i64,
    ) -> ProgramResult {
        if current_time < start_time {
            return Err(CrowdfundError::ProjectNotStarted.into());
        }
        if current_time > end_time {
            return Err(CrowdfundError::ProjectEnded.into());
        }
        Ok(())
    }
} 