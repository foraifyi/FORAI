use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_program,
    sysvar::clock::Clock,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use crate::{
    processor::Processor,
    state::{Project, ProjectStatus, Investment},
    instruction::CrowdfundInstruction,
};

#[tokio::test]
async fn test_initialize_project() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let treasury = Keypair::new();

    // Set initial state
    let space = Project::LEN;
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: vec![0; space],
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create initialize project instruction
    let title = *b"Test Project................................";
    let description = *b"Test Description............................................";
    let target_amount = 3000000;
    let start_time = 1000;
    let end_time = 2000;

    let instruction = CrowdfundInstruction::InitializeProject {
        title,
        description,
        target_amount,
        start_time,
        end_time,
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(project_keypair.pubkey(), false),
                AccountMeta::new_readonly(owner.pubkey(), true),
                AccountMeta::new_readonly(treasury.pubkey(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: instruction.pack(),
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);

    // Process transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify project state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let project = Project::unpack(&project_account.data).unwrap();
    assert!(project.is_initialized);
    assert_eq!(project.owner, owner.pubkey());
    assert_eq!(project.title, title);
    assert_eq!(project.description, description);
    assert_eq!(project.target_amount, target_amount);
    assert_eq!(project.current_amount, 0);
    assert_eq!(project.start_time, start_time);
    assert_eq!(project.end_time, end_time);
    assert_eq!(project.status, ProjectStatus::Active);
}

#[tokio::test]
async fn test_invest() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let investment_keypair = Keypair::new();
    let investor = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project account
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = Pubkey::new_unique();
    project.target_amount = 5000000;
    project.status = ProjectStatus::Active;
    project.start_time = 0;
    project.end_time = 9999999999;

    let space = Project::LEN;
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    let mut project_data = vec![0; space];
    Project::pack(project, &mut project_data).unwrap();

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: project_data,
            owner: program_id,
            ..Account::default()
        },
    );

    // Add investor funds
    program_test.add_account(
        investor.pubkey(),
        Account {
            lamports: 1000000,
            data: vec![],
            owner: system_program::id(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create invest instruction
    let investment_amount = 500000;
    let instruction = CrowdfundInstruction::Invest {
        amount: investment_amount,
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(project_keypair.pubkey(), false),
                AccountMeta::new(investment_keypair.pubkey(), false),
                AccountMeta::new(investor.pubkey(), true),
                AccountMeta::new(treasury.pubkey(), false),
            ],
            data: instruction.pack(),
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &investor], recent_blockhash);

    // Process transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify investment
    let investment_account = banks_client
        .get_account(investment_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let investment = Investment::unpack(&investment_account.data).unwrap();
    assert_eq!(investment.investor, investor.pubkey());
    assert_eq!(investment.amount, investment_amount);
    assert!(!investment.is_refunded);

    // Verify project state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let updated_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(updated_project.current_amount, investment_amount);
}

#[tokio::test]
async fn test_complete_milestone() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project account
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Funded;
    project.current_amount = 3000000;
    project.current_milestone = 0;

    let space = Project::LEN;
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    let mut project_data = vec![0; space];
    Project::pack(project, &mut project_data).unwrap();

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: project_data,
            owner: program_id,
            ..Account::default()
        },
    );

    // Add treasury funds
    program_test.add_account(
        treasury.pubkey(),
        Account {
            lamports: 3000000,
            data: vec![],
            owner: system_program::id(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create complete milestone instruction
    let instruction = CrowdfundInstruction::CompleteMilestone {
        milestone_index: 0,
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(project_keypair.pubkey(), false),
                AccountMeta::new(owner.pubkey(), true),
                AccountMeta::new(treasury.pubkey(), false),
            ],
            data: instruction.pack(),
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);

    // Process transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify project state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let updated_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(updated_project.current_milestone, 1);
}

#[tokio::test]
async fn test_claim_refund() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let investment_keypair = Keypair::new();
    let investor = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project account
    let mut project = Project::default();
    project.is_initialized = true;
    project.status = ProjectStatus::Failed;
    project.end_time = 0; // Project has ended

    let space = Project::LEN;
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    let mut project_data = vec![0; space];
    Project::pack(project, &mut project_data).unwrap();

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: project_data,
            owner: program_id,
            ..Account::default()
        },
    );

    // Initialize investment account
    let investment = Investment {
        investor: investor.pubkey(),
        project: project_keypair.pubkey(),
        amount: 500000,
        timestamp: 0,
        is_refunded: false,
    };

    let investment_space = Investment::LEN;
    let investment_lamports = rent.minimum_balance(investment_space);

    let mut investment_data = vec![0; investment_space];
    Investment::pack(investment, &mut investment_data).unwrap();

    program_test.add_account(
        investment_keypair.pubkey(),
        Account {
            lamports: investment_lamports,
            data: investment_data,
            owner: program_id,
            ..Account::default()
        },
    );

    // Add treasury funds
    program_test.add_account(
        treasury.pubkey(),
        Account {
            lamports: 500000,
            data: vec![],
            owner: system_program::id(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create claim refund instruction
    let instruction = CrowdfundInstruction::ClaimRefund;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(project_keypair.pubkey(), false),
                AccountMeta::new(investment_keypair.pubkey(), false),
                AccountMeta::new(investor.pubkey(), true),
                AccountMeta::new(treasury.pubkey(), false),
            ],
            data: instruction.pack(),
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &investor], recent_blockhash);

    // Process transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify investment state
    let investment_account = banks_client
        .get_account(investment_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let updated_investment = Investment::unpack(&investment_account.data).unwrap();
    assert!(updated_investment.is_refunded);
}

#[tokio::test]
async fn test_cancel_project() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();

    // Initialize project account
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Active;

    let space = Project::LEN;
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    let mut project_data = vec![0; space];
    Project::pack(project, &mut project_data).unwrap();

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: project_data,
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create cancel project instruction
    let instruction = CrowdfundInstruction::CancelProject;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: vec![
                AccountMeta::new(project_keypair.pubkey(), false),
                AccountMeta::new(owner.pubkey(), true),
            ],
            data: instruction.pack(),
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);

    // Process transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify project state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let updated_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(updated_project.status, ProjectStatus::Cancelled);
}

// Add more test cases for other scenarios... 