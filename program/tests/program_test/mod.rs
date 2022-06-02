use std::str::FromStr;

use solana_program::{
    system_instruction::create_account,
    pubkey::Pubkey
};

use solana_program_test::*;

use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    transaction::Transaction
};

use spl_token::{ 
    instruction::{initialize_mint, mint_to},
    state::Mint
};

use emotion_user_account::{
    entrypoint::process_instruction,
    instruction::{create, deposit},
};

use spl_associated_token_account::{ 
    get_associated_token_address,
    create_associated_token_account
};

pub struct UserAccountProgramTest {
    pub program_id: Pubkey,
    pub context: ProgramTestContext,

    pub token_mint_key: Option<Pubkey>,
    
    pub user_account_key: Option<Pubkey>,
    pub user_id: u32,
    pub user_bump: u8,
    
    pub source_account_key: Option<Pubkey>,
    pub source_token_account_key: Option<Pubkey>,
    pub source_bump: u8
}

impl UserAccountProgramTest {
    pub async fn start_new() -> Self {
        let program_id = Pubkey::from_str("7b4tT3royc2qTcNFKPgryT7gv6K1fKxDDWs1edD4qFht").unwrap();
    
        let program_test = ProgramTest::new(
            "emotion_user_account", 
            program_id, 
            processor!(process_instruction)
        );

        let ctx = program_test.start_with_context().await;

        Self {
            program_id: program_id,
            context: ctx,

            token_mint_key: None,
            
            user_account_key: None,
            user_id: 0,
            user_bump: 0,

            source_account_key: None,
            source_token_account_key: None,
            source_bump: 0
        }
    }

    pub async fn with_token_mint(&mut self) -> Pubkey {
        let pool_mint = Keypair::new();
    
        let rent = self.context.banks_client.get_rent().await.unwrap();
        let mint_rent = rent.minimum_balance(Mint::LEN);

        let transaction = Transaction::new_signed_with_payer(
            &[
                create_account(
                    &self.context.payer.pubkey(),
                    &pool_mint.pubkey(),
                    mint_rent,
                    Mint::LEN as u64,
                    &spl_token::id(),
                ),    
                initialize_mint(
                    &spl_token::id(),
                    &pool_mint.pubkey(),
                    &self.context.payer.pubkey(),
                    None,
                    9
                ).unwrap()
            ], 
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer, &pool_mint],
            self.context.last_blockhash
        );

        self.context.banks_client.process_transaction(transaction).await.unwrap();

        self.token_mint_key = Some(pool_mint.pubkey());

        return pool_mint.pubkey().clone();
    }

    pub async fn create_user_account(&mut self, user_id: u32, amount: u64) -> (Pubkey, u8) {
        let seeds = [
            &user_id.to_le_bytes()[..], 
            &self.token_mint_key.unwrap().to_bytes(), 
            &self.context.payer.pubkey().to_bytes(), 
        ];

        let (user_account_key, user_bump) = Pubkey::find_program_address(
            &seeds, 
            &self.program_id);
        
        let mut instructions = vec![
            create(
                &self.program_id, 
                user_id, 
                user_bump, 
                &user_account_key,
                &self.token_mint_key.unwrap(), 
                &self.context.payer.pubkey()
            ).unwrap()
        ];

        if amount > 0 {
            instructions.push(
                deposit(
                    &self.program_id, 
                    user_id, 
                    user_bump, 
                    amount,
                    &user_account_key,
                    &self.token_mint_key.unwrap(), 
                    &self.context.payer.pubkey()
                ).unwrap()
            );
        }

        let transaction = Transaction::new_signed_with_payer(
            &instructions, 
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer],
            self.context.last_blockhash
        );

        self.context.banks_client.process_transaction(transaction).await.unwrap();

        return (user_account_key.clone(), user_bump);
    }

    pub async fn with_user(&mut self, user_id: u32, amount: u64) -> (Pubkey, u8) {
        let (user_account_key, user_bump) = self.create_user_account(user_id, amount).await;

        self.user_id = user_id;
        self.user_account_key = Some(user_account_key);
        self.user_bump = user_bump;

        return (user_account_key.clone(), user_bump);
    }

    pub async fn with_source_user(&mut self, mint_amount: u64) -> (Pubkey, Pubkey, u8) {
        let source_user_id = 0u32;
        
        let source_seeds = [
            &source_user_id.to_le_bytes()[..], 
            &self.token_mint_key.unwrap().to_bytes(), 
            &self.context.payer.pubkey().to_bytes(), 
        ];
    
        let (source_account_key, source_bump) = Pubkey::find_program_address(
            &source_seeds, 
            &self.program_id);
    
        let source_token_account_key = get_associated_token_address(
            &source_account_key, &self.token_mint_key.unwrap()
        );
    
        let create_source_and_mint_transaction = Transaction::new_signed_with_payer(
            &[
                create(
                    &self.program_id, 
                    source_user_id, 
                    source_bump, 
                    &source_account_key,
                    &self.token_mint_key.unwrap(), 
                    &self.context.payer.pubkey()
                ).unwrap(),
                create_associated_token_account(
                    &self.context.payer.pubkey(),
                    &source_account_key,
                    &self.token_mint_key.unwrap()
                ),
                mint_to(
                    &spl_token::id(), 
                    &self.token_mint_key.unwrap(), 
                    &source_token_account_key, 
                    &self.context.payer.pubkey(), 
                    &[],
                    mint_amount
                ).unwrap()
            ], 
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer],
            self.context.last_blockhash
        );
        self.context.banks_client.process_transaction(create_source_and_mint_transaction).await.unwrap();

        self.source_account_key = Some(source_account_key);
        self.source_token_account_key = Some(source_token_account_key);
        self.source_bump = source_bump;

        return (source_account_key.clone(), source_token_account_key.clone(), source_bump);
    }
}