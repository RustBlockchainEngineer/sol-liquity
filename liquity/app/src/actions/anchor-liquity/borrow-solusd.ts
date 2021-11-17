import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey,  Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import {  GLOBAL_STATE_TAG, SOLUSD_DECIMALS, SOLUSD_MINT_TAG,  TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';

import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { checkWalletATA, createTokenAccountIfNotExist, sendTransaction } from './web3';
// This command makes an Lottery
export async function borrowSOLUSD(
  connection: Connection,
  wallet: any,
  amount:number,
  mintCollKey:PublicKey = WSOL_MINT_KEY,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [globalStateKey, globalStateNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
    
  const [tokenVaultKey, tokenVaultNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey, userTroveNonce] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );
  const [mintUsdKey, mintUsdNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(SOLUSD_MINT_TAG)],
      program.programId,
    );

  const globalState = await program.account.globalState.fetch(globalStateKey);

  const paramUserUsdTokenKey = await checkWalletATA(connection, wallet.publicKey,globalState.mintUsd.toBase58());

  const transaction = new Transaction()
  let instructions:TransactionInstruction[] = [];
  const signers:Keypair[] = [];

  const userUsdTokenKey = await createTokenAccountIfNotExist(
    connection, 
    paramUserUsdTokenKey, 
    wallet.publicKey, 
    globalState.mintUsd.toBase58(),
    null,
    transaction,
    signers
  )
  
  const borrowInstruction = await program.instruction.borrowUsd(
    new anchor.BN(amount), 
    tokenVaultNonce,
    userTroveNonce,
    globalStateNonce,
    mintUsdNonce,
    {
      accounts: {
        owner: wallet.publicKey,
        tokenVault: tokenVaultKey,
        userTrove: userTroveKey,
        globalState: globalStateKey,
        mintUsd: mintUsdKey,
        userTokenUsd: userUsdTokenKey,
        mintColl: mintCollKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    }
  );
  instructions.push(borrowInstruction);

  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("tx id->",tx);

  return "User borrowed "+(amount / Math.pow(10, SOLUSD_DECIMALS))+" SOLUSD , transaction id = "+tx;

}
