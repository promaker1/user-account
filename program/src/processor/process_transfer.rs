use crate::state::UserAccount;

use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg
};

pub fn process_transfer(
    program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    sender_user_id: u32, 
    sender_account_bump_seed: u8, 
    recipient_user_id: u32, 
    recipient_account_bump_seed: u8,
    amount: u64) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let token_mint_account = next_account_info(accounts_iter)?;
    let sender_account = next_account_info(accounts_iter)?;
    let recipient_account = next_account_info(accounts_iter)?;
    let operator_account = next_account_info(accounts_iter)?;

    if !operator_account.is_signer {
        msg!("Provided operator account is not a signer");
        return Err(ProgramError::MissingRequiredSignature);   
    }
    
    let sender_seeds = [
        &sender_user_id.to_le_bytes()[..], 
        &token_mint_account.key.to_bytes(), 
        &operator_account.key.to_bytes(), 
        &[sender_account_bump_seed]
    ];

    let sender_account_key = Pubkey::create_program_address(
        &sender_seeds, 
        &program_id)?;
        

    if *sender_account.key != sender_account_key {
        msg!("Provided sender account is invalid");
        return Err(ProgramError::InvalidAccountData);   
    }

    let recipient_seeds = [
        &recipient_user_id.to_le_bytes()[..], 
        &token_mint_account.key.to_bytes(), 
        &operator_account.key.to_bytes(), 
        &[recipient_account_bump_seed]
    ];

    let recipient_account_key = Pubkey::create_program_address(
        &recipient_seeds, 
        &program_id)?;

    if *recipient_account.key != recipient_account_key {
        msg!("Provided recipient account is invalid");
        return Err(ProgramError::InvalidAccountData);   
    }

    let mut sender_account_object = UserAccount::try_from_slice(&sender_account.data.borrow())?;
    msg!("Updating sender account [id={}, balance={}, blocked_amount={}]", 
        sender_user_id, sender_account_object.balance, sender_account_object.blocked_amount);

    if (sender_account_object.balance - sender_account_object.blocked_amount) < amount {
        msg!("The given amount is greater than the available balance");
        return Err(ProgramError::InvalidArgument);   
    }

    sender_account_object.balance -= amount;
    sender_account_object.serialize(&mut &mut sender_account.data.borrow_mut()[..])?;

    msg!("Sent {}. Updated sender account [id={}, balance={}, blocked_amount={}]", 
        amount, sender_user_id, sender_account_object.balance, sender_account_object.blocked_amount);

    let mut recipient_account_object = UserAccount::try_from_slice(&recipient_account.data.borrow())?;
    msg!("Updating recipient account [id={}, balance={}, blocked_amount={}]", 
        recipient_user_id, recipient_account_object.balance, recipient_account_object.blocked_amount);

    recipient_account_object.balance += amount;
    recipient_account_object.serialize(&mut &mut recipient_account.data.borrow_mut()[..])?;

    msg!("Received {}. Updated recipient account [id={}, balance={}, blocked_amount={}]", 
        amount, recipient_user_id, recipient_account_object.balance, recipient_account_object.blocked_amount);

    Ok(())
}