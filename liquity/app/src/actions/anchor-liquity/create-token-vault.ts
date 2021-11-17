import * as anchor from '@project-serum/anchor';
import { Connection, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { GLOBAL_STATE_TAG, TOKEN_VAULT_POOL_TAG, TOKEN_VAULT_TAG, WSOL_MINT_KEY } from './ids';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
// This command makes an Lottery
export async function createTokenVault(
  connection: Connection,
  wallet: any,
  mintCollKey: PublicKey = WSOL_MINT_KEY
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [globalStateKey, globalStateNonce] =
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
  try {
    const tokenVault = await program.account.tokenVault.fetch(tokenVaultKey);
    console.log("fetched tokenVault", tokenVault);
    console.log("This token vault was already created!")
    return "already created";
  }
  catch (e) {
  }

  try {
    await program.rpc.createTokenVault(
      tokenVaultNonce, 
      globalStateNonce, 
      tokenCollNonce, 
      {
        accounts: {
          payer: wallet.publicKey,
          tokenVault: tokenVaultKey,
          globalState: globalStateKey,
          mintColl: mintCollKey,
          tokenColl: tokenCollKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        },
      }
    );
  }
  catch (e) {
    console.log("can't create token vault")
  }
  return "created token vault successfully" ;
}
