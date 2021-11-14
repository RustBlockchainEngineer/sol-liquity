import * as anchor from '@project-serum/anchor';
import * as serumCmn from "@project-serum/common";
import { Connection, SystemProgram } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { GLOBAL_STATE_TAG } from './ids';
// This command makes an Lottery
export async function createGlobalState(
  connection: Connection,
  wallet: any,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);
console.log(program)
  const [globalStateKey, nonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
  try{
    const globalState = await program.account.globalState.fetch(globalStateKey);
    console.log("already created")
    console.log("globalState",globalState);
    return;
  }
  catch(e){
    console.log(e)
  }
  
  const solusdToken = await serumCmn.createMint(program.provider,globalStateKey,6);
  try{
    await program.rpc.createGlobalState(nonce, {
      accounts: {
        superOwner: wallet.publicKey,
        globalState: globalStateKey,
        mintUsd: solusdToken,
        systemProgram: SystemProgram.programId,
      },
    });
  }
  catch(e){
    console.log("can't create global state")
  }
  console.log("created global state=",globalStateKey.toBase58())
}
