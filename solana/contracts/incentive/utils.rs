use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    clock::Clock,
    sysvar::Sysvar,
    rent::Rent,
};
use crate::{
    error::IncentiveError,
    state::AgentAccount,
};

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<(), ProgramError> {
    if account.owner != owner {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

pub fn assert_signer(account: &AccountInfo) -> Result<(), ProgramError> {
    if !account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn assert_account_key(account: &AccountInfo, key: &Pubkey) -> Result<(), ProgramError> {
    if account.key != key {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}

pub fn assert_uninitialized<T: Pack + IsInitialized>(
    account_data: &T,
) -> Result<(), ProgramError> {
    if account_data.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    Ok(())
}

pub fn validate_agent_state(agent: &AgentAccount) -> Result<(), ProgramError> {
    if !agent.is_initialized {
        return Err(IncentiveError::UninitializedAccount.into());
    }

    if !agent.is_active {
        return Err(IncentiveError::AccountSuspended.into());
    }

    if agent.reputation_score > 10000 {
        return Err(IncentiveError::InvalidReputationScore.into());
    }

    if agent.level < 1 || agent.level > 5 {
        return Err(IncentiveError::InvalidLevel.into());
    }

    Ok(())
}

pub fn validate_rent_exempt(
    account: &AccountInfo,
    min_balance: u64,
) -> Result<(), ProgramError> {
    if account.lamports() < min_balance {
        return Err(IncentiveError::InsufficientFunds.into());
    }
    Ok(())
}

pub fn validate_treasury_balance(
    treasury: &AccountInfo,
    required_amount: u64,
) -> Result<(), ProgramError> {
    if treasury.lamports() < required_amount {
        return Err(IncentiveError::InsufficientTreasuryBalance.into());
    }
    Ok(())
}

pub fn validate_cooldown_period(
    last_action_time: i64,
    cooldown_seconds: i64,
) -> Result<(), ProgramError> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    if current_time - last_action_time < cooldown_seconds {
        return Err(IncentiveError::CooldownPeriodNotEnded.into());
    }
    Ok(())
}

pub fn validate_action_window(
    start_time: i64,
    end_time: i64,
) -> Result<(), ProgramError> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;
    
    if current_time < start_time || current_time > end_time {
        return Err(IncentiveError::InvalidActionWindow.into());
    }
    Ok(())
}

pub fn calculate_reward_multiplier(
    reputation_score: u64,
    performance_multiplier: u16,
    consecutive_successes: u32,
    level: u8,
) -> f64 {
    let reputation_bonus = (reputation_score as f64) / 5000.0;
    let performance_bonus = (performance_multiplier as f64) / 100.0;
    let streak_bonus = 1.0 + (consecutive_successes.min(10) as f64 * 0.05);
    let level_bonus = 1.0 + ((level as f64 - 1.0) * 0.2);

    reputation_bonus * performance_bonus * streak_bonus * level_bonus
}

pub fn calculate_penalty_multiplier(
    reputation_score: u64,
    level: u8,
) -> f64 {
    let reputation_factor = (reputation_score as f64) / 10000.0;
    let level_factor = level as f64 * 0.2;
    
    reputation_factor * (1.0 + level_factor)
}

pub fn calculate_level_from_score(score: u64) -> u8 {
    match score {
        0..=2000 => 1,
        2001..=4000 => 2,
        4001..=6000 => 3,
        6001..=8000 => 4,
        _ => 5,
    }
}

pub fn validate_level_requirements(
    agent: &AgentAccount,
    target_level: u8,
) -> Result<(), ProgramError> {
    if target_level <= agent.level || target_level > 5 {
        return Err(IncentiveError::InvalidTargetLevel.into());
    }

    let required_reputation = (target_level as u64 - 1) * 2000;
    if agent.reputation_score < required_reputation {
        return Err(IncentiveError::InsufficientReputation.into());
    }

    let required_tasks = (target_level as u32 - 1) * 10;
    if agent.completed_tasks < required_tasks {
        return Err(IncentiveError::InsufficientCompletedTasks.into());
    }

    Ok(())
}

pub fn calculate_performance_score(
    completed_tasks: u32,
    total_tasks: u32,
    consecutive_successes: u32,
) -> u16 {
    if total_tasks == 0 {
        return 100;
    }

    let completion_rate = (completed_tasks as f64 / total_tasks as f64) * 100.0;
    let streak_bonus = (consecutive_successes.min(10) as f64) * 5.0;
    
    ((completion_rate + streak_bonus).min(200.0) as u16).max(50)
}

pub fn verify_program_authority(
    authority: &AccountInfo,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    if authority.key != program_id {
        return Err(IncentiveError::InvalidProgramAuthority.into());
    }
    Ok(())
}

pub fn verify_account_ownership(
    account: &AccountInfo,
    expected_owner: &Pubkey,
) -> Result<(), ProgramError> {
    if account.owner != expected_owner {
        return Err(IncentiveError::InvalidOwner.into());
    }
    Ok(())
}

pub fn validate_system_constraints(
    accounts: &[AccountInfo],
    min_accounts: usize,
) -> Result<(), ProgramError> {
    if accounts.len() < min_accounts {
        return Err(ProgramError::NotEnoughAccountKeys);
    }
    Ok(())
}

pub fn validate_data_size(
    data: &[u8],
    expected_size: usize,
) -> Result<(), ProgramError> {
    if data.len() != expected_size {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

// Transaction fee calculation
pub fn calculate_transaction_fee(
    base_fee: u64,
    data_size: usize,
    priority_level: u8,
) -> u64 {
    let size_fee = (data_size as u64) * 10; // 10 lamports per byte
    let priority_multiplier = match priority_level {
        0 => 1.0,  // Normal
        1 => 1.5,  // High
        2 => 2.0,  // Urgent
        _ => 1.0,
    };
    
    ((base_fee + size_fee) as f64 * priority_multiplier) as u64
}

pub fn calculate_operation_cost(
    operation_type: u8,
    agent_level: u8,
    is_complex: bool,
) -> u64 {
    let base_cost = match operation_type {
        0 => 5000,     // Basic update
        1 => 10000,    // State change
        2 => 20000,    // Complex operation
        _ => 5000,
    };

    let level_discount = (agent_level as f64 * 0.1).min(0.5);
    let complexity_multiplier = if is_complex { 1.5 } else { 1.0 };

    ((base_cost as f64) * (1.0 - level_discount) * complexity_multiplier) as u64
}

// Advanced reward distribution
pub fn calculate_reward_distribution(
    total_reward: u64,
    agent: &AgentAccount,
    treasury_share: u8,  // Percentage (0-100)
) -> Result<(u64, u64), ProgramError> {
    if treasury_share > 100 {
        return Err(IncentiveError::InvalidDistributionRatio.into());
    }

    let performance_factor = calculate_performance_score(
        agent.completed_tasks,
        agent.total_tasks,
        agent.consecutive_successes,
    ) as f64 / 100.0;

    let base_share = (total_reward as f64) * ((100 - treasury_share) as f64 / 100.0);
    let agent_reward = (base_share * performance_factor) as u64;
    let treasury_reward = total_reward.saturating_sub(agent_reward);

    Ok((agent_reward, treasury_reward))
}

pub fn calculate_bonus_distribution(
    base_amount: u64,
    agent: &AgentAccount,
    current_time: i64,
) -> u64 {
    let time_bonus = if current_time - agent.last_task_time > 86400 {
        1.2 // 20% bonus for daily activity
    } else {
        1.0
    };

    let completion_rate = if agent.total_tasks > 0 {
        agent.completed_tasks as f64 / agent.total_tasks as f64
    } else {
        0.5
    };

    let quality_multiplier = 1.0 + (completion_rate * 0.5); // Up to 50% bonus for high quality
    
    (base_amount as f64 * time_bonus * quality_multiplier) as u64
}

// Risk control functions
pub fn assess_operation_risk(
    agent: &AgentAccount,
    operation_value: u64,
    operation_type: u8,
) -> Result<u8, ProgramError> {
    // Risk levels: 0 = Low, 1 = Medium, 2 = High, 3 = Critical
    let mut risk_score = 0u8;

    // Value-based risk assessment
    risk_score += match operation_value {
        0..=1000000 => 0,
        1000001..=5000000 => 1,
        5000001..=10000000 => 2,
        _ => 3,
    };

    // Agent history based risk
    let failure_rate = if agent.total_tasks > 0 {
        agent.failed_tasks as f64 / agent.total_tasks as f64
    } else {
        0.5
    };

    risk_score += match failure_rate {
        x if x < 0.1 => 0,
        x if x < 0.2 => 1,
        x if x < 0.3 => 2,
        _ => 3,
    };

    // Operation type risk
    risk_score += match operation_type {
        0 => 0, // Read operation
        1 => 1, // Update operation
        2 => 2, // Critical operation
        _ => 3,
    };

    // Average risk score
    Ok(risk_score / 3)
}

pub fn validate_risk_threshold(
    risk_level: u8,
    agent: &AgentAccount,
    required_reputation: u64,
) -> Result<(), ProgramError> {
    match risk_level {
        0 => Ok(()), // Low risk - always allowed
        1 => {
            // Medium risk - require minimum reputation
            if agent.reputation_score < required_reputation {
                return Err(IncentiveError::InsufficientReputation.into());
            }
            Ok(())
        }
        2 => {
            // High risk - require high reputation and minimum level
            if agent.reputation_score < required_reputation || agent.level < 3 {
                return Err(IncentiveError::InsufficientPrivileges.into());
            }
            Ok(())
        }
        _ => Err(IncentiveError::OperationTooRisky.into()),
    }
}

// Data integrity checks
pub fn verify_data_integrity(
    data: &[u8],
    expected_checksum: &[u8; 32],
) -> Result<(), ProgramError> {
    use sha2::{Sha256, Digest};
    
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();

    if result.as_slice() != expected_checksum {
        return Err(IncentiveError::DataIntegrityViolation.into());
    }
    Ok(())
}

pub fn validate_state_transition(
    old_state: &AgentAccount,
    new_state: &AgentAccount,
) -> Result<(), ProgramError> {
    // Validate immutable fields
    if old_state.owner != new_state.owner {
        return Err(IncentiveError::ImmutableFieldModification.into());
    }

    // Validate reasonable changes
    if new_state.reputation_score > old_state.reputation_score + 1000 {
        return Err(IncentiveError::UnreasonableStateChange.into());
    }

    // Validate level changes
    if new_state.level > old_state.level + 1 {
        return Err(IncentiveError::InvalidLevelJump.into());
    }

    // Validate task counts
    if new_state.total_tasks < old_state.total_tasks {
        return Err(IncentiveError::InvalidTaskCount.into());
    }

    Ok(())
}

pub fn verify_account_consistency(
    agent: &AgentAccount,
    history_count: u32,
    total_rewards: u64,
) -> Result<(), ProgramError> {
    // Verify task counts
    if agent.completed_tasks + agent.failed_tasks != agent.total_tasks {
        return Err(IncentiveError::InconsistentTaskCounts.into());
    }

    // Verify reward calculations
    let expected_rewards = calculate_reward_multiplier(
        agent.reputation_score,
        agent.performance_multiplier,
        agent.consecutive_successes,
        agent.level,
    ) as u64 * total_rewards;

    if (expected_rewards as i64 - agent.total_rewards as i64).abs() > 1000 {
        return Err(IncentiveError::InconsistentRewardCalculation.into());
    }

    Ok(())
} 