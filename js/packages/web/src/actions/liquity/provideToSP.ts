import { Connection, Keypair, TransactionInstruction } from '@solana/web3.js';
import {
  WalletSigner,
  sendTransactionWithRetry,
  StabilityPool,
  provideToSPInstruction,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function provideToSP(
  connection: Connection,
  wallet: WalletSigner,
  stabilityPool: StabilityPool,
  stabilityPoolId: string,
  solusdUserTokenId: string,
  userDepositId: string,
  frontendId: string,
  depositorFrontendId: string,
  snapshotsId: string,
  epochToScaleId: string,
  epochToPlusScaleId: string,
  sourceId: string,
  frontednDestId: string,
  depositorDestId: string,
  amount: number,
): Promise<{
  txid: string;
  slot: number;
}> {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const instructions: TransactionInstruction[] = [];
  const signers: Keypair[] = [];
  const wsolPoolGainKey = Keypair.generate().publicKey.toBase58();
  const wsolUserGainKey = Keypair.generate().publicKey.toBase58();

  await provideToSPInstruction(
    stabilityPoolId,
    stabilityPool.SOLUSDPoolTokenPubkey,
    solusdUserTokenId,
    wsolPoolGainKey,
    wsolUserGainKey,
    wallet.publicKey.toBase58(),
    userDepositId,
    frontendId,
    depositorFrontendId,
    snapshotsId,
    stabilityPool.communityIssuancePubkey,
    epochToScaleId,
    epochToPlusScaleId,
    sourceId,
    frontednDestId,
    depositorDestId,
    instructions,
    amount,
  );
  const { txid, slot } = await sendTransactionWithRetry(
    connection,
    wallet,
    instructions,
    signers,
    'confirmed',
  );

  return { txid, slot };
}
