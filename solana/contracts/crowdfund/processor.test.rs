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
    state::{Project, ProjectStatus, Investment, Milestone},
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

    // Create test milestones
    let milestones = vec![
        Milestone {
            description: *b"First milestone................................",
            target_amount: 1000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
        },
        Milestone {
            description: *b"Second milestone...............................",
            target_amount: 2000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
        },
    ];

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
        milestones: milestones.clone(),
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
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
        &[instruction],
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

    // Create test accounts and initial state
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let treasury = Keypair::new();

    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Funded;
    project.current_amount = 3000000;
    project.milestones = vec![
        Milestone {
            description: *b"First milestone................................",
            target_amount: 1000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
        },
        Milestone {
            description: *b"Second milestone...............................",
            target_amount: 2000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
        },
    ];

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

    // Complete first milestone
    let instruction = CrowdfundInstruction::CompleteMilestone {
        milestone_index: 0,
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
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
    assert!(updated_project.milestones[0].is_completed);
    assert!(updated_project.milestones[0].is_funds_released);
    assert!(!updated_project.milestones[1].is_completed);
    assert_eq!(updated_project.status, ProjectStatus::Funded);
}

#[tokio::test]
async fn test_error_handling() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let invalid_user = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project with completed status
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Completed;
    
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

    // Test 1: Invest in completed project
    let instruction = CrowdfundInstruction::Invest { amount: 1000000 };
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &invalid_user], recent_blockhash);
    
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err());

    // Test 2: Unauthorized milestone completion
    let instruction = CrowdfundInstruction::CompleteMilestone { milestone_index: 0 };
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &invalid_user], recent_blockhash);
    
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_boundary_conditions() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investor = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project with edge case values
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Active;
    project.target_amount = u64::MAX;
    project.start_time = 0;
    project.end_time = i64::MAX;

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

    // Add minimum possible funds to investor
    program_test.add_account(
        investor.pubkey(),
        Account {
            lamports: 1,
            data: vec![],
            owner: system_program::id(),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test minimum investment
    let instruction = CrowdfundInstruction::Invest { amount: 1 };
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &investor], recent_blockhash);
    
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err()); // Should fail due to rent exemption
}

#[tokio::test]
async fn test_full_project_lifecycle() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investors = vec![Keypair::new(), Keypair::new(), Keypair::new()];
    let treasury = Keypair::new();

    // Initialize project
    let milestones = vec![
        Milestone {
            description: *b"Milestone 1.................................",
            target_amount: 1000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
        },
        Milestone {
            description: *b"Milestone 2.................................",
            target_amount: 2000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
        },
    ];

    let space = Project::LEN;
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    // Setup initial project
    let instruction = CrowdfundInstruction::InitializeProject {
        title: *b"Lifecycle Test Project........................",
        description: *b"Testing full project lifecycle.............................",
        target_amount: 3000000,
        start_time: 0,
        end_time: 999999999,
        milestones: milestones.clone(),
    };

    // Add funds to investors
    for investor in &investors {
        program_test.add_account(
            investor.pubkey(),
            Account {
                lamports: 2000000,
                data: vec![],
                owner: system_program::id(),
                ..Account::default()
            },
        );
    }

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Phase 1: Project Creation
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Phase 2: Multiple Investments
    for (i, investor) in investors.iter().enumerate() {
        let amount = 1000000 + (i as u64 * 500000);
        let instruction = CrowdfundInstruction::Invest { amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, investor], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    // Phase 3: Complete Milestones
    for i in 0..milestones.len() {
        let instruction = CrowdfundInstruction::CompleteMilestone {
            milestone_index: i as u8,
        };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &owner], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }

    // Verify final project state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let final_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(final_project.status, ProjectStatus::Completed);
    assert!(final_project.milestones.iter().all(|m| m.is_completed));
    assert!(final_project.current_amount >= final_project.target_amount);
}

#[tokio::test]
async fn test_concurrent_investments() {
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
    
    // Create 50 concurrent investors
    let investors: Vec<_> = (0..50)
        .map(|_| {
            let investor = Keypair::new();
            program_test.add_account(
                investor.pubkey(),
                Account {
                    lamports: 1000000,
                    data: vec![],
                    owner: system_program::id(),
                    ..Account::default()
                },
            );
            investor
        })
        .collect();

    // Initialize project
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Active;
    project.target_amount = 10000000;
    
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

    // Create concurrent investment transactions
    let mut handles = vec![];
    for (i, investor) in investors.iter().enumerate() {
        let amount = 100000 + (i as u64 * 1000);
        let instruction = CrowdfundInstruction::Invest { amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, investor], recent_blockhash);
        
        let handle = tokio::spawn({
            let banks_client = banks_client.clone();
            async move {
                banks_client.process_transaction(transaction).await
            }
        });
        handles.push(handle);
    }

    // Wait for all transactions
    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    // Verify final state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let final_project = Project::unpack(&project_account.data).unwrap();
    assert!(final_project.current_amount > 0);
}

#[tokio::test]
async fn test_project_recovery() {
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

    // Initialize project in failed state
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Failed;
    project.current_amount = 1000000;
    project.target_amount = 5000000;
    
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

    // Attempt recovery process
    let recovery_steps = [
        CrowdfundInstruction::ClaimRefund,
        // Add more recovery steps as needed
    ];

    for instruction in recovery_steps.iter() {
        let mut transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &owner], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }
}

#[tokio::test]
async fn test_security_attacks() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let attacker = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project
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

    // Test 1: Double investment attack
    let amount = 1000000;
    let instruction = CrowdfundInstruction::Invest { amount };
    let mut transaction = Transaction::new_with_payer(
        &[instruction.clone(), instruction.clone()], // Try to invest twice in one transaction
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &attacker], recent_blockhash);
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err());

    // Test 2: Unauthorized milestone completion
    let instruction = CrowdfundInstruction::CompleteMilestone { milestone_index: 0 };
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &attacker], recent_blockhash);
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err());

    // Test 3: Invalid state transition
    let instruction = CrowdfundInstruction::ClaimRefund;
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &attacker], recent_blockhash);
    let result = banks_client.process_transaction(transaction).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_program_upgrade() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts with existing state
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project with old format
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

    // Verify state can still be accessed after upgrade
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let loaded_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(loaded_project.owner, owner.pubkey());
    assert_eq!(loaded_project.status, ProjectStatus::Active);
}

#[tokio::test]
async fn test_cross_program_interaction() {
    let program_id = Pubkey::new_unique();
    let token_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investor = Keypair::new();
    let token_mint = Keypair::new();
    let investor_token_account = Keypair::new();
    let project_token_account = Keypair::new();

    // Initialize token accounts and mint
    program_test.add_program("spl_token", token_program_id, None);
    
    // Setup token accounts with initial balances
    let token_account_space = 165;
    let rent = Rent::default();
    let token_account_rent = rent.minimum_balance(token_account_space);

    program_test.add_account(
        investor_token_account.pubkey(),
        Account {
            lamports: token_account_rent,
            data: vec![0; token_account_space],
            owner: token_program_id,
            ..Account::default()
        },
    );

    program_test.add_account(
        project_token_account.pubkey(),
        Account {
            lamports: token_account_rent,
            data: vec![0; token_account_space],
            owner: token_program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test token transfer during investment
    let investment_amount = 1000000;
    let instructions = vec![
        // Create token transfer instruction
        spl_token::instruction::transfer(
            &token_program_id,
            &investor_token_account.pubkey(),
            &project_token_account.pubkey(),
            &investor.pubkey(),
            &[],
            investment_amount,
        ).unwrap(),
        // Create investment instruction
        CrowdfundInstruction::Invest { amount: investment_amount },
    ];

    let mut transaction = Transaction::new_with_payer(
        &instructions,
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &investor], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_error_recovery() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investor = Keypair::new();

    // Initialize project in error state
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Active;
    project.current_amount = 1000000;
    project.error_count = 3;
    project.last_error_timestamp = 0;
    
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

    // Test error recovery process
    let recovery_instruction = CrowdfundInstruction::RecoverFromError {
        error_code: 1,
        recovery_data: vec![0, 1, 2, 3],
    };

    let mut transaction = Transaction::new_with_payer(
        &[recovery_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify recovery
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let recovered_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(recovered_project.error_count, 0);
    assert!(recovered_project.last_error_timestamp > 0);
}

#[tokio::test]
async fn test_performance_benchmark() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    
    // Create large number of test investors
    const NUM_INVESTORS: usize = 100;
    let investors: Vec<_> = (0..NUM_INVESTORS)
        .map(|_| {
            let investor = Keypair::new();
            program_test.add_account(
                investor.pubkey(),
                Account {
                    lamports: 10000000,
                    data: vec![],
                    owner: system_program::id(),
                    ..Account::default()
                },
            );
            investor
        })
        .collect();

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Measure transaction throughput
    let start_time = std::time::Instant::now();
    let mut handles = vec![];

    for investor in investors {
        let instruction = CrowdfundInstruction::Invest { amount: 100000 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &investor], recent_blockhash);
        
        let handle = tokio::spawn({
            let banks_client = banks_client.clone();
            async move {
                banks_client.process_transaction(transaction).await
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap().unwrap();
    }

    let duration = start_time.elapsed();
    let tps = NUM_INVESTORS as f64 / duration.as_secs_f64();
    println!("Transactions per second: {}", tps);
}

#[tokio::test]
async fn test_state_migration() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();

    // Create old version of project state
    #[derive(Default)]
    struct OldProjectState {
        is_initialized: bool,
        owner: Pubkey,
        title: [u8; 32],
        target_amount: u64,
        current_amount: u64,
    }

    let old_project = OldProjectState {
        is_initialized: true,
        owner: owner.pubkey(),
        title: *b"Old Project................................",
        target_amount: 1000000,
        current_amount: 500000,
    };

    let space = std::mem::size_of::<OldProjectState>();
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: serialize(&old_project).unwrap(),
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Perform state migration
    let instruction = CrowdfundInstruction::MigrateState {
        new_version: 2,
    };

    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify migrated state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let migrated_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(migrated_project.owner, old_project.owner);
    assert_eq!(migrated_project.title, old_project.title);
    assert_eq!(migrated_project.target_amount, old_project.target_amount);
    assert_eq!(migrated_project.current_amount, old_project.current_amount);
}

#[tokio::test]
async fn test_governance_integration() {
    let program_id = Pubkey::new_unique();
    let governance_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let dao_treasury = Keypair::new();
    let governance_token_mint = Keypair::new();

    // Initialize project with governance features
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.governance_config = Some(GovernanceConfig {
        token_mint: governance_token_mint.pubkey(),
        dao_treasury: dao_treasury.pubkey(),
        proposal_threshold: 1000000,
        voting_period: 86400, // 1 day
        quorum: 5100, // 51%
    });

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

    // Test governance proposal creation
    let proposal_instruction = CrowdfundInstruction::CreateGovernanceProposal {
        title: *b"Change Investment Terms......................",
        description: *b"Modify minimum investment amount.........................",
        execution_data: vec![1, 2, 3, 4],
    };

    let mut transaction = Transaction::new_with_payer(
        &[proposal_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &owner], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_emergency_procedures() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let emergency_admin = Keypair::new();
    let treasury = Keypair::new();

    // Initialize project with emergency features
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.emergency_admin = Some(emergency_admin.pubkey());
    project.status = ProjectStatus::Active;
    project.current_amount = 5000000;

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

    // Test emergency pause
    let pause_instruction = CrowdfundInstruction::EmergencyPause {
        reason: *b"Security vulnerability detected.............",
    };

    let mut transaction = Transaction::new_with_payer(
        &[pause_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &emergency_admin], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify paused state
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let paused_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(paused_project.status, ProjectStatus::Paused);
}

#[tokio::test]
async fn test_automated_milestone_verification() {
    let program_id = Pubkey::new_unique();
    let oracle_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let oracle = Keypair::new();

    // Initialize project with automated verification
    let milestones = vec![
        Milestone {
            description: *b"Automated milestone 1........................",
            target_amount: 1000000,
            completion_time: 0,
            is_completed: false,
            is_funds_released: false,
            verification_method: VerificationMethod::Oracle {
                oracle: oracle.pubkey(),
                data_feed: *b"github_commits",
                threshold: 100,
            },
        },
    ];

    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.milestones = milestones;
    project.status = ProjectStatus::Funded;

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

    // Test automated verification
    let verify_instruction = CrowdfundInstruction::VerifyMilestone {
        milestone_index: 0,
        oracle_data: vec![0, 1, 2, 3], // Simulated oracle data
    };

    let mut transaction = Transaction::new_with_payer(
        &[verify_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &oracle], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_reward_distribution() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investors: Vec<_> = (0..5).map(|_| Keypair::new()).collect();
    let reward_token_mint = Keypair::new();

    // Initialize project with reward structure
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.reward_config = Some(RewardConfig {
        token_mint: reward_token_mint.pubkey(),
        tokens_per_investment: 100,
        vesting_period: 30 * 86400, // 30 days
        cliff_period: 7 * 86400,    // 7 days
    });

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

    // Test reward distribution
    for investor in &investors {
        let claim_instruction = CrowdfundInstruction::ClaimRewards {
            investor: investor.pubkey(),
        };

        let mut transaction = Transaction::new_with_payer(
            &[claim_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, investor], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }
}

#[tokio::test]
async fn test_multisig_transactions() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create multisig committee members
    let committee_members: Vec<_> = (0..3).map(|_| Keypair::new()).collect();
    let threshold = 2; // Require 2 out of 3 signatures

    // Create test accounts
    let project_keypair = Keypair::new();
    let multisig_account = Keypair::new();

    // Initialize project with multisig configuration
    let mut project = Project::default();
    project.is_initialized = true;
    project.multisig_config = Some(MultisigConfig {
        members: committee_members.iter().map(|k| k.pubkey()).collect(),
        threshold,
        nonce: 0,
    });

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

    // Create multisig transaction
    let instruction = CrowdfundInstruction::UpdateProjectConfig {
        new_target_amount: Some(5000000),
        new_end_time: Some(1700000000),
    };

    // First signature
    let mut transaction = Transaction::new_with_payer(
        &[instruction.clone()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &committee_members[0]], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Second signature (should execute)
    let mut transaction = Transaction::new_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &committee_members[1]], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_offchain_data_integration() {
    let program_id = Pubkey::new_unique();
    let oracle_program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let oracle = Keypair::new();

    // Initialize project with external data requirements
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.external_data_config = Some(ExternalDataConfig {
        oracle: oracle.pubkey(),
        required_feeds: vec![
            *b"github_stats",
            *b"code_quality",
            *b"test_coverage",
        ],
        update_interval: 3600, // 1 hour
    });

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

    // Test external data update
    let update_instruction = CrowdfundInstruction::UpdateExternalData {
        feed_name: *b"github_stats",
        data: vec![1, 2, 3, 4], // Simulated external data
        timestamp: Clock::get()?.unix_timestamp,
    };

    let mut transaction = Transaction::new_with_payer(
        &[update_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &oracle], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_upgrade_committee() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create committee members
    let committee_members: Vec<_> = (0..5).map(|_| Keypair::new()).collect();
    
    // Initialize upgrade committee
    let committee = UpgradeCommittee {
        members: committee_members.iter().map(|k| k.pubkey()).collect(),
        threshold: 3, // Require 3 out of 5 votes
        proposals: vec![],
    };

    // Create test accounts
    let project_keypair = Keypair::new();
    let upgrade_buffer = Keypair::new();

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

    // Test upgrade proposal and voting
    let propose_instruction = CrowdfundInstruction::ProposeUpgrade {
        new_program_id: Pubkey::new_unique(),
        buffer: upgrade_buffer.pubkey(),
        description: *b"Security patch for milestone verification.........",
    };

    // Submit proposal
    let mut transaction = Transaction::new_with_payer(
        &[propose_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &committee_members[0]], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Committee members vote
    for member in committee_members.iter().take(3) {
        let vote_instruction = CrowdfundInstruction::VoteOnUpgrade {
            proposal_id: 0,
            approve: true,
        };
        let mut transaction = Transaction::new_with_payer(
            &[vote_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, member], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }
}

#[tokio::test]
async fn test_liquidity_pool() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let pool_keypair = Keypair::new();
    let token_mint = Keypair::new();
    let lp_token_mint = Keypair::new();

    // Initialize liquidity pool
    let pool = LiquidityPool {
        is_initialized: true,
        token_mint: token_mint.pubkey(),
        lp_token_mint: lp_token_mint.pubkey(),
        token_reserve: 1000000,
        lp_token_supply: 1000000,
        fee_rate: 30, // 0.3%
        last_update_time: 0,
    };

    let space = std::mem::size_of::<LiquidityPool>();
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    program_test.add_account(
        pool_keypair.pubkey(),
        Account {
            lamports,
            data: serialize(&pool).unwrap(),
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test liquidity provision
    let provide_liquidity_instruction = CrowdfundInstruction::ProvideLiquidity {
        token_amount: 100000,
        minimum_lp_tokens: 95000,
    };

    let mut transaction = Transaction::new_with_payer(
        &[provide_liquidity_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Test swap
    let swap_instruction = CrowdfundInstruction::Swap {
        amount_in: 10000,
        minimum_amount_out: 9900,
    };

    let mut transaction = Transaction::new_with_payer(
        &[swap_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_staking_rewards() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let staking_pool = Keypair::new();
    let reward_mint = Keypair::new();
    let stakers: Vec<_> = (0..5).map(|_| Keypair::new()).collect();

    // Initialize staking pool
    let pool = StakingPool {
        is_initialized: true,
        reward_mint: reward_mint.pubkey(),
        total_staked: 0,
        reward_rate: 100, // 1 token per second per staked token
        last_update_time: 0,
        stakers: vec![],
    };

    let space = std::mem::size_of::<StakingPool>();
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    program_test.add_account(
        staking_pool.pubkey(),
        Account {
            lamports,
            data: serialize(&pool).unwrap(),
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test staking and rewards
    for staker in &stakers {
        // Stake tokens
        let stake_instruction = CrowdfundInstruction::Stake {
            amount: 1000000,
        };
        let mut transaction = Transaction::new_with_payer(
            &[stake_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, staker], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Advance time (simulate staking period)
        let clock = Clock::get()?;
        let new_timestamp = clock.unix_timestamp + 86400; // 1 day
        program_test.set_sysvar(&clock, new_timestamp);

        // Claim rewards
        let claim_instruction = CrowdfundInstruction::ClaimStakingRewards;
        let mut transaction = Transaction::new_with_payer(
            &[claim_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, staker], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }
}

#[tokio::test]
async fn test_vesting_schedule() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let token_mint = Keypair::new();
    let beneficiary = Keypair::new();

    // Initialize vesting schedule
    let vesting = VestingSchedule {
        is_initialized: true,
        beneficiary: beneficiary.pubkey(),
        total_amount: 1000000,
        start_time: Clock::get()?.unix_timestamp,
        cliff_duration: 180 * 86400, // 6 months
        vesting_duration: 360 * 86400, // 1 year
        released_amount: 0,
        revocable: true,
    };

    let space = std::mem::size_of::<VestingSchedule>();
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: serialize(&vesting).unwrap(),
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test vesting release at different times
    let test_times = vec![
        90 * 86400,  // 3 months (before cliff)
        180 * 86400, // 6 months (at cliff)
        270 * 86400, // 9 months (during vesting)
        360 * 86400, // 12 months (end of vesting)
    ];

    for time in test_times {
        let clock = Clock::get()?;
        program_test.set_sysvar(&clock, time);

        let release_instruction = CrowdfundInstruction::ReleaseVestedTokens;
        let mut transaction = Transaction::new_with_payer(
            &[release_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &beneficiary], recent_blockhash);
        
        let result = banks_client.process_transaction(transaction).await;
        
        // Verify release based on vesting schedule
        if time < vesting.start_time + vesting.cliff_duration {
            assert!(result.is_err()); // Should fail before cliff
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_dynamic_pricing() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();

    // Initialize project with dynamic pricing
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.pricing_config = Some(DynamicPricingConfig {
        base_price: 1000000,
        min_price: 500000,
        max_price: 2000000,
        price_adjustment_rate: 50, // 5% adjustment per period
        volume_threshold: 10000000,
    });

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

    // Test price adjustments based on volume
    let volumes = vec![5000000, 15000000, 25000000];
    let mut last_price = 1000000;

    for volume in volumes {
        let update_instruction = CrowdfundInstruction::UpdateDynamicPrice {
            current_volume: volume,
        };

        let mut transaction = Transaction::new_with_payer(
            &[update_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify price adjustment
        let project_account = banks_client
            .get_account(project_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();

        let updated_project = Project::unpack(&project_account.data).unwrap();
        let new_price = updated_project.pricing_config.unwrap().current_price;
        
        assert!(new_price >= 500000 && new_price <= 2000000); // Within bounds
        assert_ne!(new_price, last_price); // Price should change
        last_price = new_price;
    }
}

#[tokio::test]
async fn test_batch_operations() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investors: Vec<_> = (0..20).map(|_| Keypair::new()).collect();

    // Initialize multiple projects
    let projects: Vec<_> = (0..5)
        .map(|i| {
            let keypair = Keypair::new();
            let mut project = Project::default();
            project.is_initialized = true;
            project.owner = owner.pubkey();
            project.title = format!("Project {}................................", i).into_bytes().try_into().unwrap();
            project.target_amount = 1000000 * (i as u64 + 1);
            project.status = ProjectStatus::Active;

            let space = Project::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut project_data = vec![0; space];
            Project::pack(project, &mut project_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: project_data,
                    owner: program_id,
                    ..Account::default()
                },
            );
            keypair
        })
        .collect();

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test batch investment across multiple projects
    let mut batch_instructions = vec![];
    for (i, project) in projects.iter().enumerate() {
        for investor in investors.iter().take(5) {
            let amount = 100000 * (i as u64 + 1);
            batch_instructions.push((
                CrowdfundInstruction::Invest { amount },
                vec![project.pubkey(), investor.pubkey()],
                vec![investor],
            ));
        }
    }

    // Process batch in chunks
    for chunk in batch_instructions.chunks(5) {
        let mut transaction = Transaction::new_with_payer(
            &chunk.iter().map(|(inst, _, _)| inst.clone()).collect::<Vec<_>>(),
            Some(&payer.pubkey()),
        );
        let signers = chunk.iter().flat_map(|(_, _, signers)| signers.iter()).collect::<Vec<_>>();
        transaction.sign(&[&payer].iter().chain(signers.iter()), recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }
}

#[tokio::test]
async fn test_market_volatility() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let oracle = Keypair::new();

    // Initialize project with market conditions
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.market_config = Some(MarketConfig {
        price_oracle: oracle.pubkey(),
        volatility_threshold: 1000, // 10%
        circuit_breaker_threshold: 2000, // 20%
        cooldown_period: 3600, // 1 hour
        last_price: 1000000,
    });

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

    // Test market price updates with different volatility levels
    let price_scenarios = vec![
        1100000, // +10% (within threshold)
        1300000, // +30% (triggers circuit breaker)
        900000,  // -10% (within threshold)
        700000,  // -30% (triggers circuit breaker)
    ];

    for new_price in price_scenarios {
        let update_instruction = CrowdfundInstruction::UpdateMarketPrice {
            new_price,
            timestamp: Clock::get()?.unix_timestamp,
        };

        let mut transaction = Transaction::new_with_payer(
            &[update_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &oracle], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;

        // Verify circuit breaker behavior
        let price_change = ((new_price as f64 - 1000000.0) / 1000000.0).abs();
        if price_change > 0.20 {
            assert!(result.is_err()); // Should trigger circuit breaker
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_risk_management() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let risk_manager = Keypair::new();

    // Initialize project with risk parameters
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.risk_config = Some(RiskConfig {
        max_investment_per_user: 1000000,
        daily_volume_limit: 5000000,
        concentration_limit: 2000, // 20% max per investor
        risk_score_threshold: 75,
        cooldown_period: 3600,
    });

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

    // Test risk assessment scenarios
    let risk_scenarios = vec![
        (2000000, 90), // High amount, high risk score
        (500000, 60),  // Medium amount, medium risk score
        (100000, 30),  // Low amount, low risk score
    ];

    for (amount, risk_score) in risk_scenarios {
        let invest_instruction = CrowdfundInstruction::InvestWithRisk {
            amount,
            risk_score,
        };

        let mut transaction = Transaction::new_with_payer(
            &[invest_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &risk_manager], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;

        // Verify risk management rules
        if amount > 1000000 || risk_score > 75 {
            assert!(result.is_err()); // Should fail high risk transactions
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_performance_optimization() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    
    // Initialize project with performance monitoring
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.performance_metrics = Some(PerformanceMetrics {
        total_transactions: 0,
        average_response_time: 0,
        peak_tps: 0,
        last_optimization: 0,
    });

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

    // Test transaction batching with different batch sizes
    let batch_sizes = vec![1, 10, 50, 100];
    let mut performance_results = vec![];

    for batch_size in batch_sizes {
        let start_time = std::time::Instant::now();
        let mut handles = vec![];

        // Create and process transactions in batches
        for i in 0..100 {
            if i % batch_size == 0 {
                let mut instructions = vec![];
                for j in 0..batch_size {
                    if i + j < 100 {
                        instructions.push(CrowdfundInstruction::UpdateMetrics {
                            transaction_count: 1,
                            response_time: 100,
                        });
                    }
                }

                let mut transaction = Transaction::new_with_payer(
                    &instructions,
                    Some(&payer.pubkey()),
                );
                transaction.sign(&[&payer], recent_blockhash);
                
                let handle = tokio::spawn({
                    let banks_client = banks_client.clone();
                    async move {
                        banks_client.process_transaction(transaction).await
                    }
                });
                handles.push(handle);
            }
        }

        // Wait for batch completion
        for handle in handles {
            handle.await.unwrap().unwrap();
        }

        let duration = start_time.elapsed();
        performance_results.push((batch_size, duration));
    }

    // Analyze performance results
    for (batch_size, duration) in performance_results {
        println!(
            "Batch size: {}, Average time per transaction: {:?}",
            batch_size,
            duration / 100
        );
    }
}

#[tokio::test]
async fn test_stress_recovery() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let investors: Vec<_> = (0..100).map(|_| Keypair::new()).collect();

    // Initialize project with stress monitoring
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Active;
    project.current_amount = 1000000;
    project.stress_metrics = Some(StressMetrics {
        error_count: 0,
        last_error_time: 0,
        recovery_attempts: 0,
        auto_recovery_enabled: true,
    });

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

    // Simulate high stress conditions
    let mut handles = vec![];
    for investor in &investors {
        // Create multiple concurrent transactions
        let invest_instruction = CrowdfundInstruction::Invest {
            amount: 100000,
        };

        let mut transaction = Transaction::new_with_payer(
            &[invest_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, investor], recent_blockhash);

        let handle = tokio::spawn({
            let banks_client = banks_client.clone();
            async move {
                banks_client.process_transaction(transaction).await
            }
        });
        handles.push(handle);
    }

    // Process all transactions and collect results
    let mut success_count = 0;
    let mut error_count = 0;
    for handle in handles {
        match handle.await.unwrap() {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }
    }

    // Verify stress handling
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let final_project = Project::unpack(&project_account.data).unwrap();
    assert!(final_project.stress_metrics.unwrap().error_count == error_count);
}

#[tokio::test]
async fn test_data_consistency() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let validator = Keypair::new();

    // Initialize project with consistency checks
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.consistency_config = Some(ConsistencyConfig {
        checksum: [0; 32],
        last_validation: 0,
        validation_interval: 1000,
        validator: validator.pubkey(),
    });

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

    // Test data validation
    let validate_instruction = CrowdfundInstruction::ValidateDataConsistency {
        current_checksum: [1; 32],
        timestamp: Clock::get()?.unix_timestamp,
    };

    let mut transaction = Transaction::new_with_payer(
        &[validate_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &validator], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify consistency check
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let updated_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(updated_project.consistency_config.unwrap().checksum, [1; 32]);
}

#[tokio::test]
async fn test_recovery_procedures() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let recovery_admin = Keypair::new();

    // Initialize project in error state
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.status = ProjectStatus::Error;
    project.error_info = Some(ErrorInfo {
        code: 1001,
        timestamp: Clock::get()?.unix_timestamp,
        recovery_attempts: 2,
        last_recovery: 0,
    });

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

    // Test recovery procedure
    let recovery_instruction = CrowdfundInstruction::InitiateRecovery {
        recovery_type: RecoveryType::StateReset,
        recovery_data: vec![0, 1, 2, 3],
    };

    let mut transaction = Transaction::new_with_payer(
        &[recovery_instruction],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &recovery_admin], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify recovery
    let project_account = banks_client
        .get_account(project_keypair.pubkey())
        .await
        .unwrap()
        .unwrap();

    let recovered_project = Project::unpack(&project_account.data).unwrap();
    assert_eq!(recovered_project.status, ProjectStatus::Active);
    assert!(recovered_project.error_info.is_none());
}

#[tokio::test]
async fn test_security_audit() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let auditor = Keypair::new();

    // Initialize project with security config
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.security_config = Some(SecurityConfig {
        auditor: auditor.pubkey(),
        last_audit: 0,
        audit_interval: 7 * 86400, // Weekly audit
        critical_vulnerabilities: 0,
        pending_fixes: 0,
        security_score: 100,
    });

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

    // Test security audit scenarios
    let audit_scenarios = vec![
        SecurityAudit {
            vulnerabilities: vec![
                Vulnerability {
                    severity: VulnerabilitySeverity::Critical,
                    description: *b"Buffer overflow risk.........................",
                    fix_deadline: Clock::get()?.unix_timestamp + 86400,
                },
            ],
            security_score: 60,
        },
        SecurityAudit {
            vulnerabilities: vec![],
            security_score: 95,
        },
    ];

    for audit in audit_scenarios {
        let audit_instruction = CrowdfundInstruction::SubmitSecurityAudit {
            audit_data: audit.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        };

        let mut transaction = Transaction::new_with_payer(
            &[audit_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &auditor], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;

        // Verify security measures
        if audit.security_score < 80 {
            // Should trigger security lockdown
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_compliance() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let compliance_officer = Keypair::new();

    // Initialize project with compliance requirements
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.compliance_config = Some(ComplianceConfig {
        officer: compliance_officer.pubkey(),
        required_checks: vec![
            ComplianceCheck::KYC,
            ComplianceCheck::AML,
            ComplianceCheck::Jurisdiction,
        ],
        last_review: 0,
        compliance_status: ComplianceStatus::Pending,
    });

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

    // Test compliance verification
    let compliance_checks = vec![
        ComplianceResult {
            check_type: ComplianceCheck::KYC,
            passed: true,
            details: *b"KYC verification complete.....................",
        },
        ComplianceResult {
            check_type: ComplianceCheck::AML,
            passed: true,
            details: *b"AML screening passed.........................",
        },
        ComplianceResult {
            check_type: ComplianceCheck::Jurisdiction,
            passed: false,
            details: *b"Restricted jurisdiction detected.............",
        },
    ];

    for check in compliance_checks {
        let verify_instruction = CrowdfundInstruction::VerifyCompliance {
            check_result: check.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        };

        let mut transaction = Transaction::new_with_payer(
            &[verify_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &compliance_officer], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;

        // Verify compliance enforcement
        if !check.passed {
            assert!(result.is_err()); // Should fail on compliance violation
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_upgrade_compatibility() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    
    // Initialize project with version info
    let mut project = Project::default();
    project.is_initialized = true;
    project.owner = owner.pubkey();
    project.version_info = Some(VersionInfo {
        current_version: 1,
        last_upgrade: 0,
        upgrade_history: vec![],
        compatibility_flags: 0,
    });

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

    // Test version upgrades
    let upgrade_versions = vec![
        UpgradeSpec {
            version: 2,
            breaking_changes: false,
            compatibility_mask: 0xFF,
            migration_data: vec![1, 2, 3],
        },
        UpgradeSpec {
            version: 3,
            breaking_changes: true,
            compatibility_mask: 0x00,
            migration_data: vec![4, 5, 6],
        },
    ];

    for spec in upgrade_versions {
        let upgrade_instruction = CrowdfundInstruction::UpgradeProgram {
            spec: spec.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        };

        let mut transaction = Transaction::new_with_payer(
            &[upgrade_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &owner], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;

        // Verify upgrade compatibility
        if spec.breaking_changes {
            // Should require special handling for breaking changes
            assert!(result.is_err());
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn test_data_migration() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "forai_crowdfund",
        program_id,
        processor!(Processor::process),
    );

    // Create test accounts
    let project_keypair = Keypair::new();
    let owner = Keypair::new();
    let migration_authority = Keypair::new();

    // Initialize project with legacy data
    #[derive(Default)]
    struct LegacyProjectData {
        is_initialized: bool,
        owner: Pubkey,
        balance: u64,
        metadata: [u8; 32],
    }

    let legacy_data = LegacyProjectData {
        is_initialized: true,
        owner: owner.pubkey(),
        balance: 1000000,
        metadata: [1; 32],
    };

    let space = std::mem::size_of::<LegacyProjectData>();
    let rent = Rent::default();
    let lamports = rent.minimum_balance(space);

    program_test.add_account(
        project_keypair.pubkey(),
        Account {
            lamports,
            data: serialize(&legacy_data).unwrap(),
            owner: program_id,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Test data migration steps
    let migration_steps = vec![
        MigrationStep {
            field_name: *b"balance",
            transformation: DataTransformation::Split {
                new_fields: vec![*b"locked_balance", *b"available_balance"],
            },
        },
        MigrationStep {
            field_name: *b"metadata",
            transformation: DataTransformation::Expand {
                new_size: 64,
                fill_pattern: 0,
            },
        },
    ];

    for step in migration_steps {
        let migrate_instruction = CrowdfundInstruction::MigrateData {
            step: step.clone(),
            timestamp: Clock::get()?.unix_timestamp,
        };

        let mut transaction = Transaction::new_with_payer(
            &[migrate_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &migration_authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify migration results
        let project_account = banks_client
            .get_account(project_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();

        match step.transformation {
            DataTransformation::Split { .. } => {
                // Verify balance split
                let project = Project::unpack(&project_account.data).unwrap();
                assert_eq!(project.locked_balance + project.available_balance, 1000000);
            }
            DataTransformation::Expand { new_size, .. } => {
                // Verify metadata expansion
                assert_eq!(project_account.data.len(), new_size as usize);
            }
        }
    }
}

// Add more test cases for other scenarios... 