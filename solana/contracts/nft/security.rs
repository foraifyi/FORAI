use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::error::NFTError;

pub struct SecurityChecks;

impl SecurityChecks {
    pub fn verify_account_owner(
        account: &AccountInfo,
        owner: &Pubkey,
    ) -> Result<(), ProgramError> {
        if account.owner != owner {
            msg!("Invalid account owner");
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    pub fn verify_signer(
        account: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !account.is_signer {
            msg!("Account must be a signer");
            return Err(NFTError::InvalidAuthority.into());
        }
        Ok(())
    }

    pub fn verify_rent_exempt(
        account: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if !Rent::get()?.is_exempt(account.lamports(), account.data_len()) {
            msg!("Account must be rent exempt");
            return Err(NFTError::NotRentExempt.into());
        }
        Ok(())
    }

    pub fn verify_account_data_len(
        account: &AccountInfo,
        min_len: usize,
    ) -> Result<(), ProgramError> {
        if account.data_len() < min_len {
            msg!("Account data length too small");
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
            return Err(NFTError::InsufficientBalance.into());
        }
        Ok(())
    }

    pub fn verify_unique_accounts(
        account1: &AccountInfo,
        account2: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if account1.key == account2.key {
            msg!("Accounts must be unique");
            return Err(NFTError::DuplicateAccount.into());
        }
        Ok(())
    }

    pub fn verify_royalty_percentage(
        percentage: u8,
    ) -> Result<(), ProgramError> {
        if percentage > 100 {
            msg!("Invalid royalty percentage");
            return Err(NFTError::InvalidRoyaltyPercentage.into());
        }
        Ok(())
    }

    pub fn verify_nft_amount(
        amount: u64,
        max_amount: u64,
    ) -> Result<(), ProgramError> {
        if amount == 0 || amount > max_amount {
            msg!("Invalid NFT amount");
            return Err(NFTError::InvalidAmount.into());
        }
        Ok(())
    }

    pub fn verify_authority(
        authority: &Pubkey,
        expected_authority: &Pubkey,
    ) -> Result<(), ProgramError> {
        if authority != expected_authority {
            msg!("Invalid authority");
            return Err(NFTError::InvalidAuthority.into());
        }
        Ok(())
    }

    pub fn verify_nft_status(
        is_active: bool,
    ) -> Result<(), ProgramError> {
        if !is_active {
            msg!("NFT is not active");
            return Err(NFTError::InvalidStatus.into());
        }
        Ok(())
    }

    pub fn verify_nft_not_locked(
        locked_amount: u64,
    ) -> Result<(), ProgramError> {
        if locked_amount > 0 {
            msg!("NFT is locked");
            return Err(NFTError::TokenLocked.into());
        }
        Ok(())
    }

    pub fn verify_nft_supply(
        current_supply: u64,
        total_supply: u64,
    ) -> Result<(), ProgramError> {
        if current_supply >= total_supply {
            msg!("NFT supply exceeded");
            return Err(NFTError::SupplyExceeded.into());
        }
        Ok(())
    }
} 