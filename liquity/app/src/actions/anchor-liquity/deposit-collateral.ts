import * as anchor from '@project-serum/anchor';
import * as serumCmn from "@project-serum/common";
import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, SendTransactionError, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import {  TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';

import { closeAccount } from '@project-serum/serum/lib/token-instructions'
import { AccountLayout, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { createTokenAccountIfNotExist, sendTransaction } from './web3';
// This command makes an Lottery
export async function depositCollateral(
  connection: Connection,
  wallet: any,
  amount:number,
  mintCollKey:PublicKey = WSOL_MINT_KEY,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [tokenVaultKey] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );

  const tokenVault = await program.account.tokenVault.fetch(tokenVaultKey);

  const transaction = new Transaction()
  let instructions:TransactionInstruction[] = [];
  const signers:Keypair[] = [];

  let wrappedSolKey = null;
  if (mintCollKey.toBase58() === WSOL_MINT_KEY.toBase58()) {
    let accountRentExempt = await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
      );
      wrappedSolKey = await createTokenAccountIfNotExist(
      program.provider.connection, 
      null, 
      wallet.publicKey, 
      mintCollKey.toBase58(),
      accountRentExempt+amount,
      transaction,
      signers
      )
  }
  
  const depositInstruction = await program.instruction.depositCollateral(new anchor.BN(amount), {
    accounts: {
      owner: wallet.publicKey,
      userTrove: userTroveKey,
      tokenVault: tokenVaultKey,
      poolTokenColl: tokenVault.tokenColl,
      userTokenColl: wrappedSolKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    },
  });
  instructions.push(depositInstruction);

  

  if (mintCollKey.toBase58() === WSOL_MINT_KEY.toBase58()) {
    instructions.push(
      closeAccount({
        source: wrappedSolKey,
        destination: wallet.publicKey,
        owner:wallet.publicKey
      })
    )
  }
  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("tx id->",tx);

  return "User deposited "+(amount / Math.pow(10, 9))+" SOL, transaction id = "+tx;
}
