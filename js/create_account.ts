import {
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';

import { struct, u8, u32 } from '@solana/buffer-layout';
import { getPayer, getRpcUrl } from './utils';
import { PROGRAM_ID, MINT_ID } from './config';

interface Data {
    instruction: number;
    userId: number;
    bumpSeed: number;
}

const DataLayout = struct<Data>([u8('instruction'), u32('userId'), u8('bumpSeed')]);

const userId = +process.argv[2];
  
async function main() {
    console.log("Let's create an account for the given program...");

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

    console.log(`Creating account ${accountPubkey.toBase58()} with bump ${bump}...`);  

    const data = Buffer.alloc(DataLayout.span);
    DataLayout.encode(
        {
            instruction: 0,
            userId: userId,
            bumpSeed: bump
        },
        data
    );

    const instruction = new TransactionInstruction({
        keys: [
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            { pubkey: accountPubkey, isSigner: false, isWritable: true },
            { pubkey: MINT_ID, isSigner: false, isWritable: false },
            { pubkey: payer.publicKey, isSigner: true, isWritable: true },
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