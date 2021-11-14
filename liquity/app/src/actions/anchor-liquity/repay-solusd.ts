import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import {  GLOBAL_STATE_TAG, SOLUSD_DECIMALS, TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';

import { AccountLayout, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { checkWalletATA, createTokenAccountIfNotExist, sendTransaction } from './web3';
// This command makes an Lottery
export async function repaySOLUSD(
  connection: Connection,
  wallet: any,
  amount:number,
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

  const paramUserUsdTokenKey = await checkWalletATA(connection, wallet.publicKey,globalState.mintUsd.toBase58());
  
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
  
  const repayInstruction = await program.instruction.repayUsd(new anchor.BN(amount), {
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
  instructions.push(repayInstruction);

  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("tx id->",tx);

  return "User repaid "+(amount / Math.pow(10, SOLUSD_DECIMALS))+" SOLUSD, transaction id = "+tx;
}
