#![cfg(feature = "test-bpf")]
mod program_test;

use solana_program_test::*;

use solana_program::{
    instruction::InstructionError,
};

use solana_sdk::{
    signature::Signer,
    transaction::{Transaction, TransactionError}
};

use emotion_user_account::{
    instruction::{deposit, withdraw, transfer, block, unblock},
};

use program_test::UserAccountProgramTest;

use spl_associated_token_account::{ 
    get_associated_token_address,
    create_associated_token_account
};

#[tokio::test]
async fn test_user_account() {
    
    let user_id = 100u32;
    let amount = 5000000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    let (user_account_key, user_bump) = program_test.with_user(user_id, 0).await;
    
    let deposit_instruction = deposit(
        &program_test.program_id, 
        user_id, 
        user_bump, 
        amount,
        &user_account_key,
        &token_mint_key, 
        &program_test.context.payer.pubkey()
    ).unwrap();

    let block_instruction = block(
        &program_test.program_id, 
        user_id, 
        user_bump, 
        amount,
        &user_account_key,
        &token_mint_key, 
        &program_test.context.payer.pubkey()
    ).unwrap();

    let unblock_instruction = unblock(
        &program_test.program_id, 
        user_id, 
        user_bump, 
        amount,
        &user_account_key,
        &token_mint_key, 
        &program_test.context.payer.pubkey()
    ).unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[
            deposit_instruction, 
            block_instruction,
            unblock_instruction
        ], Some(&program_test.context.payer.pubkey())
    );
    transaction.sign(&[&program_test.context.payer], program_test.context.last_blockhash);
    program_test.context.banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn test_withdraw() {
    let mint_amount = 100000000000u64;
    let user_id = 100u32;
    let deposit_amount = 5000000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    
    let (source_account_key, source_token_account_key, source_bump) = 
        program_test.with_source_user(mint_amount).await;
    
    let (user_account_key, user_bump) = 
        program_test.with_user(user_id, deposit_amount).await;

    let withdraw_transaction = Transaction::new_signed_with_payer(
        &[
            create_associated_token_account(
                &program_test.context.payer.pubkey(),
                &program_test.context.payer.pubkey(),
                &token_mint_key
            ),
            withdraw(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                source_bump,
                deposit_amount,
                &user_account_key,
                &token_mint_key,
                &source_token_account_key,
                &source_account_key,
                &get_associated_token_address(&program_test.context.payer.pubkey(), &token_mint_key),
                &program_test.context.payer.pubkey()
            ).unwrap()
        ], 
        Some(&program_test.context.payer.pubkey()),
        &[&program_test.context.payer],
        program_test.context.last_blockhash
    );
    program_test.context.banks_client.process_transaction(withdraw_transaction).await.unwrap();
}

#[tokio::test]
async fn test_transfer() {
    let sender_user_id = 100u32;
    let recipient_user_id = 101u32;
    let deposit_amount = 5000000000u64;
    let transfer_amount = 1000000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    
    let (sender_account_key, sender_bump) = 
        program_test.with_user(sender_user_id, deposit_amount).await;

    let (recipient_account_key, recipient_bump) = 
        program_test.create_user_account(recipient_user_id, 0).await;

    let transfer_transaction = Transaction::new_signed_with_payer(
        &[
            transfer(
                &program_test.program_id, 
                &token_mint_key,
                sender_user_id, 
                sender_bump,
                &sender_account_key,
                recipient_user_id, 
                recipient_bump,
                &recipient_account_key, 
                transfer_amount,
                &program_test.context.payer.pubkey()
            ).unwrap()
        ], 
        Some(&program_test.context.payer.pubkey()),
        &[&program_test.context.payer],
        program_test.context.last_blockhash
    );
    program_test.context.banks_client.process_transaction(transfer_transaction).await.unwrap();
}

#[tokio::test]
async fn test_block_too_big_amount_error() {
    let user_id = 100u32;
    let deposit_amount = 5000000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    
    let (user_account_key, user_bump) = 
        program_test.with_user(user_id, deposit_amount).await;
    
    let block_amount = deposit_amount - 100;

    let block_transaction = Transaction::new_signed_with_payer(
        &[
            block(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                block_amount,
                &user_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap(),
            block(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                50,
                &user_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap(),
            block(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                51,
                &user_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap()
        ], 
        Some(&program_test.context.payer.pubkey()),
        &[&program_test.context.payer],
        program_test.context.last_blockhash
    );

    assert_eq!(
        program_test.context.banks_client
            .process_transaction(block_transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(2, InstructionError::InvalidArgument)
    );
}

#[tokio::test]
async fn test_unblock_too_big_amount_error() {
    let user_id = 100u32;
    let deposit_amount = 5000000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    
    let (user_account_key, user_bump) = 
        program_test.with_user(user_id, deposit_amount).await;
    
    let block_amount = deposit_amount - 100;

    let block_transaction = Transaction::new_signed_with_payer(
        &[
            block(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                block_amount,
                &user_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap(),
            unblock(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                block_amount + 1,
                &user_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap()
        ], 
        Some(&program_test.context.payer.pubkey()),
        &[&program_test.context.payer],
        program_test.context.last_blockhash
    );

    assert_eq!(
        program_test.context.banks_client
            .process_transaction(block_transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(1, InstructionError::InvalidArgument)
    );
}

#[tokio::test]
async fn test_withdraw_too_big_amount_error() {
    let mint_amount = 100000000000u64;
    let user_id = 100u32;
    let deposit_amount = 5000000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    
    let (source_account_key, source_token_account_key, source_bump) = 
        program_test.with_source_user(mint_amount).await;
    
    let (user_account_key, user_bump) = 
        program_test.with_user(user_id, deposit_amount).await;

    let withdraw_transaction = Transaction::new_signed_with_payer(
        &[
            create_associated_token_account(
                &program_test.context.payer.pubkey(),
                &program_test.context.payer.pubkey(),
                &token_mint_key
            ),
            block(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                1,
                &user_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap(),
            withdraw(
                &program_test.program_id, 
                user_id, 
                user_bump, 
                source_bump,
                deposit_amount,
                &user_account_key,
                &token_mint_key,
                &source_token_account_key,
                &source_account_key,
                &get_associated_token_address(&program_test.context.payer.pubkey(), &token_mint_key),
                &program_test.context.payer.pubkey()
            ).unwrap()
        ], 
        Some(&program_test.context.payer.pubkey()),
        &[&program_test.context.payer],
        program_test.context.last_blockhash
    );

    assert_eq!(
        program_test.context.banks_client
            .process_transaction(withdraw_transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(2, InstructionError::InvalidArgument)
    );
}

#[tokio::test]
async fn test_transfer_too_big_amount_error() {
    let sender_user_id = 100u32;
    let recipient_user_id = 101u32;
    let deposit_amount = 5000000000u64;
    let block_amount = 3000000000u64;
    let transfer_amount = 2100000000u64;

    let mut program_test = UserAccountProgramTest::start_new().await;
    
    let token_mint_key = program_test.with_token_mint().await;
    
    let (sender_account_key, sender_bump) = 
        program_test.with_user(sender_user_id, deposit_amount).await;
    let (recipient_account_key, recipient_bump) =
        program_test.create_user_account(recipient_user_id, 0).await;

    let transfer_transaction = Transaction::new_signed_with_payer(
        &[
            block(
                &program_test.program_id, 
                sender_user_id, 
                sender_bump, 
                block_amount,
                &sender_account_key,
                &token_mint_key, 
                &program_test.context.payer.pubkey()
            ).unwrap(),
            transfer(
                &program_test.program_id, 
                &token_mint_key,
                sender_user_id, 
                sender_bump,
                &sender_account_key,
                recipient_user_id, 
                recipient_bump,
                &recipient_account_key, 
                transfer_amount,
                &program_test.context.payer.pubkey()
            ).unwrap()
        ], 
        Some(&program_test.context.payer.pubkey()),
        &[&program_test.context.payer],
        program_test.context.last_blockhash
    );

    assert_eq!(
        program_test.context.banks_client
            .process_transaction(transfer_transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(1, InstructionError::InvalidArgument)
    );
}
