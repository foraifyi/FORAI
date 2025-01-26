#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        clock::Epoch,
        account_info::AccountInfo,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
    };
    use std::str::FromStr;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    fn create_test_account(lamports: u64, data_len: usize) -> (AccountInfo, Vec<u8>) {
        let mut data = vec![0; data_len];
        let pubkey = Pubkey::new_unique();
        let account = AccountInfo::new(
            &pubkey,
            false,
            true,
            &mut lamports,
            &mut data,
            &system_program::ID,
            false,
            Epoch::default(),
        );
        (account, data)
    }

    #[test]
    fn test_initialize_agent() {
        let program_id = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Create test accounts
        let (agent_account, mut agent_data) = create_test_account(0, AgentAccount::LEN);
        let (authority_account, _) = create_test_account(1000, 0);
        let (system_account, _) = create_test_account(0, 0);

        let accounts = vec![
            agent_account.clone(),
            authority_account.clone(),
            system_account.clone(),
        ];

        // Test initialization
        let result = Processor::process_initialize_agent(&program_id, &accounts);
        assert!(result.is_ok());

        // Verify account data
        let agent = AgentAccount::unpack(&agent_data).unwrap();
        assert!(agent.is_initialized);
        assert_eq!(agent.owner, authority);
        assert_eq!(agent.reputation_score, 0);
        assert_eq!(agent.total_rewards, 0);
        assert_eq!(agent.completed_tasks, 0);
    }

    #[test]
    fn test_reward_agent() {
        let program_id = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        
        // Create test accounts
        let (mut agent_account, mut agent_data) = create_test_account(100, AgentAccount::LEN);
        let (authority_account, _) = create_test_account(1000, 0);
        let (mut treasury_account, _) = create_test_account(1000000, 0);

        // Initialize agent account
        let mut agent = AgentAccount {
            is_initialized: true,
            owner: authority,
            reputation_score: 0,
            total_rewards: 0,
            completed_tasks: 0,
        };
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        let accounts = vec![
            agent_account.clone(),
            authority_account.clone(),
            treasury_account.clone(),
        ];

        // Test reward
        let amount = 100;
        let reputation_increase = 10;
        let result = Processor::process_reward_agent(
            &program_id,
            &accounts,
            amount,
            reputation_increase,
        );
        assert!(result.is_ok());

        // Verify account data
        let agent = AgentAccount::unpack(&agent_data).unwrap();
        assert_eq!(agent.total_rewards, amount);
        assert_eq!(agent.reputation_score, reputation_increase);
        assert_eq!(agent.completed_tasks, 1);
    }

    #[tokio::test]
    async fn test_initialize_agent() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let owner = Keypair::new();
        let authority = Keypair::new();

        // Calculate required space and rent
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        // Add accounts to test environment
        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: vec![0; space],
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Build transaction
        let mut transaction = Transaction::new_with_payer(
            &[system_instruction::create_account(
                &payer.pubkey(),
                &agent_keypair.pubkey(),
                lamports,
                space as u64,
                &program_id,
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &agent_keypair], recent_blockhash);

        // Send transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify account status
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let agent = AgentAccount::unpack(&agent_account.data).unwrap();

        assert!(agent.is_initialized);
        assert_eq!(agent.owner, owner.pubkey());
        assert_eq!(agent.authority, authority.pubkey());
        assert_eq!(agent.reputation_score, 5000);
        assert_eq!(agent.level, 1);
    }

    #[tokio::test]
    async fn test_update_reputation() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create and initialize agent account
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();
        let history_keypair = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.authority = authority.pubkey();
        agent.reputation_score = 5000;
        agent.level = 1;

        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Build reputation update transaction
        let new_score = 7500u64;
        let instruction = IncentiveInstruction::UpdateReputation { new_score };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        // Send transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify updated status
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();

        assert_eq!(updated_agent.reputation_score, new_score);
        assert_eq!(updated_agent.level, 4); // Should upgrade to level 4
    }

    #[tokio::test]
    async fn test_reward_agent() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 6000;
        agent.level = 3;
        agent.performance_multiplier = 150;
        agent.consecutive_successes = 5;

        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        // Add accounts to test environment
        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        program_test.add_account(
            treasury.pubkey(),
            Account {
                lamports: 1000000000, // Sufficient reward funds
                data: vec![],
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Build reward transaction
        let reward_amount = 100000u64;
        let instruction = IncentiveInstruction::RewardAgent { amount: reward_amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        // Send transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify reward results
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();

        assert!(updated_agent.total_rewards > reward_amount); // Should have bonus
        assert_eq!(updated_agent.completed_tasks, 1);
        assert_eq!(updated_agent.consecutive_successes, 6);
    }

    #[tokio::test]
    async fn test_batch_update_reputation() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create multiple test agents
        let agent_count = 3;
        let mut agent_keypairs = vec![];
        let mut agent_pubkeys = vec![];
        let authority = Keypair::new();

        for _ in 0..agent_count {
            let keypair = Keypair::new();
            let mut agent = AgentAccount::default();
            agent.is_initialized = true;
            agent.is_active = true;
            agent.reputation_score = 5000;

            let space = AgentAccount::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut agent_data = vec![0; space];
            AgentAccount::pack(agent, &mut agent_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: agent_data,
                    owner: program_id,
                    ..Account::default()
                },
            );

            agent_pubkeys.push(keypair.pubkey());
            agent_keypairs.push(keypair);
        }

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Build batch update transaction
        let new_scores = vec![6000u64, 7000u64, 8000u64];
        let instruction = IncentiveInstruction::BatchUpdateReputation {
            agents: agent_pubkeys.clone(),
            scores: new_scores.clone(),
        };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        // Send transaction
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify all agent updates
        for (i, keypair) in agent_keypairs.iter().enumerate() {
            let account = banks_client
                .get_account(keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&account.data).unwrap();

            assert_eq!(agent.reputation_score, new_scores[i]);
            let expected_level = match new_scores[i] {
                0..=2000 => 1,
                2001..=4000 => 2,
                4001..=6000 => 3,
                6001..=8000 => 4,
                _ => 5,
            };
            assert_eq!(agent.level, expected_level);
        }
    }

    #[tokio::test]
    async fn test_penalize_agent() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 8000;
        agent.level = 4;
        agent.total_tasks = 50;
        agent.completed_tasks = 45;
        agent.consecutive_successes = 10;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let initial_lamports = rent.minimum_balance(space) + 1000000; // Sufficient for penalty

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports: initial_lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Execute penalty
        let penalty_amount = 100000u64;
        let instruction = IncentiveInstruction::PenalizeAgent { amount: penalty_amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify penalty results
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();

        assert!(updated_agent.reputation_score < 8000); // Reputation should decrease
        assert_eq!(updated_agent.consecutive_successes, 0); // Reset consecutive successes
        assert_eq!(updated_agent.failed_tasks, 1);
        assert_eq!(updated_agent.level, 3); // Should downgrade
    }

    #[tokio::test]
    async fn test_level_upgrade() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let owner = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.owner = owner.pubkey();
        agent.reputation_score = 6000;
        agent.level = 3;
        agent.completed_tasks = 30; // Meet upgrade requirements
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Request upgrade to level 4
        let instruction = IncentiveInstruction::RequestLevelUpgrade { target_level: 4 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &owner], recent_blockhash);

        banks_client.process_transaction(transaction).await.unwrap();

        // Verify upgrade results
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();

        assert_eq!(updated_agent.level, 4);
    }

    #[tokio::test]
    async fn test_authority_permissions() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();
        let fake_authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.authority = authority.pubkey();
        agent.reputation_score = 5000;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Try to update reputation with fake authority
        let instruction = IncentiveInstruction::UpdateReputation { new_score: 7000 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &fake_authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err()); // Should fail

        // Update with correct authority
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok()); // Should succeed
    }

    #[tokio::test]
    async fn test_edge_cases() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.authority = authority.pubkey();
        agent.reputation_score = 9900;
        agent.level = 5;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test edge cases:
        
        // 1. Try to exceed maximum reputation value
        let instruction = IncentiveInstruction::UpdateReputation { new_score: 10001 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());

        // 2. Try to exceed maximum level
        let instruction = IncentiveInstruction::RequestLevelUpgrade { target_level: 6 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());

        // 3. Test zero value penalty
        let instruction = IncentiveInstruction::PenalizeAgent { amount: 0 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_consecutive_rewards() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        agent.level = 2;
        agent.consecutive_successes = 0;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        program_test.add_account(
            treasury.pubkey(),
            Account {
                lamports: 10000000000,
                data: vec![],
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Give rewards 5 times consecutively
        let base_amount = 100000u64;
        let mut last_reward = 0;

        for i in 0..5 {
            let instruction = IncentiveInstruction::RewardAgent { amount: base_amount };
            let mut transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);

            banks_client.process_transaction(transaction).await.unwrap();

            let agent_account = banks_client
                .get_account(agent_keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();

            // Verify consecutive success bonus
            let current_reward = updated_agent.total_rewards - last_reward;
            if i > 0 {
                assert!(current_reward > base_amount); // Should have consecutive success bonus
            }
            last_reward = updated_agent.total_rewards;
            assert_eq!(updated_agent.consecutive_successes, (i + 1) as u32);
        }
    }

    #[tokio::test]
    async fn test_reputation_recovery() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 8000;
        agent.level = 4;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space) + 1000000;

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // First penalize
        let penalty_instruction = IncentiveInstruction::PenalizeAgent { amount: 100000 };
        let mut transaction = Transaction::new_with_payer(
            &[penalty_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify post-penalty status
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let penalized_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        assert!(penalized_agent.reputation_score < 8000);
        assert_eq!(penalized_agent.level, 3);

        // Complete tasks consecutively to recover reputation
        for _ in 0..10 {
            let reward_instruction = IncentiveInstruction::RewardAgent { amount: 50000 };
            let mut transaction = Transaction::new_with_payer(
                &[reward_instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);
            banks_client.process_transaction(transaction).await.unwrap();
        }

        // Verify recovery status
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let recovered_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        assert!(recovered_agent.reputation_score > penalized_agent.reputation_score);
        assert_eq!(recovered_agent.level, 4); // Should recover to original level
    }

    #[tokio::test]
    async fn test_performance_multiplier_effects() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        agent.performance_multiplier = 100; // 1x
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Record base reward
        let base_amount = 100000u64;
        let instruction = IncentiveInstruction::RewardAgent { amount: base_amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        banks_client.process_transaction(transaction).await.unwrap();

        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let base_reward = AgentAccount::unpack(&agent_account.data).unwrap().total_rewards;

        // Set higher performance multiplier
        let set_multiplier_instruction = IncentiveInstruction::SetPerformanceMultiplier { multiplier: 200 }; // 2x
        let mut transaction = Transaction::new_with_payer(
            &[set_multiplier_instruction],
            Some(&payer.pubkey()),
        );
        banks_client.process_transaction(transaction).await.unwrap();

        // Give reward again
        let instruction = IncentiveInstruction::RewardAgent { amount: base_amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify reward increase
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        let new_reward = updated_agent.total_rewards - base_reward;
        assert!(new_reward > base_amount * 2); // Consider other bonus factors
    }

    #[tokio::test]
    async fn test_time_based_rewards() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        agent.level = 2;
        agent.last_task_time = 0; // Start with 0 timestamp
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // First reward - should get base reward
        let base_amount = 100000u64;
        let instruction = IncentiveInstruction::RewardAgent { amount: base_amount };
        let mut transaction = Transaction::new_with_payer(
            &[instruction.clone()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let first_reward = AgentAccount::unpack(&agent_account.data).unwrap().total_rewards;

        // Wait for time bonus (simulated by updating last_task_time)
        let mut agent = AgentAccount::unpack(&agent_account.data).unwrap();
        agent.last_task_time -= 86401; // More than 24 hours
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        // Second reward - should get time bonus
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let second_reward = AgentAccount::unpack(&agent_account.data).unwrap().total_rewards - first_reward;

        // Second reward should be higher due to time bonus
        assert!(second_reward > base_amount);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Create multiple concurrent transactions
        let mut transactions = vec![];
        
        // Update reputation
        let update_instruction = IncentiveInstruction::UpdateReputation { new_score: 6000 };
        let mut tx1 = Transaction::new_with_payer(
            &[update_instruction],
            Some(&payer.pubkey()),
        );
        tx1.sign(&[&payer, &authority], recent_blockhash);
        transactions.push(tx1);

        // Set performance multiplier
        let multiplier_instruction = IncentiveInstruction::SetPerformanceMultiplier { multiplier: 150 };
        let mut tx2 = Transaction::new_with_payer(
            &[multiplier_instruction],
            Some(&payer.pubkey()),
        );
        tx2.sign(&[&payer, &authority], recent_blockhash);
        transactions.push(tx2);

        // Process transactions concurrently
        let results = futures::future::join_all(
            transactions.into_iter().map(|tx| banks_client.process_transaction(tx))
        ).await;

        // Verify all transactions succeeded
        for result in results {
            assert!(result.is_ok());
        }

        // Verify final state
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();

        assert_eq!(updated_agent.reputation_score, 6000);
        assert_eq!(updated_agent.performance_multiplier, 150);
    }

    #[tokio::test]
    async fn test_error_recovery() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Try invalid operation first
        let invalid_instruction = IncentiveInstruction::UpdateReputation { new_score: 11000 }; // Invalid score
        let mut transaction = Transaction::new_with_payer(
            &[invalid_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err()); // Should fail

        // Verify state remained unchanged
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let unchanged_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        assert_eq!(unchanged_agent.reputation_score, 5000);

        // Try valid operation after failure
        let valid_instruction = IncentiveInstruction::UpdateReputation { new_score: 6000 };
        let mut transaction = Transaction::new_with_payer(
            &[valid_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);

        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_ok()); // Should succeed

        // Verify state updated correctly
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        assert_eq!(updated_agent.reputation_score, 6000);
    }

    #[tokio::test]
    async fn test_stress_conditions() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        agent.level = 3;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space) + 1000000;

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Perform rapid reward/penalty cycles
        for i in 0..20 {
            let instruction = if i % 2 == 0 {
                IncentiveInstruction::RewardAgent { amount: 50000 }
            } else {
                IncentiveInstruction::PenalizeAgent { amount: 25000 }
            };

            let mut transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);
            banks_client.process_transaction(transaction).await.unwrap();
        }

        // Verify final state is consistent
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let final_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        
        assert!(final_agent.total_tasks == 20);
        assert!(final_agent.completed_tasks + final_agent.failed_tasks == 20);
    }

    #[tokio::test]
    async fn test_level_transitions() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 1000;
        agent.level = 1;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test all level transitions
        let level_thresholds = [2000, 4000, 6000, 8000];
        
        for (i, threshold) in level_thresholds.iter().enumerate() {
            // Update reputation to just below threshold
            let below_score = threshold - 100;
            let instruction = IncentiveInstruction::UpdateReputation { new_score: below_score };
            let mut transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);
            banks_client.process_transaction(transaction).await.unwrap();

            // Verify level hasn't changed
            let agent_account = banks_client
                .get_account(agent_keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&agent_account.data).unwrap();
            assert_eq!(agent.level, (i + 1) as u8);

            // Update reputation to just above threshold
            let above_score = threshold + 100;
            let instruction = IncentiveInstruction::UpdateReputation { new_score: above_score };
            let mut transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);
            banks_client.process_transaction(transaction).await.unwrap();

            // Verify level has increased
            let agent_account = banks_client
                .get_account(agent_keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&agent_account.data).unwrap();
            assert_eq!(agent.level, (i + 2) as u8);
        }
    }

    #[tokio::test]
    async fn test_performance_degradation() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state with perfect performance
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 9000;
        agent.level = 5;
        agent.performance_multiplier = 200;
        agent.consecutive_successes = 20;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Simulate performance degradation
        for _ in 0..5 {
            // Fail three tasks in a row
            for _ in 0..3 {
                let instruction = IncentiveInstruction::PenalizeAgent { amount: 50000 };
                let mut transaction = Transaction::new_with_payer(
                    &[instruction],
                    Some(&payer.pubkey()),
                );
                transaction.sign(&[&payer, &authority], recent_blockhash);
                banks_client.process_transaction(transaction).await.unwrap();
            }

            // Check degradation effects
            let agent_account = banks_client
                .get_account(agent_keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&agent_account.data).unwrap();
            
            assert_eq!(agent.consecutive_successes, 0);
            assert!(agent.performance_multiplier < 200);
            assert!(agent.reputation_score < 9000);
        }
    }

    #[tokio::test]
    async fn test_recovery_thresholds() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 7000;
        agent.level = 4;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Apply severe penalty
        let instruction = IncentiveInstruction::PenalizeAgent { amount: 100000 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Track recovery progress
        let mut last_score = 0;
        let mut recovery_steps = 0;

        // Attempt recovery until reaching original level
        loop {
            let agent_account = banks_client
                .get_account(agent_keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&agent_account.data).unwrap();

            if agent.level == 4 {
                break;
            }

            // Verify progressive improvement
            assert!(agent.reputation_score >= last_score);
            last_score = agent.reputation_score;

            // Perform successful task
            let instruction = IncentiveInstruction::RewardAgent { amount: 50000 };
            let mut transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);
            banks_client.process_transaction(transaction).await.unwrap();

            recovery_steps += 1;
            assert!(recovery_steps < 100); // Prevent infinite loop
        }

        // Verify recovery was achieved
        assert!(recovery_steps > 0);
    }

    #[tokio::test]
    async fn test_extreme_state_transitions() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let authority = Keypair::new();

        // Set initial state at minimum level
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 1000;
        agent.level = 1;
        agent.performance_multiplier = 50;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space) + 1000000;

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Rapid ascent to maximum level
        let max_score = 9999;
        let instruction = IncentiveInstruction::UpdateReputation { new_score: max_score };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify max state
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let max_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        assert_eq!(max_agent.level, 5);
        assert!(max_agent.performance_multiplier > 190);

        // Sudden drop to minimum
        let min_score = 100;
        let instruction = IncentiveInstruction::UpdateReputation { new_score: min_score };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();

        // Verify min state
        let agent_account = banks_client
            .get_account(agent_keypair.pubkey())
            .await
            .unwrap()
            .unwrap();
        let min_agent = AgentAccount::unpack(&agent_account.data).unwrap();
        assert_eq!(min_agent.level, 1);
        assert!(min_agent.performance_multiplier < 60);
    }

    #[tokio::test]
    async fn test_resource_exhaustion() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state with minimal resources
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 5000;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let min_lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports: min_lamports + 100, // Minimal balance
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        program_test.add_account(
            treasury.pubkey(),
            Account {
                lamports: 1000, // Very low treasury balance
                data: vec![],
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Attempt reward beyond treasury balance
        let instruction = IncentiveInstruction::RewardAgent { amount: 2000 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());

        // Attempt penalty beyond agent balance
        let instruction = IncentiveInstruction::PenalizeAgent { amount: min_lamports + 200 };
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &authority], recent_blockhash);
        let result = banks_client.process_transaction(transaction).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multi_agent_interactions() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create multiple agents
        let agent_count = 5;
        let mut agent_keypairs = vec![];
        let mut initial_scores = vec![3000, 4000, 5000, 6000, 7000];
        let authority = Keypair::new();

        for i in 0..agent_count {
            let keypair = Keypair::new();
            let mut agent = AgentAccount::default();
            agent.is_initialized = true;
            agent.reputation_score = initial_scores[i];
            agent.level = (initial_scores[i] / 2000) as u8;
            
            let space = AgentAccount::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut agent_data = vec![0; space];
            AgentAccount::pack(agent, &mut agent_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: agent_data,
                    owner: program_id,
                    ..Account::default()
                },
            );

            agent_keypairs.push(keypair);
        }

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Simulate competition scenario
        for _ in 0..10 {
            // Update all agents' scores based on relative performance
            for (i, keypair) in agent_keypairs.iter().enumerate() {
                let performance_delta = rand::random::<i32>() % 1000;
                let new_score = (initial_scores[i] as i32 + performance_delta).max(0).min(10000) as u64;
                
                let instruction = IncentiveInstruction::UpdateReputation { new_score };
                let mut transaction = Transaction::new_with_payer(
                    &[instruction],
                    Some(&payer.pubkey()),
                );
                transaction.sign(&[&payer, &authority], recent_blockhash);
                banks_client.process_transaction(transaction).await.unwrap();

                initial_scores[i] = new_score;
            }

            // Verify relative rankings
            let mut current_scores = vec![];
            for keypair in &agent_keypairs {
                let account = banks_client
                    .get_account(keypair.pubkey())
                    .await
                    .unwrap()
                    .unwrap();
                let agent = AgentAccount::unpack(&account.data).unwrap();
                current_scores.push(agent.reputation_score);
            }

            // Check score distribution
            let min_score = current_scores.iter().min().unwrap();
            let max_score = current_scores.iter().max().unwrap();
            assert!(*max_score - *min_score < 8000); // Ensure reasonable score spread
        }
    }

    #[tokio::test]
    async fn test_complex_reward_distribution() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create test accounts
        let agent_keypair = Keypair::new();
        let treasury = Keypair::new();
        let authority = Keypair::new();

        // Set initial state with various performance factors
        let mut agent = AgentAccount::default();
        agent.is_initialized = true;
        agent.reputation_score = 7500;
        agent.level = 4;
        agent.performance_multiplier = 150;
        agent.consecutive_successes = 8;
        agent.completed_tasks = 50;
        agent.total_tasks = 55;
        
        let space = AgentAccount::LEN;
        let rent = Rent::default();
        let lamports = rent.minimum_balance(space);

        let mut agent_data = vec![0; space];
        AgentAccount::pack(agent, &mut agent_data).unwrap();

        program_test.add_account(
            agent_keypair.pubkey(),
            Account {
                lamports,
                data: agent_data,
                owner: program_id,
                ..Account::default()
            },
        );

        program_test.add_account(
            treasury.pubkey(),
            Account {
                lamports: 10000000000,
                data: vec![],
                owner: program_id,
                ..Account::default()
            },
        );

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test different reward scenarios
        let test_cases = vec![
            // Base amount, Expected multiplier range (min, max)
            (100000, (2.0, 3.0)),    // Normal reward
            (500000, (2.2, 3.2)),    // Large reward
            (50000, (1.8, 2.8)),     // Small reward
            (1000000, (2.5, 3.5)),   // Exceptional reward
        ];

        for (base_amount, (min_mult, max_mult)) in test_cases {
            let instruction = IncentiveInstruction::RewardAgent { amount: base_amount };
            let mut transaction = Transaction::new_with_payer(
                &[instruction],
                Some(&payer.pubkey()),
            );
            transaction.sign(&[&payer, &authority], recent_blockhash);
            banks_client.process_transaction(transaction).await.unwrap();

            let agent_account = banks_client
                .get_account(agent_keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let updated_agent = AgentAccount::unpack(&agent_account.data).unwrap();
            
            let actual_multiplier = updated_agent.total_rewards as f64 / base_amount as f64;
            assert!(actual_multiplier >= min_mult && actual_multiplier <= max_mult);
        }
    }

    #[tokio::test]
    async fn test_large_scale_operations() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create a large number of agents
        let agent_count = 50;
        let mut agent_keypairs = vec![];
        let authority = Keypair::new();

        // Initialize agents with varying characteristics
        for i in 0..agent_count {
            let keypair = Keypair::new();
            let mut agent = AgentAccount::default();
            agent.is_initialized = true;
            agent.reputation_score = 2000 + (i as u64 * 100);
            agent.level = ((i / 10) + 1) as u8;
            agent.performance_multiplier = 100 + (i as u16);
            
            let space = AgentAccount::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut agent_data = vec![0; space];
            AgentAccount::pack(agent, &mut agent_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: agent_data,
                    owner: program_id,
                    ..Account::default()
                },
            );

            agent_keypairs.push(keypair);
        }

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Perform batch operations
        let batch_size = 10;
        for batch in agent_keypairs.chunks(batch_size) {
            let mut instructions = vec![];
            
            // Create mixed operations for each batch
            for (i, keypair) in batch.iter().enumerate() {
                let instruction = match i % 4 {
                    0 => IncentiveInstruction::RewardAgent { amount: 50000 },
                    1 => IncentiveInstruction::UpdateReputation { 
                        new_score: 5000 + (i as u64 * 100) 
                    },
                    2 => IncentiveInstruction::SetPerformanceMultiplier { 
                        multiplier: 150 
                    },
                    _ => IncentiveInstruction::RequestLevelUpgrade { 
                        target_level: ((i / 20) + 2) as u8 
                    },
                };
                instructions.push(instruction);
            }

            // Process batch
            for instruction in instructions {
                let mut transaction = Transaction::new_with_payer(
                    &[instruction],
                    Some(&payer.pubkey()),
                );
                transaction.sign(&[&payer, &authority], recent_blockhash);
                banks_client.process_transaction(transaction).await.unwrap();
            }
        }

        // Verify system stability
        let mut total_reputation = 0;
        let mut level_distribution = vec![0; 6];

        for keypair in &agent_keypairs {
            let account = banks_client
                .get_account(keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&account.data).unwrap();
            
            total_reputation += agent.reputation_score;
            level_distribution[agent.level as usize] += 1;
        }

        // Verify system-wide metrics
        let average_reputation = total_reputation / agent_count as u64;
        assert!(average_reputation >= 3000 && average_reputation <= 7000);
        
        // Verify level distribution is reasonable
        for (level, count) in level_distribution.iter().enumerate().skip(1) {
            assert!(*count > 0, "Level {} has no agents", level);
        }
    }

    #[tokio::test]
    async fn test_time_based_competition() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create competing agents
        let agent_count = 3;
        let mut agent_keypairs = vec![];
        let authority = Keypair::new();
        let mut last_task_times = vec![];

        // Initialize agents with similar starting conditions
        for _ in 0..agent_count {
            let keypair = Keypair::new();
            let mut agent = AgentAccount::default();
            agent.is_initialized = true;
            agent.reputation_score = 5000;
            agent.level = 3;
            agent.last_task_time = 0;
            
            let space = AgentAccount::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut agent_data = vec![0; space];
            AgentAccount::pack(agent, &mut agent_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: agent_data,
                    owner: program_id,
                    ..Account::default()
                },
            );

            agent_keypairs.push(keypair);
            last_task_times.push(0i64);
        }

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Simulate time-based competition
        let simulation_steps = 20;
        let mut current_time = 0i64;

        for _ in 0..simulation_steps {
            current_time += 3600; // Advance 1 hour

            for (i, keypair) in agent_keypairs.iter().enumerate() {
                // Calculate time-based bonus
                let time_since_last = current_time - last_task_times[i];
                let time_bonus = if time_since_last > 86400 { 
                    150 // 50% bonus for daily activity
                } else if time_since_last > 43200 {
                    125 // 25% bonus for 12-hour activity
                } else {
                    100 // No bonus
                };

                // Perform task with time-based reward
                let base_amount = 50000u64;
                let adjusted_amount = (base_amount as f64 * time_bonus as f64 / 100.0) as u64;
                
                let instruction = IncentiveInstruction::RewardAgent { amount: adjusted_amount };
                let mut transaction = Transaction::new_with_payer(
                    &[instruction],
                    Some(&payer.pubkey()),
                );
                transaction.sign(&[&payer, &authority], recent_blockhash);
                banks_client.process_transaction(transaction).await.unwrap();

                last_task_times[i] = current_time;
            }
        }

        // Verify time-based performance differences
        let mut final_rewards = vec![];
        for keypair in &agent_keypairs {
            let account = banks_client
                .get_account(keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&account.data).unwrap();
            final_rewards.push(agent.total_rewards);
        }

        // Check reward distribution reflects time-based bonuses
        let max_reward = final_rewards.iter().max().unwrap();
        let min_reward = final_rewards.iter().min().unwrap();
        assert!(max_reward - min_reward < max_reward / 2); // Ensure reasonable reward spread
    }

    #[tokio::test]
    async fn test_system_load_balancing() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create a large number of agents with different activity patterns
        let agent_count = 100;
        let mut agent_keypairs = vec![];
        let authority = Keypair::new();

        // Initialize agents with different activity patterns
        for i in 0..agent_count {
            let keypair = Keypair::new();
            let mut agent = AgentAccount::default();
            agent.is_initialized = true;
            agent.reputation_score = 5000;
            agent.level = ((i / 20) + 1) as u8;
            agent.performance_multiplier = 100 + ((i % 5) * 20) as u16;
            
            let space = AgentAccount::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut agent_data = vec![0; space];
            AgentAccount::pack(agent, &mut agent_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: agent_data,
                    owner: program_id,
                    ..Account::default()
                },
            );

            agent_keypairs.push(keypair);
        }

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Simulate high-load periods
        let simulation_cycles = 5;
        for cycle in 0..simulation_cycles {
            // Create transaction batches with varying sizes
            let batch_sizes = vec![5, 10, 20, 50];
            
            for &batch_size in &batch_sizes {
                let mut transactions = vec![];
                
                // Create mixed transaction types
                for agents in agent_keypairs.chunks(batch_size) {
                    for (i, keypair) in agents.iter().enumerate() {
                        let instruction = match i % 4 {
                            0 => IncentiveInstruction::RewardAgent { amount: 50000 },
                            1 => IncentiveInstruction::UpdateReputation { 
                                new_score: 5000 + (cycle * 1000) as u64 
                            },
                            2 => IncentiveInstruction::SetPerformanceMultiplier { 
                                multiplier: 150 
                            },
                            _ => IncentiveInstruction::RequestLevelUpgrade { 
                                target_level: ((i / 20) + 2) as u8 
                            },
                        };

                        let mut tx = Transaction::new_with_payer(
                            &[instruction],
                            Some(&payer.pubkey()),
                        );
                        tx.sign(&[&payer, &authority], recent_blockhash);
                        transactions.push(tx);
                    }
                }

                // Process transactions in parallel
                let results = futures::future::join_all(
                    transactions.into_iter().map(|tx| banks_client.process_transaction(tx))
                ).await;

                // Verify success rate
                let success_count = results.iter().filter(|r| r.is_ok()).count();
                let success_rate = success_count as f64 / results.len() as f64;
                assert!(success_rate > 0.95); // Expect >95% success rate
            }
        }

        // Verify system stability after high load
        let mut level_changes = 0;
        let mut performance_improvements = 0;

        for keypair in &agent_keypairs {
            let account = banks_client
                .get_account(keypair.pubkey())
                .await
                .unwrap()
                .unwrap();
            let agent = AgentAccount::unpack(&account.data).unwrap();
            
            if agent.level > ((agent_keypairs.iter().position(|k| k.pubkey() == keypair.pubkey()).unwrap() / 20) + 1) as u8 {
                level_changes += 1;
            }
            if agent.performance_multiplier > 100 + ((agent_keypairs.iter().position(|k| k.pubkey() == keypair.pubkey()).unwrap() % 5) * 20 as u16 {
                performance_improvements += 1;
            }
        }

        // Verify system evolution
        assert!(level_changes > 0, "No agents progressed in levels");
        assert!(performance_improvements > 0, "No performance improvements recorded");
    }

    #[tokio::test]
    async fn test_reward_distribution_fairness() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "forai_incentive",
            program_id,
            processor!(Processor::process),
        );

        // Create agents with different characteristics
        let configurations = vec![
            // (reputation, level, performance_multiplier, consecutive_successes)
            (9000, 5, 200, 20),  // High performer
            (5000, 3, 150, 10),  // Average performer
            (2000, 1, 100, 0),   // New agent
            (7000, 4, 180, 15),  // Good performer
            (3000, 2, 120, 5),   // Improving agent
        ];

        let mut agent_keypairs = vec![];
        let authority = Keypair::new();

        for (reputation, level, multiplier, successes) in configurations {
            let keypair = Keypair::new();
            let mut agent = AgentAccount::default();
            agent.is_initialized = true;
            agent.reputation_score = reputation;
            agent.level = level;
            agent.performance_multiplier = multiplier;
            agent.consecutive_successes = successes;
            
            let space = AgentAccount::LEN;
            let rent = Rent::default();
            let lamports = rent.minimum_balance(space);

            let mut agent_data = vec![0; space];
            AgentAccount::pack(agent, &mut agent_data).unwrap();

            program_test.add_account(
                keypair.pubkey(),
                Account {
                    lamports,
                    data: agent_data,
                    owner: program_id,
                    ..Account::default()
                },
            );

            agent_keypairs.push(keypair);
        }

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Test reward distribution for different task types
        let task_types = vec![
            (100000, "small"),    // Small task
            (500000, "medium"),   // Medium task
            (1000000, "large"),   // Large task
        ];

        let mut reward_ratios = vec![];

        for (base_amount, _task_type) in task_types {
            let mut cycle_rewards = vec![];

            // Distribute rewards
            for keypair in &agent_keypairs {
                let instruction = IncentiveInstruction::RewardAgent { amount: base_amount };
                let mut transaction = Transaction::new_with_payer(
                    &[instruction],
                    Some(&payer.pubkey()),
                );
                transaction.sign(&[&payer, &authority], recent_blockhash);
                banks_client.process_transaction(transaction).await.unwrap();

                let account = banks_client
                    .get_account(keypair.pubkey())
                    .await
                    .unwrap()
                    .unwrap();
                let agent = AgentAccount::unpack(&account.data).unwrap();
                cycle_rewards.push(agent.total_rewards);
            }

            // Calculate reward ratios
            let max_reward = *cycle_rewards.iter().max().unwrap();
            let min_reward = *cycle_rewards.iter().min().unwrap();
            reward_ratios.push(max_reward as f64 / min_reward as f64);
        }

        // Verify reward distribution fairness
        for ratio in reward_ratios {
            // Maximum reward should not be more than 5 times the minimum reward
            assert!(ratio < 5.0, "Reward distribution too skewed: {}", ratio);
        }
    }
} 