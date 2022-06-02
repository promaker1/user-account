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
    userId: number;
    bumpSeed: number;
    amount: BigInt;
}

const DataLayout = struct<Data>([u8('instruction'), u32('userId'), u8('bumpSeed'), u64('amount')]);

const userId = +process.argv[2];
const amount = BigInt(+process.argv[3]*LAMPORTS_PER_SOL);
  
async function main() {
    console.log(`Let's block ${amount} for the given user account...`);

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
    const userIdBuf = Buffer.alloc(4);
    userIdBuf.writeUInt32LE(userId);

    let [accountPubkey, bump] = await PublicKey.findProgramAddress(
      [
          userIdBuf,
          MINT_ID.toBytes(),
          payer.publicKey.toBytes()
      ],  
      PROGRAM_ID
    );

    console.log(`Blocking ${amount} for the account ${accountPubkey.toBase58()}...`);  

    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: 4,
            userId: userId,
            bumpSeed: bump,
            amount: amount,
        },
        data
    );

    const instruction = new TransactionInstruction({
        keys: [
            { pubkey: accountPubkey, isSigner: false, isWritable: true },
            { pubkey: MINT_ID, isSigner: false, isWritable: false },
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