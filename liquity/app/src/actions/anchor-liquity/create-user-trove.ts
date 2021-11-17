import * as anchor from '@project-serum/anchor';
import { Connection, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
// This command makes an Lottery
export async function createUserTrove(
  connection: Connection,
  wallet: any,
  mintCollKey:PublicKey = WSOL_MINT_KEY
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [tokenVaultKey, tokenVaultNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey, userTroveNonce] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );
  try{
    const userTrove = await program.account.userTrove.fetch(userTroveKey);
    console.log("fetched userTrove", userTrove);
    console.log("This user trove was already created!")
    return "already created!"; 
  }
  catch(e){
  }
  

  try{
    await program.rpc.createUserTrove(
      userTroveNonce, 
      tokenVaultNonce, 
      {
        accounts: {
          troveOwner: wallet.publicKey,
          userTrove: userTroveKey,
          tokenVault: tokenVaultKey,
          mintColl:mintCollKey,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY,
        },
      }
    );
  }
  catch(e){
    console.log("can't create user trove")
  }
  return "created user trove successfully!";
}
