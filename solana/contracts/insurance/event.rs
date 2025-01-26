use solana_program::{
    account_info::AccountInfo,
    msg,
    pubkey::Pubkey,
};

pub enum InsuranceEvent<'a> {
    // Pool events
    PoolInitialized {
        pool: &'a Pubkey,
        admin: &'a Pubkey,
        name: [u8; 32],
        min_capital_requirement: u64,
        coverage_ratio: u8,
        premium_rate: u8,
        claim_period: u64,
    },
    PoolUpdated {
        pool: &'a Pubkey,
        admin: &'a Pubkey,
        coverage_ratio: Option<u8>,
        premium_rate: Option<u8>,
        claim_period: Option<u64>,
        min_capital_requirement: Option<u64>,
        timestamp: i64,
    },
    PoolStatusChanged {
        pool: &'a Pubkey,
        admin: &'a Pubkey,
        paused: bool,
        timestamp: i64,
    },

    // Capital events
    CapitalAdded {
        pool: &'a Pubkey,
        provider: &'a Pubkey,
        amount: u64,
        total_capital: u64,
        timestamp: i64,
    },
    CapitalWithdrawn {
        pool: &'a Pubkey,
        admin: &'a Pubkey,
        amount: u64,
        remaining_capital: u64,
        timestamp: i64,
    },

    // Policy events
    PolicyCreated {
        pool: &'a Pubkey,
        policy: &'a Pubkey,
        insured: &'a Pubkey,
        coverage_amount: u64,
        premium_amount: u64,
        start_time: i64,
        end_time: i64,
    },
    PolicyExpired {
        pool: &'a Pubkey,
        policy: &'a Pubkey,
        insured: &'a Pubkey,
        timestamp: i64,
    },

    // Claim events
    ClaimSubmitted {
        pool: &'a Pubkey,
        policy: &'a Pubkey,
        claim: &'a Pubkey,
        claimant: &'a Pubkey,
        amount: u64,
        timestamp: i64,
    },
    ClaimProcessed {
        pool: &'a Pubkey,
        policy: &'a Pubkey,
        claim: &'a Pubkey,
        approved: bool,
        amount: u64,
        timestamp: i64,
    },

    // Error events
    OperationFailed {
        pool: &'a Pubkey,
        operation: &'a str,
        error: &'a str,
        timestamp: i64,
    },
}

impl<'a> InsuranceEvent<'a> {
    pub fn emit(&self) {
        match self {
            // Pool events
            Self::PoolInitialized {
                pool,
                admin,
                name,
                min_capital_requirement,
                coverage_ratio,
                premium_rate,
                claim_period,
            } => {
                msg!("Insurance Pool Initialized: pool={}, admin={}, name={:?}, min_capital={}, coverage_ratio={}, premium_rate={}, claim_period={}",
                    pool, admin, name, min_capital_requirement, coverage_ratio, premium_rate, claim_period);
            }
            Self::PoolUpdated {
                pool,
                admin,
                coverage_ratio,
                premium_rate,
                claim_period,
                min_capital_requirement,
                timestamp,
            } => {
                msg!("Insurance Pool Updated: pool={}, admin={}, coverage_ratio={:?}, premium_rate={:?}, claim_period={:?}, min_capital={:?}, time={}",
                    pool, admin, coverage_ratio, premium_rate, claim_period, min_capital_requirement, timestamp);
            }
            Self::PoolStatusChanged {
                pool,
                admin,
                paused,
                timestamp,
            } => {
                msg!("Insurance Pool Status Changed: pool={}, admin={}, paused={}, time={}",
                    pool, admin, paused, timestamp);
            }

            // Capital events
            Self::CapitalAdded {
                pool,
                provider,
                amount,
                total_capital,
                timestamp,
            } => {
                msg!("Capital Added: pool={}, provider={}, amount={}, total_capital={}, time={}",
                    pool, provider, amount, total_capital, timestamp);
            }
            Self::CapitalWithdrawn {
                pool,
                admin,
                amount,
                remaining_capital,
                timestamp,
            } => {
                msg!("Capital Withdrawn: pool={}, admin={}, amount={}, remaining_capital={}, time={}",
                    pool, admin, amount, remaining_capital, timestamp);
            }

            // Policy events
            Self::PolicyCreated {
                pool,
                policy,
                insured,
                coverage_amount,
                premium_amount,
                start_time,
                end_time,
            } => {
                msg!("Insurance Policy Created: pool={}, policy={}, insured={}, coverage={}, premium={}, start={}, end={}",
                    pool, policy, insured, coverage_amount, premium_amount, start_time, end_time);
            }
            Self::PolicyExpired {
                pool,
                policy,
                insured,
                timestamp,
            } => {
                msg!("Insurance Policy Expired: pool={}, policy={}, insured={}, time={}",
                    pool, policy, insured, timestamp);
            }

            // Claim events
            Self::ClaimSubmitted {
                pool,
                policy,
                claim,
                claimant,
                amount,
                timestamp,
            } => {
                msg!("Insurance Claim Submitted: pool={}, policy={}, claim={}, claimant={}, amount={}, time={}",
                    pool, policy, claim, claimant, amount, timestamp);
            }
            Self::ClaimProcessed {
                pool,
                policy,
                claim,
                approved,
                amount,
                timestamp,
            } => {
                msg!("Insurance Claim Processed: pool={}, policy={}, claim={}, approved={}, amount={}, time={}",
                    pool, policy, claim, approved, amount, timestamp);
            }

            // Error events
            Self::OperationFailed {
                pool,
                operation,
                error,
                timestamp,
            } => {
                msg!("Insurance Operation Failed: pool={}, operation={}, error={}, time={}",
                    pool, operation, error, timestamp);
            }
        }
    }
} 