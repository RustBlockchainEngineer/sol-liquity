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
  createSPLTokenKeypair,
  createStabilityPoolInstruction,
  sizeOfState,
  programIds,
  StabilityPool,
  SOLUSD_TOKEN_MINT,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function createStabilityPool(
  connection: Connection,
  wallet: WalletSigner,
): Promise<{
  txid: string;
  slot: number;
}> {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const instructions: TransactionInstruction[] = [];

  const signers: Keypair[] = [];

  const stabilityPoolKey = new Keypair();

  const stabilityPoolSpan = sizeOfState(StabilityPool);
  const stabilityPoolRentExempt =
    await connection.getMinimumBalanceForRentExemption(stabilityPoolSpan);

  instructions.push(
    SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: stabilityPoolKey.publicKey,
      lamports: stabilityPoolRentExempt,
      space: stabilityPoolSpan,
      programId: toPublicKey(programIds().stability_pool),
    }),
  );

  const SOLUSDPoolAccount = await createSPLTokenKeypair(
    instructions,
    connection,
    wallet.publicKey,
    stabilityPoolKey.publicKey,
    toPublicKey(SOLUSD_TOKEN_MINT),
  );

  const communityIssuanceKey = new Keypair();

  await createStabilityPoolInstruction(
    stabilityPoolKey.publicKey.toBase58(),
    SOLUSDPoolAccount.publicKey.toBase58(),
    communityIssuanceKey.publicKey.toBase58(),
    instructions,
  );

  signers.push(stabilityPoolKey);
  signers.push(SOLUSDPoolAccount);

  const { txid, slot } = await sendTransactionWithRetry(
    connection,
    wallet,
    instructions,
    signers,
    'single',
  );

  return { txid, slot };
}
