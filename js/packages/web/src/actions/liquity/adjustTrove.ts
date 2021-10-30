import { Connection, Keypair, TransactionInstruction } from '@solana/web3.js';
import {
  WalletSigner,
  sendTransactionWithRetry,
  toPublicKey,
  createSPLTokenKeypair,
  SOLUSD_TOKEN_MINT_KEY,
  adjustTroveInstruction,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function adjustTrove(
  connection: Connection,
  wallet: WalletSigner,
  solusdUserToken: string = '',
  isDebtIncrease: number = 1,
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

  await adjustTroveInstruction(
    borrowerOperationsKey as string,
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    wallet.publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    solusdUserAccountKey,
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    wallet.publicKey.toBase58(),
    new Keypair().publicKey.toBase58(),
    instructions,

    0,
    10 * 1000000,
    isDebtIncrease,
    0,
    0,
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
