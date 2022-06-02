use crate::state::UserAccount;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg
};

pub fn process_block(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    user_id: u32, 
    user_account_bump_seed: u8,
    amount: u64) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

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

    let mut user_account_object = UserAccount::try_from_slice(&user_account.data.borrow())?;
    msg!("Updating user account [id={}, balance={}, blocked_amount={}]", 
        user_id, user_account_object.balance, user_account_object.blocked_amount);

    if user_account_object.blocked_amount + amount > user_account_object.balance {
        msg!("The given amount is greater than the available balance");
        return Err(ProgramError::InvalidArgument);   
    }

    user_account_object.blocked_amount += amount;
    user_account_object.serialize(&mut &mut user_account.data.borrow_mut()[..])?;

    msg!("Blocked {}. Updated user account [id={}, balance={}, blocked_amount={}]", 
        amount, user_id, user_account_object.balance, user_account_object.blocked_amount);

    Ok(())
}