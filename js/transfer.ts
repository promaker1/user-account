import {
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';

import { struct, u8, u32 } from '@solana/buffer-layout';
import { u64 } from '@solana/buffer-layout-utils';
import { getPayer, getRpcUrl } from './utils';
import { PROGRAM_ID, MINT_ID } from './config';

interface Data {
    instruction: number;
    senderUserId: number;
    senderBumpSeed: number;
    recipientUserId: number;
    recipientBumpSeed: number;
    amount: BigInt;
}

const DataLayout = struct<Data>(
    [u8('instruction'), 
    u32('senderUserId'), 
    u8('senderBumpSeed'), 
    u32('recipientUserId'), 
    u8('recipientBumpSeed'), 
    u64('amount')]);

const senderUserId = +process.argv[2];
const recipientUserId = +process.argv[3];
const amount = BigInt(+process.argv[4]*LAMPORTS_PER_SOL);
  
async function main() {
    console.log(`Let's transfer ${amount} from the user ${senderUserId} to ${recipientUserId}...`);

    const rpcUrl = await getRpcUrl();
    let connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
    console.log('Success');

    let payer = await getPayer();
    let lamports = await connection.getBalance(payer.publicKey);

    console.log(
      'Using account',
      payer.publicKey.toBase58(),
      'containing',
      lamports / LAMPORTS_PER_SOL,
      'SOL to pay for fees',
    );
    const senderUserIdBuf = Buffer.alloc(4);
    senderUserIdBuf.writeUInt32LE(senderUserId);

    let [senderAccountPubkey, senderBump] = await PublicKey.findProgramAddress(
      [
        senderUserIdBuf,
        MINT_ID.toBytes(),
        payer.publicKey.toBytes()
      ],  
      PROGRAM_ID
    );

    const recipientUserIdBuf = Buffer.alloc(4);
    recipientUserIdBuf.writeUInt32LE(recipientUserId);

    let [recipientAccountPubkey, recipientBump] = await PublicKey.findProgramAddress(
      [
        recipientUserIdBuf,
        MINT_ID.toBytes(),
        payer.publicKey.toBytes()
      ],  
      PROGRAM_ID
    );

    console.log(`Transferring from account ${senderAccountPubkey.toBase58()} to ${recipientAccountPubkey.toBase58()}...`);  

    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: 3,
            senderUserId: senderUserId,
            senderBumpSeed: senderBump,
            recipientUserId: recipientUserId,
            recipientBumpSeed: recipientBump,
            amount: amount,
        },
        data
    );

    const instruction = new TransactionInstruction({
        keys: [
            { pubkey: MINT_ID, isSigner: false, isWritable: false },
            { pubkey: senderAccountPubkey, isSigner: false, isWritable: true },
            { pubkey: recipientAccountPubkey, isSigner: false, isWritable: true },
            { pubkey: payer.publicKey, isSigner: true, isWritable: false },
        ],
        programId: PROGRAM_ID,
        data: data
    });

    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );

    console.log("Done.");
}
  
main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
);