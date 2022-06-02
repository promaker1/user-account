use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    },
};

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum UserAccountInstruction {

    /// Creates a new account for the given `user_id`.
    ///
    /// Accounts expected by this instruction:
    ///   0. `[]` The system program account
    ///   1. `[writeable]` The user account
    ///   2. `[]` The token mint address
    ///   3. `[signer]` Operator/fee payer account
    Create { 
        user_id: u32,
        user_account_bump_seed: u8
    },

    /// Adds the given `amount` of tokens to the balance of the existing user
    ///
    /// Accounts expected by this instruction:
    ///   0. `[writeable]` The user account
    ///   1. `[]` The token mint address
    ///   2. `[signer]` Operator/fee payer account
    Deposit { 
        user_id: u32,
        user_account_bump_seed: u8,
        amount: u64   
    },

    /// Withdraws the given `amount` of tokens from the balance of the existing user
    /// 
    /// Accounts expected by this instruction:
    ///   0. `[]` The SPL token program account
    ///   1. `[writeable]` The user account
    ///   2. `[]` The token mint address
    ///   3. `[writeable]` Account to be used as the source for the transfer operation 
    ///   4. `[]` The source authority account
    ///   5. `[writeable]` Account to be used as the destination for the transfer operation 
    ///   6. `[signer]` Operator/fee payer account
    Withdraw { 
        user_id: u32,
        user_account_bump_seed: u8,
        source_authority_bump_seed: u8,
        amount: u64
    },

    /// Transfers the given `amount` of tokens from the `sender` to `recipient`.
    /// 
    /// Accounts expected by this instruction:
    ///   0. `[]` The token mint address
    ///   1. `[writeable]` The sender account
    ///   2. `[writeable]` The recipient account 
    ///   3. `[signer]` Operator/fee payer account
    Transfer { 
        sender_user_id: u32,
        sender_account_bump_seed: u8,
        recipient_user_id: u32,
        recipient_account_bump_seed: u8,
        amount: u64
    },

    /// Blocks the given amount of tokens on the existing user's balance.
    ///
    /// Accounts expected by this instruction:
    ///   0. `[writeable]` The user account
    ///   1. `[]` The token mint address
    ///   2. `[signer]` Operator/fee payer account
    Block { 
        user_id: u32,
        user_account_bump_seed: u8,
        amount: u64
    },

    /// Unblocks the given amount of tokens on the existing user's balance.
    ///
    /// Accounts expected by this instruction:
    ///   0. `[writeable]` The user account
    ///   1. `[]` The token mint address
    ///   2. `[signer]` Operator/fee payer account
    Unblock { 
        user_id: u32,
        user_account_bump_seed: u8,
        amount: u64
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create(
    program_id: &Pubkey,
    user_id: u32,
    user_account_bump_seed: u8,
    user_account_key: &Pubkey,
    token_mint_key: &Pubkey,
    payer_key: &Pubkey
) -> Result<Instruction, ProgramError> {

    let instruction_data = UserAccountInstruction::Create { user_id, user_account_bump_seed };
    let data = instruction_data.try_to_vec().unwrap();

    let accounts = vec![
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new(*user_account_key, false),
        AccountMeta::new_readonly(*token_mint_key, false),
        AccountMeta::new_readonly(*payer_key, true)
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn deposit(
    program_id: &Pubkey,
    user_id: u32,
    user_account_bump_seed: u8,
    amount: u64, 
    user_account_key: &Pubkey,
    token_mint_key: &Pubkey,
    payer_key: &Pubkey,
) -> Result<Instruction, ProgramError> {

    let instruction_data = UserAccountInstruction::Deposit { user_id, user_account_bump_seed, amount };
    let data = instruction_data.try_to_vec().unwrap();

    let accounts = vec![
        AccountMeta::new(*user_account_key, false),
        AccountMeta::new_readonly(*token_mint_key, false),
        AccountMeta::new_readonly(*payer_key, true)
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn withdraw(
    program_id: &Pubkey,
    user_id: u32,
    user_account_bump_seed: u8,
    source_authority_bump_seed: u8,
    amount: u64,
    user_account_key: &Pubkey,
    token_mint_key: &Pubkey,
    source_account_key: &Pubkey,
    source_authority_account_key: &Pubkey,
    destination_account_key: &Pubkey,
    payer_key: &Pubkey,
) -> Result<Instruction, ProgramError> {

    let instruction_data = UserAccountInstruction::Withdraw { 
        user_id, 
        user_account_bump_seed, 
        source_authority_bump_seed, 
        amount 
    };
    let data = instruction_data.try_to_vec().unwrap();

    let accounts = vec![
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new(*user_account_key, false),
        AccountMeta::new_readonly(*token_mint_key, false),
        AccountMeta::new(*source_account_key, false),
        AccountMeta::new_readonly(*source_authority_account_key, false),
        AccountMeta::new(*destination_account_key, false),
        AccountMeta::new_readonly(*payer_key, true)
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn transfer(
    program_id: &Pubkey,
    token_mint_key: &Pubkey,
    sender_user_id: u32,
    sender_account_bump_seed: u8,
    sender_account_key: &Pubkey,
    recipient_user_id: u32,
    recipient_account_bump_seed: u8,
    recipient_account_key: &Pubkey,
    amount: u64,
    payer_key: &Pubkey,
) -> Result<Instruction, ProgramError> {

    let instruction_data = UserAccountInstruction::Transfer { 
        sender_user_id, 
        sender_account_bump_seed, 
        recipient_user_id,
        recipient_account_bump_seed, 
        amount 
    };
    let data = instruction_data.try_to_vec().unwrap();

    let accounts = vec![
        AccountMeta::new_readonly(*token_mint_key, false),
        AccountMeta::new(*sender_account_key, false),
        AccountMeta::new(*recipient_account_key, false),
        AccountMeta::new_readonly(*payer_key, true)
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn block(
    program_id: &Pubkey,
    user_id: u32,
    user_account_bump_seed: u8,
    amount: u64, 
    user_account_key: &Pubkey,
    token_mint_key: &Pubkey,
    payer_key: &Pubkey,
) -> Result<Instruction, ProgramError> {

    let instruction_data = UserAccountInstruction::Block { user_id, user_account_bump_seed, amount };
    let data = instruction_data.try_to_vec().unwrap();

    let accounts = vec![
        AccountMeta::new(*user_account_key, false),
        AccountMeta::new_readonly(*token_mint_key, false),
        AccountMeta::new_readonly(*payer_key, true)
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn unblock(
    program_id: &Pubkey,
    user_id: u32,
    user_account_bump_seed: u8,
    amount: u64, 
    user_account_key: &Pubkey,
    token_mint_key: &Pubkey,
    payer_key: &Pubkey,
) -> Result<Instruction, ProgramError> {

    let instruction_data = UserAccountInstruction::Unblock { user_id, user_account_bump_seed, amount };
    let data = instruction_data.try_to_vec().unwrap();

    let accounts = vec![
        AccountMeta::new(*user_account_key, false),
        AccountMeta::new_readonly(*token_mint_key, false),
        AccountMeta::new_readonly(*payer_key, true)
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}