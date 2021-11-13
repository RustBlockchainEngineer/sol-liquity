import * as anchor from '@project-serum/anchor';
import { Connection, SystemProgram } from '@solana/web3.js';
import { WalletSigner } from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

import { getProgramInstance } from './get-program';
// This command makes an Lottery
export async function borrowSOLUSD(
  connection: Connection,
  wallet: WalletSigner,
) {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const program = getProgramInstance(connection, wallet);

  const [globalStateKey, nonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from('golbal-state-seed')],
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
