import * as anchor from '@project-serum/anchor';
import { Connection, SystemProgram } from '@solana/web3.js';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
import { GLOBAL_STATE_TAG } from './ids';
// This command makes an Lottery
export async function depositCollateral(
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

  await program.rpc.createGlobalState(nonce, {
    accounts: {
      superOwner: wallet.publicKey,
      globalState: globalStateKey,
      systemProgram: SystemProgram.programId,
    },
  });
}
