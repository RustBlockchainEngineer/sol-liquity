import {
  PublicKey,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from '@solana/web3.js';
import { programIds } from '../../utils/programIds';
import { serialize } from 'borsh';
import { StringPublicKey, toPublicKey } from '../../utils';
import { SCHEMA } from './state';
import {
  CreateStabilityPoolArgs,
  ProvideToSPArgs,
  RegisterFrontendArgs,
  WithdrawFromSPArgs,
  WithdrawSOLGainToTroveArgs,
} from '..';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import {
  AdjustTroveArgs,
  ApplyPendingRewardsArgs,
  CloseTroveArgs,
  CreateBorrowerOperationsArgs,
  CreateSolidStakingArgs,
  CreateTroveManagerArgs,
  LiquidateArgs,
  LiquidateTrovesArgs,
  OpenTroveArgs,
  RedeemCollateralArgs,
  StakeArgs,
  UnstakeArgs,
} from '.';

export async function createStabilityPoolInstruction(
  stabilityPoolKey: StringPublicKey,
  SOLUSDPoolKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const stabilityPoolProgramId = programIds().stabilityPool;
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(stabilityPoolKey).toBuffer()],
    stabilityPoolProgramId,
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
      pubkey: authority,
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
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(stabilityPoolKey).toBuffer()],
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
      pubkey: authority,
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
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(stabilityPoolKey).toBuffer()],
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
      pubkey: authority,
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
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(stabilityPoolKey).toBuffer()],
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
      pubkey: authority,
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
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(stabilityPoolKey).toBuffer()],
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
      pubkey: authority,
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
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(borrowerOperationsKey).toBuffer()],
    borrowerOperationsProgramId,
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
      pubkey: authority,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(gasPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(collSurplusPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdTokenKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(oracleProgramKey),
      isSigner: false,
      isWritable: true,
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
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(borrowerOperationsProgramId),
      data: data,
    }),
  );
}

export async function openTroveInstruction(
  borrowerOperationsKey: StringPublicKey,
  troveManagerKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  gasPoolKey: StringPublicKey,
  solusdTokenKey: StringPublicKey,
  oracleProgramKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  borrowerKey: StringPublicKey,
  borrowerTroveKey: StringPublicKey,
  ownerKey: StringPublicKey,
  instructions: TransactionInstruction[],

  maxFeePercentage: number,
  solusdAmount: number,
  collIncrease: number,
  SOLAmount: number,
) {
  const borrowerOperationsProgramId = programIds().borrowerOperations;
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(borrowerOperationsKey).toBuffer()],
    borrowerOperationsProgramId,
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new OpenTroveArgs({
        maxFeePercentage,
        solusdAmount,
        collIncrease,
        SOLAmount,
      }),
    ),
  );

  const keys = [
    {
      pubkey: toPublicKey(borrowerOperationsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: authority,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(gasPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdTokenKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: toPublicKey(oracleProgramKey),
      isSigner: false,
      isWritable: true,
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
      pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerTroveKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(ownerKey),
      isSigner: false,
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

export async function adjustTroveInstruction(
  borrowerOperationsKey: StringPublicKey,
  troveManagerKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  ownerKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  gasPoolKey: StringPublicKey,
  solusdTokenKey: StringPublicKey,
  solidStakingKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  borrowerKey: StringPublicKey,
  borrowerTroveKey: StringPublicKey,
  instructions: TransactionInstruction[],

  collWithdrawal: number,
  SOLUSDChange: number,
  isDebtIncrease: number,
  maxFeePercentage: number,
  SOLAmount: number,
) {
  const borrowerOperationsProgramId = programIds().borrowerOperations;
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(borrowerOperationsKey).toBuffer()],
    borrowerOperationsProgramId,
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new AdjustTroveArgs({
        collWithdrawal,
        SOLUSDChange,
        isDebtIncrease,
        maxFeePercentage,
        SOLAmount,
      }),
    ),
  );

  const keys = [
    {
      pubkey: toPublicKey(borrowerOperationsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: authority,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(ownerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(gasPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdTokenKey),
      isSigner: false,
      isWritable: true,
    },

    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
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
      pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerTroveKey),
      isSigner: false,
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

export async function closeTroveInstruction(
  borrowerOperationsKey: StringPublicKey,
  troveManagerKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  gasPoolKey: StringPublicKey,
  solusdTokenKey: StringPublicKey,
  solidStakingKey: StringPublicKey,
  rewardSnapshotsKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  borrowerKey: StringPublicKey,
  borrowerTroveKey: StringPublicKey,
  instructions: TransactionInstruction[],

  amount: number,
) {
  const borrowerOperationsProgramId = programIds().borrowerOperations;
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(borrowerOperationsKey).toBuffer()],
    borrowerOperationsProgramId,
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new CloseTroveArgs({
        amount,
      }),
    ),
  );

  const keys = [
    {
      pubkey: toPublicKey(borrowerOperationsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: authority,
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(gasPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdTokenKey),
      isSigner: false,
      isWritable: true,
    },

    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: toPublicKey(rewardSnapshotsKey),
      isSigner: false,
      isWritable: true,
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
      pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerTroveKey),
      isSigner: false,
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

export async function createTroveManagerInstruction(
  troveManagerKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  gasPoolKey: StringPublicKey,
  collSurplusPoolKey: StringPublicKey,
  borrowerOperationsKey: StringPublicKey,
  oracleProgramKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  solusdTokenKey: StringPublicKey,
  solidStakingKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const troveManagerProgramId = programIds().troveManager;
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(troveManagerKey).toBuffer()],
    troveManagerProgramId,
  );

  const data = Buffer.from(
    serialize(SCHEMA, new CreateTroveManagerArgs({ nonce })),
  );

  const keys = [
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(gasPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(collSurplusPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerOperationsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(oracleProgramKey),
      isSigner: false,
      isWritable: true,
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
      pubkey: toPublicKey(solusdTokenKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(authority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(troveManagerProgramId),
      data: data,
    }),
  );
}

export async function applyPendingRewardsInstruction(
  troveManagerKey: StringPublicKey,
  borrowerKey: StringPublicKey,
  borrowerTroveKey: StringPublicKey,
  rewardSnapshotsKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  callerKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const troveManagerProgramId = programIds().troveManager;
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(troveManagerKey).toBuffer()],
    troveManagerProgramId,
  );

  const data = Buffer.from(serialize(SCHEMA, new ApplyPendingRewardsArgs()));

  const keys = [
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerTroveKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(rewardSnapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(callerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(authority),
      isSigner: false,
      isWritable: true,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(troveManagerProgramId),
      data: data,
    }),
  );
}

export async function liquidateInstruction(
  troveManagerKey: StringPublicKey,
  borrowerKey: StringPublicKey,
  borrowerTroveKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  collSurplusPoolKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  rewardSnapshotsKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  epochToScaleKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const troveManagerProgramId = programIds().troveManager;
  // const [_authority] = await PublicKey.findProgramAddress(
  //   [

  //     toPublicKey(troveManagerKey).toBuffer(),
  //   ],
  //   troveManagerProgramId,
  // );

  const data = Buffer.from(serialize(SCHEMA, new LiquidateArgs()));

  const keys = [
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(borrowerTroveKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(collSurplusPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(rewardSnapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
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
      pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
      isSigner: false,
      isWritable: true,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(troveManagerProgramId),
      data: data,
    }),
  );
}

export async function redeemCollateralInstruction(
  troveManagerKey: StringPublicKey,
  solidStakingKey: StringPublicKey,
  collSurplusPoolKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  ownerKey: StringPublicKey,
  solusdTokenMintKey: StringPublicKey,
  solusdDestKey: StringPublicKey,
  instructions: TransactionInstruction[],

  solusdAmount: number,
  partialRedemptionHintNicr: number,
  maxIterations: number,
  maxFeePercentage: number,
  totalSolDrawn: number,
  totalSolusdToRedeem: number,
) {
  const troveManagerProgramId = programIds().troveManager;
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(troveManagerKey).toBuffer()],
    troveManagerProgramId,
  );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new RedeemCollateralArgs({
        solusdAmount,
        partialRedemptionHintNicr,
        maxIterations,
        maxFeePercentage,
        totalSolDrawn,
        totalSolusdToRedeem,
        nonce,
      }),
    ),
  );

  const keys = [
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(collSurplusPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
      isSigner: false,
      isWritable: true,
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
      pubkey: toPublicKey(ownerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdTokenMintKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solusdDestKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(authority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
    {
      pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
      isSigner: false,
      isWritable: true,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(troveManagerProgramId),
      data: data,
    }),
  );
}

export async function liquidateTrovesInstruction(
  troveManagerKey: StringPublicKey,
  defaultPoolKey: StringPublicKey,
  activePoolKey: StringPublicKey,
  stabilityPoolKey: StringPublicKey,
  communityIssuanceKey: StringPublicKey,
  epochToScaleKey: StringPublicKey,
  collSurplusPoolKey: StringPublicKey,
  pythProductKey: StringPublicKey,
  pythPriceKey: StringPublicKey,
  instructions: TransactionInstruction[],

  totalCollInSequence: number,
  totalDebtInSequence: number,
  totalCollGasCompensation: number,
  totalSolusdGasCompensation: number,
  totalDebtToOffset: number,
  totalCollToSendToSp: number,
  totalDebtToRedistribute: number,
  totalCollToRedistribute: number,
  totalCollSurplus: number,
) {
  const troveManagerProgramId = programIds().troveManager;
  // const [_authority] = await PublicKey.findProgramAddress(
  //   [

  //     toPublicKey(troveManagerKey).toBuffer(),
  //   ],
  //   troveManagerProgramId,
  // );

  const data = Buffer.from(
    serialize(
      SCHEMA,
      new LiquidateTrovesArgs({
        totalCollInSequence,
        totalDebtInSequence,
        totalCollGasCompensation,
        totalSolusdGasCompensation,
        totalDebtToOffset,
        totalCollToSendToSp,
        totalDebtToRedistribute,
        totalCollToRedistribute,
        totalCollSurplus,
      }),
    ),
  );

  const keys = [
    {
      pubkey: toPublicKey(troveManagerKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(defaultPoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(activePoolKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(stabilityPoolKey),
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
      pubkey: toPublicKey(collSurplusPoolKey),
      isSigner: false,
      isWritable: true,
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
      pubkey: toPublicKey(SYSVAR_CLOCK_PUBKEY),
      isSigner: false,
      isWritable: true,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(troveManagerProgramId),
      data: data,
    }),
  );
}

export async function createSolidStakingInstruction(
  solidStakingKey: StringPublicKey,
  solidPoolTokenKey: StringPublicKey,
  instructions: TransactionInstruction[],
) {
  const solidStakingProgramId = programIds().solidStaking;
  const [authority, nonce] = await PublicKey.findProgramAddress(
    [toPublicKey(solidStakingKey).toBuffer()],
    solidStakingProgramId,
  );

  const data = Buffer.from(
    serialize(SCHEMA, new CreateSolidStakingArgs({ nonce })),
  );

  const keys = [
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: true,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(authority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidPoolTokenKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(solidStakingProgramId),
      data: data,
    }),
  );
}

export async function stakeInstruction(
  solidStakingKey: StringPublicKey,
  solidPoolTokenKey: StringPublicKey,
  solidUserTokenKey: StringPublicKey,
  userTransferAuthority: StringPublicKey,
  userDepositKey: StringPublicKey,
  snapshotsKey: StringPublicKey,
  instructions: TransactionInstruction[],

  amount: number,
) {
  const solidStakingProgramId = programIds().solidStaking;
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(solidStakingKey).toBuffer()],
    solidStakingProgramId,
  );

  const data = Buffer.from(serialize(SCHEMA, new StakeArgs({ amount })));

  const keys = [
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(authority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidPoolTokenKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidUserTokenKey),
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
      pubkey: toPublicKey(snapshotsKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(solidStakingProgramId),
      data: data,
    }),
  );
}

export async function unstakeInstruction(
  solidStakingKey: StringPublicKey,
  solidPoolTokenKey: StringPublicKey,
  solidUserTokenKey: StringPublicKey,
  userTransferAuthority: StringPublicKey,
  userDepositKey: StringPublicKey,
  instructions: TransactionInstruction[],

  amount: number,
) {
  const solidStakingProgramId = programIds().solidStaking;
  const [authority] = await PublicKey.findProgramAddress(
    [toPublicKey(solidStakingKey).toBuffer()],
    solidStakingProgramId,
  );

  const data = Buffer.from(serialize(SCHEMA, new UnstakeArgs({ amount })));

  const keys = [
    {
      pubkey: toPublicKey(solidStakingKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(authority),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidPoolTokenKey),
      isSigner: false,
      isWritable: true,
    },
    {
      pubkey: toPublicKey(solidUserTokenKey),
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
      pubkey: toPublicKey(TOKEN_PROGRAM_ID),
      isSigner: false,
      isWritable: false,
    },
  ];
  instructions.push(
    new TransactionInstruction({
      keys,
      programId: toPublicKey(solidStakingProgramId),
      data: data,
    }),
  );
}
