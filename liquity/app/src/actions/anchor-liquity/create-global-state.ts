import * as anchor from '@project-serum/anchor';
import * as serumCmn from "@project-serum/common";
import { Connection, Keypair, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { GLOBAL_STATE_TAG, SOLUSD_DECIMALS } from './ids';
import { sendTransaction } from './web3';
import { MintLayout, Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { initializeMint } from '@project-serum/serum/lib/token-instructions';
// This command makes an Lottery
export async function createGlobalState(
  connection: Connection,
  wallet: any,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);
  const [globalStateKey, nonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
  try{
    const globalState = await program.account.globalState.fetch(globalStateKey);
    console.log("already created")
    console.log("globalState",globalState);
    return "already created";
  }
  catch(e){
    console.log(e)
  }
  
  let instructions:TransactionInstruction[] = [];
  const signers:Keypair[] = [];
  
  const solusdTokenKeypair = new Keypair();
  
  instructions.push(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: solusdTokenKeypair.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(MintLayout.span),
      space: MintLayout.span,
      programId:TOKEN_PROGRAM_ID
    })
  );

  instructions.push(
    initializeMint({
      mint: solusdTokenKeypair.publicKey,
      decimals: SOLUSD_DECIMALS,
      mintAuthority:globalStateKey,
    })
  )
  signers.push(solusdTokenKeypair);

  try{
    await program.rpc.createGlobalState(nonce, {
      accounts: {
        superOwner: wallet.publicKey,
        globalState: globalStateKey,
        mintUsd: solusdTokenKeypair.publicKey,
        systemProgram: SystemProgram.programId,
      },
      instructions:instructions,
      signers:signers
    });

    try{
      const globalState = await program.account.globalState.fetch(globalStateKey);
      console.log("already created")
      console.log("globalState",globalState);
      return "already created";
    }
    catch(e){
      console.log(e)
    }
  }
  catch(e){
    console.log("can't create global state")
  }

  return "created global state";
}
