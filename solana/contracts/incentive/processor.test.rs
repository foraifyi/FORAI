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
} 