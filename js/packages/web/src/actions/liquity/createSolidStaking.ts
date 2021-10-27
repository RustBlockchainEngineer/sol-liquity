import {
  Connection,
  Keypair,
  SystemProgram,
  TransactionInstruction,
} from '@solana/web3.js';
import {
  WalletSigner,
  sendTransactionWithRetry,
  toPublicKey,
  sizeOfState,
  programIds,
  SOLIDStaking,
  createSolidStakingInstruction,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function createSolidStaking(
  connection: Connection,
  wallet: WalletSigner,
): Promise<{
  txid: string;
  slot: number;
}> {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const instructions: TransactionInstruction[] = [];

  const signers: Keypair[] = [];

  const solidStakingKey = new Keypair();

  const span = sizeOfState(SOLIDStaking);
  const rentExempt = await connection.getMinimumBalanceForRentExemption(span);

  instructions.push(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: solidStakingKey.publicKey,
      lamports: rentExempt,
      space: span,
      programId: toPublicKey(programIds().borrowerOperations),
    }),
  );

  await createSolidStakingInstruction(
    solidStakingKey.publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    instructions,
  );

  signers.push(solidStakingKey);

  const { txid, slot } = await sendTransactionWithRetry(
    connection,
    wallet,
    instructions,
    signers,
    'confirmed',
  );

  return { txid, slot };
}
