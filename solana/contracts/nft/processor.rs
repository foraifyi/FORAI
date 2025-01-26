use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
    msg,
};

use crate::{
    state::{NFTMetadata, NFTHolder, NFTStatus, Pack},
    instruction::NFTInstruction,
    error::NFTError,
    event::NFTEvent,
};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = NFTInstruction::unpack(instruction_data)?;

        match instruction {
            NFTInstruction::InitializeCollection {
                name,
                symbol,
                uri,
                royalty_percentage,
                total_supply,
            } => {
                Self::process_initialize_collection(
                    program_id,
                    accounts,
                    name,
                    symbol,
                    uri,
                    royalty_percentage,
                    total_supply,
                )
            }
            NFTInstruction::MintNFT { amount } => {
                Self::process_mint(program_id, accounts, amount)
            }
            NFTInstruction::TransferNFT { amount } => {
                Self::process_transfer(program_id, accounts, amount)
            }
            NFTInstruction::LockNFT { amount } => {
                Self::process_lock(program_id, accounts, amount)
            }
            NFTInstruction::UnlockNFT { amount } => {
                Self::process_unlock(program_id, accounts, amount)
            }
            NFTInstruction::BurnNFT { amount } => {
                Self::process_burn(program_id, accounts, amount)
            }
            NFTInstruction::UpdateMetadata {
                name,
                uri,
                royalty_percentage,
            } => {
                Self::process_update_metadata(
                    program_id,
                    accounts,
                    name,
                    uri,
                    royalty_percentage,
                )
            }
        }
    }

    fn process_initialize_collection(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        name: [u8; 32],
        symbol: [u8; 8],
        uri: [u8; 128],
        royalty_percentage: u8,
        total_supply: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let creator_account = next_account_info(account_info_iter)?;
        let project_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !creator_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        if royalty_percentage > 100 {
            return Err(NFTError::InvalidRoyaltyPercentage.into());
        }

        let clock = Clock::get()?;
        let mut metadata = NFTMetadata::unpack_unchecked(&metadata_account.data.borrow())?;

        if metadata.is_initialized {
            return Err(NFTError::AlreadyInitialized.into());
        }

        metadata.is_initialized = true;
        metadata.creator = *creator_account.key;
        metadata.owner = *creator_account.key;
        metadata.name = name;
        metadata.symbol = symbol;
        metadata.uri = uri;
        metadata.status = NFTStatus::Active;
        metadata.royalty_percentage = royalty_percentage;
        metadata.total_supply = total_supply;
        metadata.current_supply = 0;
        metadata.project = *project_account.key;
        metadata.creation_time = clock.unix_timestamp;
        metadata.update_time = clock.unix_timestamp;

        NFTMetadata::pack(metadata, &mut metadata_account.data.borrow_mut())?;

        NFTEvent::CollectionCreated {
            metadata: metadata_account.key,
            creator: creator_account.key,
            name,
            symbol,
            total_supply,
        }.emit();

        Ok(())
    }

    fn process_mint(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let holder_account = next_account_info(account_info_iter)?;
        let creator_account = next_account_info(account_info_iter)?;
        let recipient_account = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id || holder_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !creator_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        let mut metadata = NFTMetadata::unpack(&metadata_account.data.borrow())?;
        if metadata.creator != *creator_account.key {
            return Err(NFTError::InvalidAuthority.into());
        }

        if metadata.current_supply.checked_add(amount).ok_or(NFTError::InvalidAmount)? > metadata.total_supply {
            return Err(NFTError::SupplyExceeded.into());
        }

        let clock = Clock::get()?;
        let mut holder = NFTHolder::unpack_unchecked(&holder_account.data.borrow())?;
        
        holder.owner = *recipient_account.key;
        holder.nft_mint = *metadata_account.key;
        holder.amount = amount;
        holder.locked_amount = 0;
        holder.acquisition_time = clock.unix_timestamp;

        metadata.current_supply = metadata.current_supply.checked_add(amount).unwrap();
        metadata.update_time = clock.unix_timestamp;

        NFTMetadata::pack(metadata, &mut metadata_account.data.borrow_mut())?;
        NFTHolder::pack(holder, &mut holder_account.data.borrow_mut())?;

        NFTEvent::TokensMinted {
            metadata: metadata_account.key,
            recipient: recipient_account.key,
            amount,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }

    fn process_transfer(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let sender_holder_account = next_account_info(account_info_iter)?;
        let recipient_holder_account = next_account_info(account_info_iter)?;
        let sender_account = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id 
            || sender_holder_account.owner != program_id 
            || recipient_holder_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !sender_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        let metadata = NFTMetadata::unpack(&metadata_account.data.borrow())?;
        let mut sender_holder = NFTHolder::unpack(&sender_holder_account.data.borrow())?;
        let mut recipient_holder = NFTHolder::unpack_unchecked(&recipient_holder_account.data.borrow())?;

        if sender_holder.owner != *sender_account.key {
            return Err(NFTError::InvalidAuthority.into());
        }

        let available_amount = sender_holder.amount.checked_sub(sender_holder.locked_amount)
            .ok_or(NFTError::InsufficientBalance)?;
        if amount > available_amount {
            return Err(NFTError::InsufficientBalance.into());
        }

        let clock = Clock::get()?;

        sender_holder.amount = sender_holder.amount.checked_sub(amount).unwrap();
        recipient_holder.owner = *recipient_holder_account.key;
        recipient_holder.nft_mint = *metadata_account.key;
        recipient_holder.amount = recipient_holder.amount.checked_add(amount).unwrap();
        recipient_holder.acquisition_time = clock.unix_timestamp;

        NFTHolder::pack(sender_holder, &mut sender_holder_account.data.borrow_mut())?;
        NFTHolder::pack(recipient_holder, &mut recipient_holder_account.data.borrow_mut())?;

        NFTEvent::TokensTransferred {
            metadata: metadata_account.key,
            from: sender_account.key,
            to: recipient_holder_account.key,
            amount,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }

    fn process_lock(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let holder_account = next_account_info(account_info_iter)?;
        let owner_account = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id || holder_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !owner_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        let metadata = NFTMetadata::unpack(&metadata_account.data.borrow())?;
        let mut holder = NFTHolder::unpack(&holder_account.data.borrow())?;

        if holder.owner != *owner_account.key {
            return Err(NFTError::InvalidAuthority.into());
        }

        let available_amount = holder.amount.checked_sub(holder.locked_amount)
            .ok_or(NFTError::InsufficientBalance)?;
        if amount > available_amount {
            return Err(NFTError::InsufficientBalance.into());
        }

        let clock = Clock::get()?;
        holder.locked_amount = holder.locked_amount.checked_add(amount).unwrap();

        NFTHolder::pack(holder, &mut holder_account.data.borrow_mut())?;

        NFTEvent::TokensLocked {
            metadata: metadata_account.key,
            owner: owner_account.key,
            amount,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }

    fn process_unlock(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let holder_account = next_account_info(account_info_iter)?;
        let owner_account = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id || holder_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !owner_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        let metadata = NFTMetadata::unpack(&metadata_account.data.borrow())?;
        let mut holder = NFTHolder::unpack(&holder_account.data.borrow())?;

        if holder.owner != *owner_account.key {
            return Err(NFTError::InvalidAuthority.into());
        }

        if amount > holder.locked_amount {
            return Err(NFTError::InsufficientBalance.into());
        }

        let clock = Clock::get()?;
        holder.locked_amount = holder.locked_amount.checked_sub(amount).unwrap();

        NFTHolder::pack(holder, &mut holder_account.data.borrow_mut())?;

        NFTEvent::TokensUnlocked {
            metadata: metadata_account.key,
            owner: owner_account.key,
            amount,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }

    fn process_burn(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let holder_account = next_account_info(account_info_iter)?;
        let owner_account = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id || holder_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !owner_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        let mut metadata = NFTMetadata::unpack(&metadata_account.data.borrow())?;
        let mut holder = NFTHolder::unpack(&holder_account.data.borrow())?;

        if holder.owner != *owner_account.key {
            return Err(NFTError::InvalidAuthority.into());
        }

        let available_amount = holder.amount.checked_sub(holder.locked_amount)
            .ok_or(NFTError::InsufficientBalance)?;
        if amount > available_amount {
            return Err(NFTError::InsufficientBalance.into());
        }

        let clock = Clock::get()?;
        
        holder.amount = holder.amount.checked_sub(amount).unwrap();
        metadata.current_supply = metadata.current_supply.checked_sub(amount).unwrap();
        metadata.update_time = clock.unix_timestamp;

        NFTMetadata::pack(metadata, &mut metadata_account.data.borrow_mut())?;
        NFTHolder::pack(holder, &mut holder_account.data.borrow_mut())?;

        NFTEvent::TokensBurned {
            metadata: metadata_account.key,
            owner: owner_account.key,
            amount,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }

    fn process_update_metadata(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        name: Option<[u8; 32]>,
        uri: Option<[u8; 128]>,
        royalty_percentage: Option<u8>,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let metadata_account = next_account_info(account_info_iter)?;
        let creator_account = next_account_info(account_info_iter)?;

        if metadata_account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !creator_account.is_signer {
            return Err(NFTError::InvalidAuthority.into());
        }

        let mut metadata = NFTMetadata::unpack(&metadata_account.data.borrow())?;

        if metadata.creator != *creator_account.key {
            return Err(NFTError::InvalidAuthority.into());
        }

        if let Some(name) = name {
            metadata.name = name;
        }

        if let Some(uri) = uri {
            metadata.uri = uri;
        }

        if let Some(percentage) = royalty_percentage {
            if percentage > 100 {
                return Err(NFTError::InvalidRoyaltyPercentage.into());
            }
            metadata.royalty_percentage = percentage;
        }

        let clock = Clock::get()?;
        metadata.update_time = clock.unix_timestamp;

        NFTMetadata::pack(metadata, &mut metadata_account.data.borrow_mut())?;

        NFTEvent::MetadataUpdated {
            metadata: metadata_account.key,
            updater: creator_account.key,
            timestamp: clock.unix_timestamp,
        }.emit();

        Ok(())
    }
} 