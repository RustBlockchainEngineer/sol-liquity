import {
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from '@solana/web3.js';
import { programIds } from '../../utils/programIds';
import { serialize } from 'borsh';
import { StringPublicKey, toPublicKey } from '../../utils';
import { SCHEMA, STABILITY_POOL_TAG } from './state';
import {
  CreateStabilityPoolArgs,
  ProvideToSPArgs,
  RegisterFrontendArgs,
  WithdrawFromSPArgs,
  WithdrawSOLGainToTroveArgs,
} from '..';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { BORROWER_OPERATIONS_TAG, CreateBorrowerOperationsArgs } from '.';

export async function createStabilityPoolInstruction(
  stabilityPoolKey: StringPublicKey,
  SOLUSDPoolKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const stabilityPoolProgramId = programIds().stabilityPool;
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

export async function provideToSPInstruction(
  stabilityPoolKey: StringPublicKey,
  SOLUSDPoolKey: StringPublicKey,
  SOLUSDUserKey: StringPublicKey,
  WSOLPoolGainKey: StringPublicKey,
  WSOLUserKey: StringPublicKey,
  userTransferAuthority: StringPublicKey,
  userDepositKey: StringPublicKey,
  frontendKey: StringPublicKey,
  depositorFrontendKey: StringPublicKey,
  snapshotsKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  epochToScaleKey: StringPublicKey,
  epochToPlusScaleKey: StringPublicKey,
  sourceKey: StringPublicKey,
  frontendDestKey: StringPublicKey,
  depositorDestKey: StringPublicKey,
  instructions: TransactionInstruction[],

  amount: number,
) {
  const stabilityPoolProgramId = programIds().stabilityPool;
  const [aurthority, nonce] = await PublicKey.findProgramAddress(
    [Buffer.from(STABILITY_POOL_TAG), toPublicKey(stabilityPoolKey).toBuffer()],
    toPublicKey(stabilityPoolProgramId),
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new ProvideToSPArgs({
        amount,
        communityIssuancePool: communityIssuanceKey,
        nonce,
      }),
    ),
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
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(SOLUSDUserKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(WSOLPoolGainKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(WSOLUserKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(userTransferAuthority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(userDepositKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(frontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(depositorFrontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(snapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(communityIssuanceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(epochToScaleKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(epochToPlusScaleKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(sourceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(frontendDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(depositorDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: SYSVAR_CLOCK_PUBKEY,
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

export async function withdrawFromSPInstruction(
  stabilityPoolKey: StringPublicKey,
  SOLUSDPoolKey: StringPublicKey,
  SOLUSDUserKey: StringPublicKey,
  WSOLPoolGainKey: StringPublicKey,
  WSOLUserKey: StringPublicKey,
  troveManagerKey: StringPublicKey,
  rewardSnapshotsKey: StringPublicKey,
  lowestTroveKey: StringPublicKey,
  frontendKey: StringPublicKey,
  depositorFrontendKey: StringPublicKey,
  snapshotsKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  epochToScaleKey: StringPublicKey,
  epochToPlusScaleKey: StringPublicKey,
  userTransferAuthority: StringPublicKey,
  userDepositKey: StringPublicKey,
  sourceKey: StringPublicKey,
  frontendDestKey: StringPublicKey,
  depositorDestKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  instructions: TransactionInstruction[],

  amount: number,
) {
  const stabilityPoolProgramId = programIds().stabilityPool;
  const [aurthority, nonce] = await PublicKey.findProgramAddress(
    [Buffer.from(STABILITY_POOL_TAG), toPublicKey(stabilityPoolKey).toBuffer()],
    toPublicKey(stabilityPoolProgramId),
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new WithdrawFromSPArgs({
        amount,
        communityIssuancePool: communityIssuanceKey,
        nonce,
      }),
    ),
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
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(SOLUSDUserKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(WSOLPoolGainKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(WSOLUserKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(rewardSnapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(lowestTroveKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(frontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(depositorFrontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(snapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(communityIssuanceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(epochToScaleKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(epochToPlusScaleKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(userTransferAuthority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(userDepositKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(sourceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(frontendDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(depositorDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: toPublicKey(pythProductKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(pythPriceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: SYSVAR_CLOCK_PUBKEY,
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

export async function withdrawSOLGainToTroveInstruction(
  stabilityPoolKey: StringPublicKey,
  frontendKey: StringPublicKey,
  depositorFrontendKey: StringPublicKey,
  snapshotsKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  epochToScaleKey: StringPublicKey,
  epochToPlusScaleKey: StringPublicKey,
  userDepositKey: StringPublicKey,
  sourceKey: StringPublicKey,
  frontendDestKey: StringPublicKey,
  depositorDestKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const stabilityPoolProgramId = programIds().stabilityPool;
  const [aurthority, nonce] = await PublicKey.findProgramAddress(
    [Buffer.from(STABILITY_POOL_TAG), toPublicKey(stabilityPoolKey).toBuffer()],
    toPublicKey(stabilityPoolProgramId),
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new WithdrawSOLGainToTroveArgs({
        communityIssuancePool: communityIssuanceKey,
        nonce,
      }),
    ),
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
      pubkey: toPublicKey(frontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(depositorFrontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(snapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(communityIssuanceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(epochToScaleKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(epochToPlusScaleKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(userDepositKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(sourceKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(frontendDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(depositorDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: TOKEN_PROGRAM_ID,
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: SYSVAR_CLOCK_PUBKEY,
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

export async function registerFrontendInstruction(
  stabilityPoolKey: StringPublicKey,
  frontendKey: StringPublicKey,
  userDepositKey: StringPublicKey,
  instructions: TransactionInstruction[],

  kickbackRate: number,
) {
  const stabilityPoolProgramId = programIds().stabilityPool;
  const [aurthority] = await PublicKey.findProgramAddress(
    [Buffer.from(STABILITY_POOL_TAG), toPublicKey(stabilityPoolKey).toBuffer()],
    toPublicKey(stabilityPoolProgramId),
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new RegisterFrontendArgs({
        kickbackRate,
      }),
    ),
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
      pubkey: toPublicKey(frontendKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(userDepositKey),
      isSigner: false,
      isWritable: true,
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

export async function createBorrowerOperationsInstruction(
  borrowerOperationsKey: StringPublicKey,
  troveManagerKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  gasPoolKey: StringPublicKey,
  collSurplusPoolKey: StringPublicKey,
  solusdTokenKey: StringPublicKey,
  solidStakingKey: StringPublicKey,
  oracleProgramKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const borrowerOperationsProgramId = programIds().borrowerOperations;
  const [aurthority, nonce] = await PublicKey.findProgramAddress(
    [
      Buffer.from(BORROWER_OPERATIONS_TAG),
      toPublicKey(borrowerOperationsKey).toBuffer(),
    ],
    toPublicKey(borrowerOperationsKey),
  );

  const data = Buffer.from(
    serialize(SCHEMA, new CreateBorrowerOperationsArgs({ nonce })),
  );

  const keys = [
    {
      pubkey: toPublicKey(borrowerOperationsKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: aurthority,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(gasPoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(collSurplusPoolKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdTokenKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(oracleProgramKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(pythProductKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(pythPriceKey),
      isSigner: true,
      isWritable: true,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(borrowerOperationsProgramId),
      data: data,
    }),
  );
}
