import { Connection, Keypair, TransactionInstruction } from '@solana/web3.js';
import {
  WalletSigner,
  sendTransactionWithRetry,
  toPublicKey,
  createSPLTokenKeypair,
  SOLUSD_TOKEN_MINT_KEY,
  openTroveInstruction,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function openTrove(
  connection: Connection,
  wallet: WalletSigner,
): Promise<{
  txid: string;
  slot: number;
}> {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const instructions: TransactionInstruction[] = [];

  const signers: Keypair[] = [];

  const borrowerOperationsKey = localStorage.getItem('borrower-operation-id');
  if (borrowerOperationsKey === null) {
    alert('please create borrower-operation before this operation');
  }

  const solusdUserAccount = await createSPLTokenKeypair(
    instructions,
    connection,
    wallet.publicKey,
    wallet.publicKey,
    toPublicKey(SOLUSD_TOKEN_MINT_KEY),
  );

  await openTroveInstruction(
    borrowerOperationsKey as string,
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    solusdUserAccount.publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    wallet.publicKey.toBase58(),
    instructions,

    10,
    100 * 1000000,
    0,
    0,
  );

  signers.push(solusdUserAccount);

  const { txid, slot } = await sendTransactionWithRetry(
    connection,
    wallet,
    instructions,
    signers,
    'confirmed',
  );

  return { txid, slot };
}
