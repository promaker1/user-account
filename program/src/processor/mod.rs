//! Program instruction processor
mod process_create;
mod process_deposit;
mod process_withdraw;
mod process_transfer;
mod process_block;
mod process_unblock;

use process_create::*;
use process_deposit::*;
use process_transfer::*;
use process_withdraw::*;
use process_block::*;
use process_unblock::*;

use crate::instruction::UserAccountInstruction;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg
};

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Beginning processing");
    let instruction = UserAccountInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    msg!("Instruction unpacked");

    match instruction {
        UserAccountInstruction::Create { user_id, user_account_bump_seed } => {
            process_create(
                program_id, 
                accounts, 
                user_id, 
                user_account_bump_seed
            )
        }

        UserAccountInstruction::Deposit { user_id, user_account_bump_seed, amount } => {
            process_deposit(
                program_id, 
                accounts, 
                user_id, 
                user_account_bump_seed, 
                amount
            )
        }

        UserAccountInstruction::Withdraw { 
            user_id, 
            user_account_bump_seed, 
            source_authority_bump_seed, 
            amount 
        } => {
            process_withdraw(
                program_id, 
                accounts, 
                user_id, 
                user_account_bump_seed, 
                source_authority_bump_seed, 
                amount
            )
        }

        UserAccountInstruction::Transfer { 
            sender_user_id, 
            sender_account_bump_seed, 
            recipient_user_id, 
            recipient_account_bump_seed,
            amount 
        } => {
            process_transfer(
                program_id, 
                accounts, 
                sender_user_id, 
                sender_account_bump_seed, 
                recipient_user_id, 
                recipient_account_bump_seed,
                amount
            )
        }

        UserAccountInstruction::Block { user_id, user_account_bump_seed, amount } => {
            process_block(
                program_id, 
                accounts, 
                user_id, 
                user_account_bump_seed, 
                amount
            )
        }

        UserAccountInstruction::Unblock { user_id, user_account_bump_seed, amount } => {
            process_unblock(
                program_id, 
                accounts, 
                user_id, 
                user_account_bump_seed, 
                amount
            )
        }
    }
}