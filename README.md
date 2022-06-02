# Install the development environment 
1. Install Rust development environment (see [https://www.rust-lang.org/tools/install]).
2. Install Solana SDK and other tools (see [https://docs.solana.com/ru/cli/install-solana-cli-tools]).
3. Install NodeJs.

# Build and deploy the program emotion-user-account
1. From the folder */user-account/program/*, run ```cargo build-bpf```
2. *(optional)* From the folder */user-account/program/*, run ```cargo test-bpf``` to make sure that all the automatic tests passed successfully
3. Make sure you have enough balance to deploy the program. For this purpose, run ```solana balance```. 
**NOTE:** If you're in a dev, or test mode you can refill your balance running ```solana airdrop 1``` as many times as you need.
4. To deploy this program, run ```solana program deploy <an absolute path to the local repository>/user-account/program/target/deploy/emotion_user_account.so```. Save the program id for further use.

# Configure the program emotion-user-account and the JS client
1. To create a new token run ```spl-token create-token```. Save the token address for further use.
2. Open and edit /user-account/js/config.ts. Set the values for ```PROGRAM_ID``` and ```MINT_ID``` variables to the program id and token address correspondingly (see the recent steps).
3. From the folder */user-account/js/*, run ```npm install```
3. Create an associated token account for your current wallet with this command ```spl-token create-account <TOKEN_ADDRESS>```
4. Mint the required amount of tokens to your wallet with this command ```spl-token mint <TOKEN_ADDRESS> <AMOUNT>```.
5. Create a source account for the program on behalf of your wallet. A source account serves as the custodial for all the users' tokens and are used to withdraw the tokens from the program to the users' wallets. To create this account, run the command ```npm run create-account 0``` from the folder */user-account/js/*. Save the source account address for further use.
6. Top-up the balance of the source account. To do so, you have to transfer the tokens from your current wallet to this account. It can be done with this command ```spl-token transfer <TOKEN_ADDRESS> <AMOUNT> <SOURCE_ACCOUNT_ADDRESS> --fund-recipient```.

# Run the program instructions
The program supports the following instructions:
* Create a new user account
```npm run create-account <user_id>```
* Deposit
```npm run deposit-account <user_id> <amount>```
* Withdraw
```npm run withdraw-account <user_id> <amount>```
* Transfer
```npm run transfer <sender_id> <recipient_id> <amount>```
* Block
```npm run block-account <user_id> <amount>```
* Unblock
```npm run unblock-account <user_id> <amount>```
