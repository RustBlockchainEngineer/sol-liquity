import * as anchor from '@project-serum/anchor';
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

  const [globalStateKey, nonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
  const globalState = await program.account.globalState.fetch(globalStateKey);
  console.log("fetched globalState", globalState);
  if(globalState){
    console.log("already created!")
    return;
  }

  const solusdToken = await Token.createMint(connection, wallet, globalStateKey, null, 6, TOKEN_PROGRAM_ID)

  try{
    await program.rpc.createGlobalState(nonce, {
      accounts: {
        superOwner: wallet.publicKey,
        globalState: globalStateKey,
        mintUsd: solusdToken.publicKey,
        systemProgram: SystemProgram.programId,
      },
    });
  }
  catch(e){
    console.log("can't create global state")
  }
}
