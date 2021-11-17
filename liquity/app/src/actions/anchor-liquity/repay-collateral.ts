import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import {  GLOBAL_STATE_TAG, TOKEN_VAULT_POOL_TAG, TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';

import { closeAccount } from '@project-serum/serum/lib/token-instructions'
import { AccountLayout, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { createTokenAccountIfNotExist, sendTransaction } from './web3';
// This command makes an Lottery
export async function repayCollateral(
  connection: Connection,
  wallet: any,
  amount:number,
  userCollAddress: string | null = null,
  mintCollKey:PublicKey = WSOL_MINT_KEY,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [globalStateKey] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
  const globalState = await program.account.globalState.fetch(globalStateKey);
  console.log("fetched globalState", globalState);

  const [tokenVaultKey, tokenVaultNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [tokenCollKey, tokenCollNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_POOL_TAG), tokenVaultKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey, userTroveNonce] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );

  const transaction = new Transaction()
  let instructions:TransactionInstruction[] = [];
  const signers:Keypair[] = [];

  let userCollKey = null;

  if (mintCollKey.toBase58() === WSOL_MINT_KEY.toBase58()) {
    let accountRentExempt = await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
      );
    userCollKey = await createTokenAccountIfNotExist(
      program.provider.connection, 
      null, 
      wallet.publicKey, 
      mintCollKey.toBase58(),
      accountRentExempt + amount,
      transaction,
      signers
      )
  }
  else if(userCollAddress == null){
    console.log("user doesn't have any collateral");
    return "user doesn't have any collateral";
  }
  
  const depositInstruction = await program.instruction.repayCollateral(
    new anchor.BN(amount), 
    tokenVaultNonce,
    userTroveNonce,
    tokenCollNonce,
    {
      accounts: {
        owner: wallet.publicKey,
        userTrove: userTroveKey,
        tokenVault: tokenVaultKey,
        poolTokenColl: tokenCollKey,
        userTokenColl: userCollKey,
        mintColl: mintCollKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    }
  );
  instructions.push(depositInstruction);

  

  if (mintCollKey.toBase58() === WSOL_MINT_KEY.toBase58()) {
    instructions.push(
      closeAccount({
        source: userCollKey,
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

  return "User repaid "+(amount / Math.pow(10, 9))+" SOL, transaction id = "+tx;
}
