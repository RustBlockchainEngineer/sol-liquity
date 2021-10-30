import { Connection, Keypair, TransactionInstruction } from '@solana/web3.js';
import {
  WalletSigner,
  sendTransactionWithRetry,
  provideToSPInstruction,
  SOLUSD_TOKEN_MINT_KEY,
  toPublicKey,
  createSPLTokenKeypair,
  decodeState,
  StabilityPool,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function provideToSP(
  connection: Connection,
  wallet: WalletSigner,
  solusdUserToken: string = '',
): Promise<{
  txid: string;
  slot: number;
}> {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const instructions: TransactionInstruction[] = [];
  const signers: Keypair[] = [];

  const stabilityPoolKey = localStorage.getItem('stability-pool-id');
  if (stabilityPoolKey === null) {
    alert('please create stability-pool before this operation');
  }

  const solusdUserAccountKey =
    solusdUserToken === '' || solusdUserToken === null
      ? (
          await createSPLTokenKeypair(
            instructions,
            connection,
            wallet.publicKey,
            wallet.publicKey,
            toPublicKey(SOLUSD_TOKEN_MINT_KEY),
          )
        ).publicKey.toBase58()
      : solusdUserToken;

  const data = (
    await connection.getAccountInfo(
      toPublicKey(stabilityPoolKey as string),
      'confirmed',
    )
  )?.data as Buffer;
  if (!data) {
    alert("can't load account data");
  }
  const stabilityPool = decodeState(data, StabilityPool);

  await provideToSPInstruction(
    stabilityPoolKey as string,
    stabilityPool.SOLUSDPoolTokenPubkey,
    solusdUserAccountKey,
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    wallet.publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    stabilityPool.communityIssuancePubkey,
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    instructions,
    10,
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
