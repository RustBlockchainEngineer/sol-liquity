import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey,  Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import {  GLOBAL_STATE_TAG, TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';

import { AccountLayout, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { createTokenAccountIfNotExist, sendTransaction } from './web3';
import { TokenAccount } from '../../models';
// This command makes an Lottery
export async function borrowSOLUSD(
  connection: Connection,
  wallet: any,
  amount:number,
  accountByMint: Map<string, TokenAccount>,
  mintCollKey:PublicKey = WSOL_MINT_KEY,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  

  const program = getProgramInstance(connection, wallet);

  const [globalStateKey] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
    
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

  const globalState = await program.account.globalState.fetch(globalStateKey);
  const tokenVault = await program.account.tokenVault.fetch(tokenVaultKey);

  const paramUserUsdTokenAccount = accountByMint.get(globalState.mintUsd.toBase58());
  const paramUserUsdTokenKey = paramUserUsdTokenAccount == undefined? undefined:paramUserUsdTokenAccount.pubkey;

  const transaction = new Transaction()
  let instructions:TransactionInstruction[] = [];
  const signers:Keypair[] = [];

  let accountRentExempt = await connection.getMinimumBalanceForRentExemption(
    AccountLayout.span
    );
  const userUsdTokenKey = await createTokenAccountIfNotExist(
    program.provider.connection, 
    paramUserUsdTokenKey, 
    wallet.publicKey, 
    mintCollKey.toBase58(),
    accountRentExempt,
    transaction,
    signers
  )

  
  const borrowInstruction = await program.instruction.borrowUsd(new anchor.BN(amount), {
    accounts: {
      owner: wallet.publicKey,
      userTrove: userTroveKey,
      tokenVault: tokenVaultKey,
      globalState: globalStateKey,
      poolTokenColl: tokenVault.tokenColl,
      mintUsd: globalState.mintUsd,
      userTokenUsd: userUsdTokenKey,
      tokenProgram: TOKEN_PROGRAM_ID,
    },
  });
  instructions.push(borrowInstruction);

  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("tx id->",tx);

}
