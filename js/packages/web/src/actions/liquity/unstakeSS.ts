import {
  Connection,
  Keypair,
  PublicKey,
  TransactionInstruction,
} from '@solana/web3.js';
import {
  WalletSigner,
  sendTransactionWithRetry,
  toPublicKey,
  programIds,
  SOLIDStaking,
  unstakeInstruction,
  createSPLTokenKeypair,
  decodeState,
  SOLID_TOKEN_MINT_KEY,
} from '@oyster/common';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';

// This command makes an Lottery
export async function unstakeSS(
  connection: Connection,
  wallet: WalletSigner,
  solidUserToken: string = '',
): Promise<{
  txid: string;
  slot: number;
}> {
  if (!wallet.publicKey) throw new WalletNotConnectedError();

  const instructions: TransactionInstruction[] = [];

  const signers: Keypair[] = [];

  const solidStakingKey = localStorage.getItem('solid-staking-id');
  if (solidStakingKey === null) {
    alert('please create solid-staking before this operation');
    return { txid: '', slot: 0 };
  }

  if (solidUserToken === '' || solidUserToken === null) {
    const solidUserTokenAccount = await createSPLTokenKeypair(
      instructions,
      connection,
      wallet.publicKey,
      wallet.publicKey,
      toPublicKey(SOLID_TOKEN_MINT_KEY),
    );
    signers.push(solidUserTokenAccount);
    solidUserToken = solidUserTokenAccount.publicKey.toBase58();
  }

  const data = (
    await connection.getAccountInfo(
      toPublicKey(solidStakingKey as string),
      'confirmed',
    )
  )?.data as Buffer;
  if (!data) {
    alert("can't load account data");
  }
  const solidStaking: SOLIDStaking = decodeState(data, SOLIDStaking);

  const [userDepositKey] = await PublicKey.findProgramAddress(
    [
      Buffer.from('liquity-solid-staking'),
      wallet.publicKey.toBuffer(),
      toPublicKey(solidStakingKey).toBuffer(),
    ],
    programIds().solidStaking,
  );

  await unstakeInstruction(
    solidStakingKey as string,
    solidStaking.solidPoolTokenPubkey,
    solidUserToken,
    wallet.publicKey.toBase58(),
    userDepositKey.toBase58(),
    instructions,
    100,
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
