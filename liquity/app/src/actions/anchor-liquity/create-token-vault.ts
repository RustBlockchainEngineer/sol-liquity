import * as anchor from '@project-serum/anchor';
import * as serumCmn from "@project-serum/common";
import { Connection, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { GLOBAL_STATE_TAG, TOKEN_VAULT_TAG, WSOL_MINT_KEY } from './ids';
import { createTokenAccountIfNotExist, sendTransaction } from './web3';
import { AccountLayout } from '@solana/spl-token';
// This command makes an Lottery
export async function createTokenVault(
  connection: Connection,
  wallet: any,
  mintCollKey:PublicKey = WSOL_MINT_KEY
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
  
  const [tokenVaultKey, nonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
    try{
      const tokenVault = await program.account.tokenVault.fetch(tokenVaultKey);
      console.log("fetched tokenVault", tokenVault);
      console.log("This token vault was already created!")
      return "already created";
    }
    catch(e){
    }
  

  const transaction = new Transaction()
  let instructions:TransactionInstruction[] = [];
  const signers:Keypair[] = [];
  
  let accountRentExempt = await connection.getMinimumBalanceForRentExemption(
    AccountLayout.span
    );
  const tokenCollKey = await createTokenAccountIfNotExist(
    program.provider.connection, 
    null, 
    wallet.publicKey, 
    mintCollKey.toBase58(),
    accountRentExempt,
    transaction,
    signers
    )

  try{
    const instruction = await program.instruction.createTokenVault(nonce, {
      accounts: {
        payer: wallet.publicKey,
        tokenVault: tokenVaultKey,
        globalState: globalStateKey,
        mintColl: mintCollKey,
        tokenColl: tokenCollKey,
        systemProgram: SystemProgram.programId
      },
    });
    instructions.push(instruction);
  }
  catch(e){
    console.log("can't create token vault")
  }

  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("tx id->",tx);

  return "created token vault successfully, transaction id = "+tx;
}
