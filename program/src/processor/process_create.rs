use crate::state::UserAccount;

use std::mem::size_of;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    system_instruction::create_account,
    msg
};

pub fn process_create(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    user_id: u32, 
    user_account_bump_seed: u8) -> ProgramResult {
    
    let accounts_iter = &mut accounts.iter();

    let system_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let token_mint_account = next_account_info(accounts_iter)?;
    let operator_account = next_account_info(accounts_iter)?;

    if !operator_account.is_signer {
        msg!("Provided operator account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);   
    }
    
    let seeds = [
        &user_id.to_le_bytes()[..], 
        &token_mint_account.key.to_bytes(), 
        &operator_account.key.to_bytes(), 
        &[user_account_bump_seed]
    ];

    let user_account_key = Pubkey::create_program_address(
        &seeds, 
        &program_id)?;

    if *user_account.key != user_account_key {
        msg!("Provided user account is invalid");
        return Err(ProgramError::InvalidAccountData);   
    }

    let user_account_size = size_of::<UserAccount>();

    let rent = Rent::get()?;
    let ix = create_account(
        &operator_account.key,
        &user_account.key,
        rent.minimum_balance(user_account_size),
        user_account_size as u64,
        &program_id,
    );

    invoke_signed(
        &ix,
        &[
            operator_account.clone(),
            user_account.clone(),
            system_account.clone()
        ],
        &[&seeds],
    )?;

    msg!("The user account is created");

    Ok(())
}