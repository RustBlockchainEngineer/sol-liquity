import {
  Connection,
  Keypair,
  PublicKey,
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
  SOLID_TOKEN_MINT_KEY,
  createSPLTokenKeypair,
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
      programId: toPublicKey(programIds().solidStaking),
    }),
  );

  const solidStakingProgramId = programIds().solidStaking;
  const [authority] = await PublicKey.findProgramAddress(
    [solidStakingKey.publicKey.toBuffer()],
    solidStakingProgramId,
  );

  const solidPoolTokenAccount = await createSPLTokenKeypair(
    instructions,
    connection,
    wallet.publicKey,
    authority,
    toPublicKey(SOLID_TOKEN_MINT_KEY),
  );
  signers.push(solidPoolTokenAccount);

  await createSolidStakingInstruction(
    solidStakingKey.publicKey.toBase58(),
    solidPoolTokenAccount.publicKey.toBase58(),
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

  localStorage.setItem(
    'solid-staking-id',
    solidStakingKey.publicKey.toBase58(),
  ); // for demo

  return { txid, slot };
}
