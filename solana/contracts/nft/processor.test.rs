use solana_program::{
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use crate::{
    processor::Processor,
    state::{NFTMetadata, NFTHolder, NFTStatus},
    instruction::NFTInstruction,
};

#[tokio::test]
async fn test_initialize_collection() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "nft",
        program_id,
        processor!(Processor::process),
    );

    // Create accounts
    let metadata_keypair = Keypair::new();
    let creator_keypair = Keypair::new();
    let project_keypair = Keypair::new();

    // Add accounts to program
    program_test.add_account(
        metadata_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTMetadata::LEN),
            data: vec![0; NFTMetadata::LEN],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create initialize collection instruction
    let name = *b"Test Collection                    ";
    let symbol = *b"TEST    ";
    let uri = *b"https://test.uri                                                                                                                        ";
    let royalty_percentage = 5;
    let total_supply = 1000;

    let instruction = NFTInstruction::InitializeCollection {
        name,
        symbol,
        uri,
        royalty_percentage,
        total_supply,
    };

    // Create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &creator_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify collection state
    let metadata_account = banks_client
        .get_account(metadata_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let metadata = NFTMetadata::unpack(&metadata_account.data).unwrap();
    assert!(metadata.is_initialized);
    assert_eq!(metadata.creator, creator_keypair.pubkey());
    assert_eq!(metadata.name, name);
    assert_eq!(metadata.symbol, symbol);
    assert_eq!(metadata.uri, uri);
    assert_eq!(metadata.status, NFTStatus::Active);
    assert_eq!(metadata.royalty_percentage, royalty_percentage);
    assert_eq!(metadata.total_supply, total_supply);
    assert_eq!(metadata.current_supply, 0);
    assert_eq!(metadata.project, project_keypair.pubkey());
}

#[tokio::test]
async fn test_mint_nft() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "nft",
        program_id,
        processor!(Processor::process),
    );

    // Create accounts
    let metadata_keypair = Keypair::new();
    let holder_keypair = Keypair::new();
    let creator_keypair = Keypair::new();
    let recipient_keypair = Keypair::new();

    // Initialize metadata account
    let mut metadata = NFTMetadata::default();
    metadata.is_initialized = true;
    metadata.creator = creator_keypair.pubkey();
    metadata.total_supply = 1000;
    
    let mut metadata_data = vec![0; NFTMetadata::LEN];
    NFTMetadata::pack(metadata, &mut metadata_data).unwrap();

    // Add accounts to program
    program_test.add_account(
        metadata_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTMetadata::LEN),
            data: metadata_data,
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        holder_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTHolder::LEN),
            data: vec![0; NFTHolder::LEN],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create mint instruction
    let amount = 100;
    let instruction = NFTInstruction::MintNFT { amount };

    // Create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &creator_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify holder state
    let holder_account = banks_client
        .get_account(holder_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let holder = NFTHolder::unpack(&holder_account.data).unwrap();
    assert_eq!(holder.owner, recipient_keypair.pubkey());
    assert_eq!(holder.nft_mint, metadata_keypair.pubkey());
    assert_eq!(holder.amount, amount);
    assert_eq!(holder.locked_amount, 0);

    // Verify metadata state
    let metadata_account = banks_client
        .get_account(metadata_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let metadata = NFTMetadata::unpack(&metadata_account.data).unwrap();
    assert_eq!(metadata.current_supply, amount);
}

#[tokio::test]
async fn test_transfer_nft() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "nft",
        program_id,
        processor!(Processor::process),
    );

    // Create accounts
    let metadata_keypair = Keypair::new();
    let sender_holder_keypair = Keypair::new();
    let recipient_holder_keypair = Keypair::new();
    let sender_keypair = Keypair::new();

    // Initialize metadata account
    let mut metadata = NFTMetadata::default();
    metadata.is_initialized = true;
    metadata.current_supply = 1000;
    
    let mut metadata_data = vec![0; NFTMetadata::LEN];
    NFTMetadata::pack(metadata, &mut metadata_data).unwrap();

    // Initialize sender holder account
    let mut sender_holder = NFTHolder::default();
    sender_holder.owner = sender_keypair.pubkey();
    sender_holder.amount = 500;
    
    let mut sender_holder_data = vec![0; NFTHolder::LEN];
    NFTHolder::pack(sender_holder, &mut sender_holder_data).unwrap();

    // Add accounts to program
    program_test.add_account(
        metadata_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTMetadata::LEN),
            data: metadata_data,
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        sender_holder_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTHolder::LEN),
            data: sender_holder_data,
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        recipient_holder_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTHolder::LEN),
            data: vec![0; NFTHolder::LEN],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create transfer instruction
    let amount = 100;
    let instruction = NFTInstruction::TransferNFT { amount };

    // Create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &sender_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify sender holder state
    let sender_holder_account = banks_client
        .get_account(sender_holder_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let sender_holder = NFTHolder::unpack(&sender_holder_account.data).unwrap();
    assert_eq!(sender_holder.amount, 400);

    // Verify recipient holder state
    let recipient_holder_account = banks_client
        .get_account(recipient_holder_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let recipient_holder = NFTHolder::unpack(&recipient_holder_account.data).unwrap();
    assert_eq!(recipient_holder.amount, 100);
}

#[tokio::test]
async fn test_lock_unlock_nft() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "nft",
        program_id,
        processor!(Processor::process),
    );

    // Create accounts
    let metadata_keypair = Keypair::new();
    let holder_keypair = Keypair::new();
    let owner_keypair = Keypair::new();

    // Initialize metadata account
    let mut metadata = NFTMetadata::default();
    metadata.is_initialized = true;
    metadata.current_supply = 1000;
    
    let mut metadata_data = vec![0; NFTMetadata::LEN];
    NFTMetadata::pack(metadata, &mut metadata_data).unwrap();

    // Initialize holder account
    let mut holder = NFTHolder::default();
    holder.owner = owner_keypair.pubkey();
    holder.amount = 500;
    holder.locked_amount = 0;
    
    let mut holder_data = vec![0; NFTHolder::LEN];
    NFTHolder::pack(holder, &mut holder_data).unwrap();

    // Add accounts to program
    program_test.add_account(
        metadata_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTMetadata::LEN),
            data: metadata_data,
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        holder_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTHolder::LEN),
            data: holder_data,
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test lock
    let lock_amount = 200;
    let lock_instruction = NFTInstruction::LockNFT { amount: lock_amount };

    let mut transaction = Transaction::new_with_payer(
        &[lock_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify locked state
    let holder_account = banks_client
        .get_account(holder_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let holder = NFTHolder::unpack(&holder_account.data).unwrap();
    assert_eq!(holder.locked_amount, lock_amount);

    // Test unlock
    let unlock_amount = 100;
    let unlock_instruction = NFTInstruction::UnlockNFT { amount: unlock_amount };

    let mut transaction = Transaction::new_with_payer(
        &[unlock_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify unlocked state
    let holder_account = banks_client
        .get_account(holder_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let holder = NFTHolder::unpack(&holder_account.data).unwrap();
    assert_eq!(holder.locked_amount, lock_amount - unlock_amount);
}

#[tokio::test]
async fn test_burn_nft() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "nft",
        program_id,
        processor!(Processor::process),
    );

    // Create accounts
    let metadata_keypair = Keypair::new();
    let holder_keypair = Keypair::new();
    let owner_keypair = Keypair::new();

    // Initialize metadata account
    let mut metadata = NFTMetadata::default();
    metadata.is_initialized = true;
    metadata.current_supply = 1000;
    
    let mut metadata_data = vec![0; NFTMetadata::LEN];
    NFTMetadata::pack(metadata, &mut metadata_data).unwrap();

    // Initialize holder account
    let mut holder = NFTHolder::default();
    holder.owner = owner_keypair.pubkey();
    holder.amount = 500;
    holder.locked_amount = 0;
    
    let mut holder_data = vec![0; NFTHolder::LEN];
    NFTHolder::pack(holder, &mut holder_data).unwrap();

    // Add accounts to program
    program_test.add_account(
        metadata_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTMetadata::LEN),
            data: metadata_data,
            owner: program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        holder_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTHolder::LEN),
            data: holder_data,
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create burn instruction
    let burn_amount = 200;
    let instruction = NFTInstruction::BurnNFT { amount: burn_amount };

    // Create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify holder state
    let holder_account = banks_client
        .get_account(holder_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let holder = NFTHolder::unpack(&holder_account.data).unwrap();
    assert_eq!(holder.amount, 300);

    // Verify metadata state
    let metadata_account = banks_client
        .get_account(metadata_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let metadata = NFTMetadata::unpack(&metadata_account.data).unwrap();
    assert_eq!(metadata.current_supply, 800);
}

#[tokio::test]
async fn test_update_metadata() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "nft",
        program_id,
        processor!(Processor::process),
    );

    // Create accounts
    let metadata_keypair = Keypair::new();
    let creator_keypair = Keypair::new();

    // Initialize metadata account
    let mut metadata = NFTMetadata::default();
    metadata.is_initialized = true;
    metadata.creator = creator_keypair.pubkey();
    metadata.name = *b"Old Name                           ";
    metadata.uri = *b"Old URI                                                                                                                            ";
    metadata.royalty_percentage = 5;
    
    let mut metadata_data = vec![0; NFTMetadata::LEN];
    NFTMetadata::pack(metadata, &mut metadata_data).unwrap();

    // Add accounts to program
    program_test.add_account(
        metadata_keypair.pubkey(),
        Account {
            lamports: Rent::default().minimum_balance(NFTMetadata::LEN),
            data: metadata_data,
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create update metadata instruction
    let new_name = *b"New Name                           ";
    let new_uri = *b"New URI                                                                                                                            ";
    let new_royalty_percentage = 10;

    let instruction = NFTInstruction::UpdateMetadata {
        name: Some(new_name),
        uri: Some(new_uri),
        royalty_percentage: Some(new_royalty_percentage),
    };

    // Create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &creator_keypair], recent_blockhash);

    banks_client.process_transaction(transaction).await.unwrap();

    // Verify metadata state
    let metadata_account = banks_client
        .get_account(metadata_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let metadata = NFTMetadata::unpack(&metadata_account.data).unwrap();
    assert_eq!(metadata.name, new_name);
    assert_eq!(metadata.uri, new_uri);
    assert_eq!(metadata.royalty_percentage, new_royalty_percentage);
} 