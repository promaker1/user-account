use crate::state::UserAccount;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg
};
use spl_token::instruction::transfer;
use spl_associated_token_account::get_associated_token_address;

pub fn process_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo], 
    user_id: u32,
    user_account_bump_seed: u8, 
    source_authority_bump_seed: u8, 
    amount: u64) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let token_program_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let token_mint_account = next_account_info(accounts_iter)?;
    let source_account = next_account_info(accounts_iter)?;
    let source_authority_account = next_account_info(accounts_iter)?;
    let destination_account = next_account_info(accounts_iter)?;
    let operator_account = next_account_info(accounts_iter)?;

    if !operator_account.is_signer {
        msg!("Provided operator account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);   
    }
    
    let user_account_seeds = [
        &user_id.to_le_bytes()[..], 
        &token_mint_account.key.to_bytes(), 
        &operator_account.key.to_bytes(), 
        &[user_account_bump_seed]
    ];

    let user_account_key = Pubkey::create_program_address(
        &user_account_seeds, 
        &program_id)?;

    if *user_account.key != user_account_key {
        msg!("Provided user account is invalid");
        return Err(ProgramError::InvalidAccountData);   
    }

    let source_authority_account_seeds = [
        &(0 as u32).to_le_bytes()[..], 
        &token_mint_account.key.to_bytes(), 
        &operator_account.key.to_bytes(), 
        &[source_authority_bump_seed]
    ];

    let source_authority_account_key = Pubkey::create_program_address(
        &source_authority_account_seeds, 
        &program_id)?;

    if *source_authority_account.key != source_authority_account_key {
        msg!("Provided source authority account is invalid");
        return Err(ProgramError::InvalidAccountData); 
    }

    let source_account_key = get_associated_token_address(
        &source_authority_account_key, 
        token_mint_account.key);

    if *source_account.key != source_account_key {
        msg!("Provided source account is invalid");
        return Err(ProgramError::InvalidAccountData); 
    }

    let mut user_account_object = UserAccount::try_from_slice(&user_account.data.borrow())?;
    msg!("Updating user account [id={}, balance={}, blocked_amount={}]", 
        user_id, user_account_object.balance, user_account_object.blocked_amount);

    if (user_account_object.balance - user_account_object.blocked_amount) < amount {
        msg!("The given amount is greater than the available balance");
        return Err(ProgramError::InvalidArgument);   
    }

    let ix = transfer(
        token_program_account.key, 
        source_account.key, 
        destination_account.key, 
        source_authority_account.key, 
        &[],
        amount
    )?;

    invoke_signed(
        &ix,
        &[
            source_account.clone(), 
            destination_account.clone(), 
            source_authority_account.clone(), 
            token_program_account.clone()
        ],
        &[&source_authority_account_seeds],
    )?;

    user_account_object.balance -= amount;
    user_account_object.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

    msg!("Withdrawn {}. Updated user account [id={}, balance={}, blocked_amount={}]", 
        amount, user_id, user_account_object.balance, user_account_object.blocked_amount);

    Ok(())
}