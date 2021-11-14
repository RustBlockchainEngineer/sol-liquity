import * as anchor from '@project-serum/anchor';
import * as serumCmn from "@project-serum/common";
import { Connection, PublicKey, SystemProgram } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { GLOBAL_STATE_TAG, TOKEN_VAULT_TAG, WSOL_MINT_KEY } from './ids';
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
  if(globalState){
    console.log("already created!")
    return;
  }
  
  const [tokenVaultKey, nonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const tokenVault = await program.account.tokenVault.fetch(tokenVaultKey);
  console.log("fetched tokenVault", tokenVault);
  if(tokenVault){
    console.log("This token vault was already created!")
    return;
  }

  const tokenCollKey = await serumCmn.createTokenAccount(
    program.provider,
    mintCollKey,
    tokenVaultKey
);

  try{
    await program.rpc.createTokenVault(nonce, {
      accounts: {
        vaultOwner: wallet.publicKey,
        tokenVault: tokenVaultKey,
        tokenColl: tokenCollKey,
        mintColl: mintCollKey,
        mintUSD: globalState.mintUsd,
        systemProgram: SystemProgram.programId
      },
    });
  }
  catch(e){
    console.log("can't create token vault")
  }
}
