import * as anchor from '@project-serum/anchor';
import { Connection, PublicKey, SystemProgram } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { TOKEN_VAULT_TAG, USER_TROVE_TAG, WSOL_MINT_KEY } from './ids';
// This command makes an Lottery
export async function createUserTrove(
  connection: Connection,
  wallet: any,
  mintCollKey:PublicKey = WSOL_MINT_KEY
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [tokenVaultKey] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey, nonceTrove] =
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
    await program.rpc.createUserTrove(nonceTrove, {
      accounts: {
        troveOwner: wallet.publicKey,
        userTrove: userTroveKey,
        tokenVault: tokenVaultKey,
        systemProgram: SystemProgram.programId
      },
    });
  }
  catch(e){
    console.log("can't create user trove")
  }
  return "created user trove successfully!";
}
