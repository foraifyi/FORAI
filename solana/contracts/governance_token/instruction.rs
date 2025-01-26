use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    program_error::ProgramError,
};

#[derive(Debug)]
pub enum GovernanceTokenInstruction {
    /// Initialize token config
    /// 
    /// Accounts expected:
    /// 1. `[writable]` New config account
    /// 2. `[signer]` Authority
    /// 3. `[writable]` Token mint
    /// 4. `[writable]` Treasury account
    /// 5. `[]` Token program
    InitializeConfig {
        decimals: u8,
        initial_supply: u64,
    },

    /// Request token mint
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` New mint request account
    /// 3. `[signer]` Requester
    /// 4. `[]` Recipient token account
    RequestMint {
        amount: u64,
        expiry_time: i64,
    },

    /// Execute mint request
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` Mint request account
    /// 3. `[signer]` Authority
    /// 4. `[writable]` Token mint
    /// 5. `[writable]` Recipient token account
    /// 6. `[]` Token program
    ExecuteMint,

    /// Set transfer limit
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[writable]` Transfer limit account
    /// 3. `[signer]` Authority
    /// 4. `[]` Owner
    SetTransferLimit {
        daily_limit: u64,
        is_exempt: bool,
    },

    /// Update transfer status
    /// 
    /// Accounts expected:
    /// 1. `[writable]` Config account
    /// 2. `[signer]` Authority
    UpdateTransferStatus {
        enable_transfers: bool,
    },
}

impl GovernanceTokenInstruction {
    /// Unpacks a byte buffer into a GovernanceTokenInstruction
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => {
                let (decimals, rest) = Self::unpack_u8(rest)?;
                let (initial_supply, _) = Self::unpack_u64(rest)?;
                Self::InitializeConfig {
                    decimals,
                    initial_supply,
                }
            }
            1 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (expiry_time, _) = Self::unpack_i64(rest)?;
                Self::RequestMint {
                    amount,
                    expiry_time,
                }
            }
            2 => Self::ExecuteMint,
            3 => {
                let (daily_limit, rest) = Self::unpack_u64(rest)?;
                let (is_exempt, _) = Self::unpack_bool(rest)?;
                Self::SetTransferLimit {
                    daily_limit,
                    is_exempt,
                }
            }
            4 => {
                let (enable_transfers, _) = Self::unpack_bool(rest)?;
                Self::UpdateTransferStatus {
                    enable_transfers,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .try_into()
            .map(u64::from_le_bytes)
            .or(Err(ProgramError::InvalidInstructionData))?;
        Ok((amount, rest))
    }

    fn unpack_i64(input: &[u8]) -> Result<(i64, &[u8]), ProgramError> {
        if input.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (amount, rest) = input.split_at(8);
        let amount = amount
            .try_into()
            .map(i64::from_le_bytes)
            .or(Err(ProgramError::InvalidInstructionData))?;
        Ok((amount, rest))
    }

    fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (value, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok((*value, rest))
    }

    fn unpack_bool(input: &[u8]) -> Result<(bool, &[u8]), ProgramError> {
        if input.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bool_byte, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok((*bool_byte != 0, rest))
    }
} 