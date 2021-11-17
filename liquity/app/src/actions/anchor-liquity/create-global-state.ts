import * as anchor from '@project-serum/anchor';
import { Connection, Keypair, SystemProgram,  SYSVAR_RENT_PUBKEY,  TransactionInstruction } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { GLOBAL_STATE_TAG, SOLUSD_MINT_TAG } from './ids';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
// This command makes an Lottery
export async function createGlobalState(
  connection: Connection,
  wallet: any,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);
  const [globalStateKey, globalStateNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
  const [mintUsdKey, mintUsdNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(SOLUSD_MINT_TAG)],
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
  try{
    await program.rpc.createGlobalState(
      globalStateNonce, 
      mintUsdNonce, 
      {
        accounts: {
          superOwner: wallet.publicKey,
          mintUsd: mintUsdKey,
          globalState: globalStateKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        },
        instructions:instructions,
        signers:signers
      }
    );
  }
  catch(e){
    console.log("can't create global state")
  }

  return "created global state";
}
