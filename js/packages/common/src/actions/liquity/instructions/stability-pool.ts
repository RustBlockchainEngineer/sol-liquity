import { PublicKey, TransactionInstruction } from '@solana/web3.js';
import { programIds } from '../../../utils/programIds';
import { serialize } from 'borsh';
import { StringPublicKey, toPublicKey } from '../../../utils';
import { SCHEMA, STABILITY_POOL_TAG } from '../state';
import { CreateStabilityPoolArgs } from '..';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';

export async function createStabilityPoolInstruction(
  stabilityPoolKey: StringPublicKey,
  SOLUSDPoolKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const stabilityPoolProgramId = programIds().stability_pool;
  const [aurthority, nonce] = await PublicKey.findProgramAddress(
    [Buffer.from(STABILITY_POOL_TAG), toPublicKey(stabilityPoolKey).toBuffer()],
    toPublicKey(stabilityPoolProgramId),
  );

  const data = Buffer.from(
    serialize(SCHEMA, new CreateStabilityPoolArgs({ nonce })),
  );

  const keys = [
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: aurthority,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(SOLUSDPoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(communityIssuanceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(stabilityPoolProgramId),
      data: data,
    }),
  );
}
