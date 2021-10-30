import { deserializeUnchecked } from 'borsh';
import BN from 'bn.js';
import { StringPublicKey } from '../../utils';

export const STABILITY_POOL_TAG = 'liquity_stability_pool';
export const TROVE_MANAGER_TAG = 'liquity_trove_manager';
export const BORROWER_OPERATIONS_TAG = 'liquity_borrower_operations';

export class StabilityPool {
  nonce: number;
  tokenProgramPubkey: StringPublicKey;
  SOLUSDPoolTokenPubkey: StringPublicKey;
  borrowerOperationsPubkey: StringPublicKey;
  troveManagerPubkey: StringPublicKey;
  communityIssuancePubkey: StringPublicKey;
  totalSOLUSDDeposits: BN;
  lastSOLIDError: BN;
  lastSOLErrorOffset: BN;
  lastSOLUSDLossErrorOffset: BN;
  p: BN;
  currentScale: BN;
  currentEpoch: BN;
  sol: BN;
  oracleProgramId: StringPublicKey;
  quoteCurrency: StringPublicKey;

  constructor(obj: {
    nonce: number;
    tokenProgramPubkey: StringPublicKey;
    SOLUSDPoolTokenPubkey: StringPublicKey;
    borrowerOperationsPubkey: StringPublicKey;
    troveManagerPubkey: StringPublicKey;
    communityIssuancePubkey: StringPublicKey;
    totalSOLUSDDeposits: BN;
    lastSOLIDError: BN;
    lastSOLErrorOffset: BN;
    lastSOLUSDLossErrorOffset: BN;
    p: BN;
    currentScale: BN;
    currentEpoch: BN;
    sol: BN;
    oracleProgramId: StringPublicKey;
    quoteCurrency: StringPublicKey;
  }) {
    this.nonce = obj.nonce;
    this.tokenProgramPubkey = obj.tokenProgramPubkey;
    this.SOLUSDPoolTokenPubkey = obj.SOLUSDPoolTokenPubkey;
    this.borrowerOperationsPubkey = obj.borrowerOperationsPubkey;
    this.troveManagerPubkey = obj.troveManagerPubkey;
    this.communityIssuancePubkey = obj.communityIssuancePubkey;
    this.totalSOLUSDDeposits = obj.totalSOLUSDDeposits;
    this.lastSOLIDError = obj.lastSOLIDError;
    this.lastSOLErrorOffset = obj.lastSOLErrorOffset;
    this.lastSOLUSDLossErrorOffset = obj.lastSOLUSDLossErrorOffset;
    this.p = obj.p;
    this.currentScale = obj.currentScale;
    this.currentEpoch = obj.currentEpoch;
    this.sol = obj.sol;
    this.oracleProgramId = obj.oracleProgramId;
    this.quoteCurrency = obj.quoteCurrency;
  }
}

export class BorrowerOperations {
  nonce: number;
  tokenProgramPubkey: StringPublicKey;
  troveManagerId: StringPublicKey;
  activePoolId: StringPublicKey;
  defaultPoolId: StringPublicKey;
  stabilityPoolId: StringPublicKey;
  gasPoolId: StringPublicKey;
  collSurplusPoolId: StringPublicKey;
  solusdTokenId: StringPublicKey;
  solidStakingId: StringPublicKey;

  oracleProgramId: StringPublicKey;
  pythProductId: StringPublicKey;
  pythPriceId: StringPublicKey;
  quoteCurrency: StringPublicKey;

  constructor(obj: {
    nonce: number;
    tokenProgramPubkey: StringPublicKey;
    troveManagerId: StringPublicKey;
    activePoolId: StringPublicKey;
    defaultPoolId: StringPublicKey;
    stabilityPoolId: StringPublicKey;
    gasPoolId: StringPublicKey;
    collSurplusPoolId: StringPublicKey;
    solusdTokenId: StringPublicKey;
    solidStakingId: StringPublicKey;

    oracleProgramId: StringPublicKey;
    pythProductId: StringPublicKey;
    pythPriceId: StringPublicKey;
    quoteCurrency: StringPublicKey;
  }) {
    this.nonce = obj.nonce;
    this.tokenProgramPubkey = obj.tokenProgramPubkey;
    this.troveManagerId = obj.troveManagerId;
    this.activePoolId = obj.activePoolId;
    this.defaultPoolId = obj.defaultPoolId;
    this.stabilityPoolId = obj.stabilityPoolId;
    this.gasPoolId = obj.gasPoolId;
    this.collSurplusPoolId = obj.collSurplusPoolId;
    this.solusdTokenId = obj.solusdTokenId;
    this.solidStakingId = obj.solidStakingId;
    this.oracleProgramId = obj.oracleProgramId;
    this.pythProductId = obj.pythProductId;
    this.pythPriceId = obj.pythPriceId;
    this.quoteCurrency = obj.quoteCurrency;
  }
}

export class TroveManager {
  nonce: number;
  borrowerOperationsId: StringPublicKey;
  stabilityPoolId: StringPublicKey;
  gasPoolId: StringPublicKey;
  collSurplusPoolId: StringPublicKey;
  SOLUSDTokenPubkey: StringPublicKey;
  SOLIDTokenPubkey: StringPublicKey;
  SOLIDStakingPubkey: StringPublicKey;
  tokenProgramId: StringPublicKey;
  defaultPoolId: StringPublicKey;
  activePoolId: StringPublicKey;
  oracleProgramId: StringPublicKey;
  pythProductId: StringPublicKey;
  pythPriceId: StringPublicKey;
  quoteCurrency: StringPublicKey;
  baseRate: BN;
  lasstFeeOperationTime: BN;
  totalStakes: BN;
  totalStakesSnapshot: BN;
  totalCollateralSnapshot: BN;
  lSol: BN;
  lSOLUSDDebt: BN;
  lastSOLErrorRedistribution: BN;
  lastSOLUSDDebtErrorRedistribution: BN;

  constructor(obj: {
    nonce: number;
    borrowerOperationsId: StringPublicKey;
    stabilityPoolId: StringPublicKey;
    gasPoolId: StringPublicKey;
    collSurplusPoolId: StringPublicKey;
    SOLUSDTokenPubkey: StringPublicKey;
    SOLIDTokenPubkey: StringPublicKey;
    SOLIDStakingPubkey: StringPublicKey;
    tokenProgramId: StringPublicKey;
    defaultPoolId: StringPublicKey;
    activePoolId: StringPublicKey;
    oracleProgramId: StringPublicKey;
    pythProductId: StringPublicKey;
    pythPriceId: StringPublicKey;
    quoteCurrency: StringPublicKey;
    baseRate: BN;
    lasstFeeOperationTime: BN;
    totalStakes: BN;
    totalStakesSnapshot: BN;
    totalCollateralSnapshot: BN;
    lSol: BN;
    lSOLUSDDebt: BN;
    lastSOLErrorRedistribution: BN;
    lastSOLUSDDebtErrorRedistribution: BN;
  }) {
    this.nonce = obj.nonce;
    this.borrowerOperationsId = obj.borrowerOperationsId;
    this.stabilityPoolId = obj.stabilityPoolId;
    this.gasPoolId = obj.gasPoolId;
    this.collSurplusPoolId = obj.collSurplusPoolId;
    this.SOLUSDTokenPubkey = obj.SOLUSDTokenPubkey;
    this.SOLIDTokenPubkey = obj.SOLIDTokenPubkey;
    this.SOLIDStakingPubkey = obj.SOLIDStakingPubkey;
    this.tokenProgramId = obj.tokenProgramId;
    this.defaultPoolId = obj.defaultPoolId;
    this.activePoolId = obj.activePoolId;
    this.oracleProgramId = obj.oracleProgramId;
    this.pythProductId = obj.pythProductId;
    this.pythPriceId = obj.pythPriceId;
    this.quoteCurrency = obj.quoteCurrency;
    this.baseRate = obj.baseRate;
    this.lasstFeeOperationTime = obj.lasstFeeOperationTime;
    this.totalStakes = obj.totalStakes;
    this.totalStakesSnapshot = obj.totalStakesSnapshot;
    this.totalCollateralSnapshot = obj.totalCollateralSnapshot;
    this.lSol = obj.lSol;
    this.lSOLUSDDebt = obj.lSOLUSDDebt;
    this.lastSOLErrorRedistribution = obj.lastSOLErrorRedistribution;
    this.lastSOLUSDDebtErrorRedistribution =
      obj.lastSOLUSDDebtErrorRedistribution;
  }
}

export class SOLIDStaking {
  nonce: number;
  tokenProgramPubkey: StringPublicKey;
  solidPoolTokenPubkey: StringPublicKey;
  troveManagerId: StringPublicKey;
  borrowerOperationsId: StringPublicKey;
  activePoolId: StringPublicKey;
  totalStakedAmount: BN;
  fSol: BN;
  fSolusd: BN;

  constructor(obj: {
    nonce: number;
    tokenProgramPubkey: StringPublicKey;
    solidPoolTokenPubkey: StringPublicKey;
    troveManagerId: StringPublicKey;
    borrowerOperationsId: StringPublicKey;
    activePoolId: StringPublicKey;
    totalStakedAmount: BN;
    fSol: BN;
    fSolusd: BN;
  }) {
    this.nonce = obj.nonce;
    this.tokenProgramPubkey = obj.tokenProgramPubkey;
    this.solidPoolTokenPubkey = obj.solidPoolTokenPubkey;
    this.troveManagerId = obj.troveManagerId;
    this.borrowerOperationsId = obj.borrowerOperationsId;
    this.activePoolId = obj.activePoolId;
    this.totalStakedAmount = obj.totalStakedAmount;
    this.fSol = obj.fSol;
    this.fSolusd = obj.fSolusd;
  }
}

export class CreateStabilityPoolArgs {
  instruction: number = 0;
  nonce: number;
  constructor(args: { nonce: number }) {
    this.nonce = args.nonce;
  }
}

export class ProvideToSPArgs {
  instruction: number = 1;
  amount: BN;
  communityIssuancePool: StringPublicKey;
  nonce: number;
  constructor(args: {
    amount: number;
    communityIssuancePool: StringPublicKey;
    nonce: number;
  }) {
    this.amount = new BN(args.amount);
    this.communityIssuancePool = args.communityIssuancePool;
    this.nonce = args.nonce;
  }
}

export class WithdrawFromSPArgs {
  instruction: number = 2;
  amount: BN;
  communityIssuancePool: StringPublicKey;
  nonce: number;
  constructor(args: {
    amount: number;
    communityIssuancePool: StringPublicKey;
    nonce: number;
  }) {
    this.amount = new BN(args.amount);
    this.communityIssuancePool = args.communityIssuancePool;
    this.nonce = args.nonce;
  }
}

export class WithdrawSOLGainToTroveArgs {
  instruction: number = 3;
  communityIssuancePool: StringPublicKey;
  nonce: number;
  constructor(args: { communityIssuancePool: StringPublicKey; nonce: number }) {
    this.communityIssuancePool = args.communityIssuancePool;
    this.nonce = args.nonce;
  }
}

export class RegisterFrontendArgs {
  instruction: number = 4;
  kickbackRate: BN;
  constructor(args: { kickbackRate: number }) {
    this.kickbackRate = new BN(args.kickbackRate);
  }
}

export class CreateBorrowerOperationsArgs {
  instruction: number = 0;
  nonce: number;
  constructor(args: { nonce: number }) {
    this.nonce = args.nonce;
  }
}

export class OpenTroveArgs {
  instruction: number = 1;
  maxFeePercentage: BN;
  solusdAmount: BN;
  collIncrease: BN;
  SOLAmount: BN;
  constructor(args: {
    maxFeePercentage: number;
    solusdAmount: number;
    collIncrease: number;
    SOLAmount: number;
  }) {
    this.maxFeePercentage = new BN(args.maxFeePercentage);
    this.solusdAmount = new BN(args.solusdAmount);
    this.collIncrease = new BN(args.collIncrease);
    this.SOLAmount = new BN(args.SOLAmount);
  }
}

export class AdjustTroveArgs {
  instruction: number = 2;
  collWithdrawal: BN;
  SOLUSDChange: BN;
  isDebtIncrease: number;
  maxFeePercentage: BN;
  SOLAmount: BN;
  constructor(args: {
    collWithdrawal: number;
    SOLUSDChange: number;
    isDebtIncrease: number;
    maxFeePercentage: number;
    SOLAmount: number;
  }) {
    this.collWithdrawal = new BN(args.collWithdrawal);
    this.SOLUSDChange = new BN(args.SOLUSDChange);
    this.isDebtIncrease = args.isDebtIncrease;
    this.maxFeePercentage = new BN(args.maxFeePercentage);
    this.SOLAmount = new BN(args.SOLAmount);
  }
}

export class CloseTroveArgs {
  instruction: number = 3;
  amount: BN;
  constructor(args: { amount: number }) {
    this.amount = new BN(args.amount);
  }
}

export class CreateTroveManagerArgs {
  instruction: number = 0;
  nonce: number;
  constructor(args: { nonce: number }) {
    this.nonce = args.nonce;
  }
}
export class ApplyPendingRewardsArgs {
  instruction: number = 1;
  constructor() {}
}

export class LiquidateArgs {
  instruction: number = 2;
  constructor() {}
}

export class RedeemCollateralArgs {
  instruction: number = 3;
  solusdAmount: BN;
  partialRedemptionHintNicr: BN;
  maxIterations: BN;
  maxFeePercentage: BN;
  totalSolDrawn: BN;
  totalSolusdToRedeem: BN;
  nonce: number;
  constructor(args: {
    solusdAmount: number;
    partialRedemptionHintNicr: number;
    maxIterations: number;
    maxFeePercentage: number;
    totalSolDrawn: number;
    totalSolusdToRedeem: number;
    nonce: number;
  }) {
    this.solusdAmount = new BN(args.solusdAmount);
    this.partialRedemptionHintNicr = new BN(args.partialRedemptionHintNicr);
    this.maxIterations = new BN(args.maxIterations);
    this.maxFeePercentage = new BN(args.maxFeePercentage);
    this.totalSolDrawn = new BN(args.totalSolDrawn);
    this.totalSolusdToRedeem = new BN(args.totalSolusdToRedeem);
    this.nonce = args.nonce;
  }
}

export class LiquidateTrovesArgs {
  instruction: number = 4;
  totalCollInSequence: BN;
  totalDebtInSequence: BN;
  totalCollGasCompensation: BN;
  totalSolusdGasCompensation: BN;
  totalDebtToOffset: BN;
  totalCollToSendToSp: BN;
  totalDebtToRedistribute: BN;
  totalCollToRedistribute: BN;
  totalCollSurplus: BN;
  constructor(args: {
    totalCollInSequence: number;
    totalDebtInSequence: number;
    totalCollGasCompensation: number;
    totalSolusdGasCompensation: number;
    totalDebtToOffset: number;
    totalCollToSendToSp: number;
    totalDebtToRedistribute: number;
    totalCollToRedistribute: number;
    totalCollSurplus: number;
  }) {
    this.totalCollInSequence = new BN(args.totalCollInSequence);
    this.totalDebtInSequence = new BN(args.totalDebtInSequence);
    this.totalCollGasCompensation = new BN(args.totalCollGasCompensation);
    this.totalSolusdGasCompensation = new BN(args.totalSolusdGasCompensation);
    this.totalDebtToOffset = new BN(args.totalDebtToOffset);
    this.totalCollToSendToSp = new BN(args.totalCollToSendToSp);
    this.totalDebtToRedistribute = new BN(args.totalDebtToRedistribute);
    this.totalCollToRedistribute = new BN(args.totalCollToRedistribute);
    this.totalCollSurplus = new BN(args.totalCollSurplus);
  }
}

export class CreateSolidStakingArgs {
  instruction: number = 0;
  nonce: number;
  constructor(args: { nonce: number }) {
    this.nonce = args.nonce;
  }
}

export class StakeArgs {
  instruction: number = 1;
  amount: BN;
  constructor(args: { amount: number }) {
    this.amount = new BN(args.amount);
  }
}

export class UnstakeArgs {
  instruction: number = 2;
  amount: BN;
  constructor(args: { amount: number }) {
    this.amount = new BN(args.amount);
  }
}

export const decodeState = (buffer: Buffer, classType: any) => {
  return deserializeUnchecked(SCHEMA, classType, buffer);
};
export const sizeOfState = (classType: any) => {
  const fields = SCHEMA.get(classType).fields as [];
  let size = 0;
  fields.forEach(field => {
    switch (field[1]) {
      case 'u8':
        size += 1;
        break;
      case 'u32':
        size += 4;
        break;
      case 'u64':
        size += 8;
        break;
      case 'u128':
        size += 16;
        break;
      case 'u256':
        size += 32;
        break;
      case 'i8':
        size += 1;
        break;
      case 'i32':
        size += 4;
        break;
      case 'i64':
        size += 8;
        break;
      case 'i128':
        size += 16;
        break;
      case 'i256':
        size += 32;
        break;
      case 'pubkeyAsString':
        size += 32;
        break;
      default:
        console.log('error in sizeOfState function');
        break;
    }
  });
  return size;
};

export const SCHEMA = new Map<any, any>([
  [
    StabilityPool,
    {
      kind: 'struct',
      fields: [
        ['nonce', 'u8'],
        ['tokenProgramPubkey', 'pubkeyAsString'],
        ['SOLUSDPoolTokenPubkey', 'pubkeyAsString'],
        ['borrowerOperationsPubkey', 'pubkeyAsString'],
        ['troveManagerPubkey', 'pubkeyAsString'],
        ['communityIssuancePubkey', 'pubkeyAsString'],
        ['totalSOLUSDDeposits', 'u128'],
        ['lastSOLIDError', 'u128'],
        ['lastSOLErrorOffset', 'u128'],
        ['lastSOLUSDLossErrorOffset', 'u128'],
        ['p', 'u128'],
        ['currentScale', 'u128'],
        ['currentEpoch', 'u128'],
        ['sol', 'u128'],
        ['oracleProgramId', 'pubkeyAsString'],
        ['quoteCurrency', 'pubkeyAsString'],
      ],
    },
  ],
  [
    BorrowerOperations,
    {
      kind: 'struct',
      fields: [
        ['nonce', 'u8'],
        ['tokenProgramPubkey', 'pubkeyAsString'],
        ['troveManagerId', 'pubkeyAsString'],
        ['activePoolId', 'pubkeyAsString'],
        ['defaultPoolId', 'pubkeyAsString'],
        ['stabilityPoolId', 'pubkeyAsString'],
        ['gasPoolId', 'pubkeyAsString'],
        ['collSurplusPoolId', 'pubkeyAsString'],
        ['solusdTokenId', 'pubkeyAsString'],
        ['solidStakingId', 'pubkeyAsString'],
        ['oracleProgramId', 'pubkeyAsString'],
        ['pythProductId', 'pubkeyAsString'],
        ['pythPriceId', 'pubkeyAsString'],
        ['quoteCurrency', 'pubkeyAsString'],
      ],
    },
  ],
  [
    TroveManager,
    {
      kind: 'struct',
      fields: [
        ['nonce', 'u8'],
        ['borrowerOperationsId', 'pubkeyAsString'],
        ['stabilityPoolId', 'pubkeyAsString'],
        ['gasPoolId', 'pubkeyAsString'],
        ['collSurplusPoolId', 'pubkeyAsString'],
        ['SOLUSDTokenPubkey', 'pubkeyAsString'],
        ['SOLIDTokenPubkey', 'pubkeyAsString'],
        ['SOLIDStakingPubkey', 'pubkeyAsString'],
        ['tokenProgramId', 'pubkeyAsString'],
        ['defaultPoolId', 'pubkeyAsString'],
        ['activePoolId', 'pubkeyAsString'],
        ['oracleProgramId', 'pubkeyAsString'],
        ['pythProductId', 'pubkeyAsString'],
        ['pythPriceId', 'pubkeyAsString'],
        ['quoteCurrency', 'pubkeyAsString'],
        ['baseRate', 'u128'],
        ['lasstFeeOperationTime', 'u128'],
        ['totalStakes', 'u128'],
        ['totalStakesSnapshot', 'u128'],
        ['totalCollateralSnapshot', 'u128'],
        ['lSol', 'u128'],
        ['lSOLUSDDebt', 'u128'],
        ['lastSOLErrorRedistribution', 'u128'],
        ['lastSOLUSDDebtErrorRedistribution', 'u128'],
      ],
    },
  ],
  [
    SOLIDStaking,
    {
      kind: 'struct',
      fields: [
        ['nonce', 'u8'],
        ['tokenProgramPubkey', 'pubkeyAsString'],
        ['solidPoolTokenPubkey', 'pubkeyAsString'],
        ['troveManagerId', 'pubkeyAsString'],
        ['borrowerOperationsId', 'pubkeyAsString'],
        ['activePoolId', 'pubkeyAsString'],
        ['totalStakedAmount', 'u128'],
        ['fSol', 'u128'],
        ['fSolusd', 'u128'],
      ],
    },
  ],
  [
    CreateStabilityPoolArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    ProvideToSPArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
        ['communityIssuancePool', 'pubkeyAsString'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    WithdrawFromSPArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
        ['communityIssuancePool', 'pubkeyAsString'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    WithdrawSOLGainToTroveArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['communityIssuancePool', 'pubkeyAsString'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    RegisterFrontendArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['kickbackRate', 'u64'],
      ],
    },
  ],
  [
    CreateBorrowerOperationsArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    OpenTroveArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['maxFeePercentage', 'u64'],
        ['solusdAmount', 'u64'],
        ['collIncrease', 'u64'],
        ['SOLAmount', 'u64'],
      ],
    },
  ],
  [
    AdjustTroveArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['collWithdrawal', 'u64'],
        ['SOLUSDChange', 'u64'],
        ['isDebtIncrease', 'u8'],
        ['maxFeePercentage', 'u64'],
        ['SOLAmount', 'u64'],
      ],
    },
  ],
  [
    CloseTroveArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u64'],
      ],
    },
  ],
  [
    CreateTroveManagerArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    ApplyPendingRewardsArgs,
    {
      kind: 'struct',
      fields: [['instruction', 'u8']],
    },
  ],
  [
    LiquidateArgs,
    {
      kind: 'struct',
      fields: [['instruction', 'u8']],
    },
  ],
  [
    RedeemCollateralArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['solusdAmount', 'u128'],
        ['partialRedemptionHintNicr', 'u128'],
        ['maxIterations', 'u128'],
        ['maxFeePercentage', 'u128'],
        ['totalSolDrawn', 'u128'],
        ['totalSolusdToRedeem', 'u128'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    LiquidateTrovesArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['totalCollInSequence', 'u128'],
        ['totalDebtInSequence', 'u128'],
        ['totalCollGasCompensation', 'u128'],
        ['totalSolusdGasCompensation', 'u128'],
        ['totalDebtToOffset', 'u128'],
        ['totalCollToSendToSp', 'u128'],
        ['totalDebtToRedistribute', 'u128'],
        ['totalCollToRedistribute', 'u128'],
        ['totalCollSurplus', 'u128'],
      ],
    },
  ],
  [
    CreateSolidStakingArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['nonce', 'u8'],
      ],
    },
  ],
  [
    StakeArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u128'],
      ],
    },
  ],
  [
    UnstakeArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['amount', 'u128'],
      ],
    },
  ],
]);

// export class Frontend {
//   poolIdPubkey: StringPublicKey;
//   ownerPubkey: StringPublicKey;
//   kickbackRate: number;
//   registered: boolean;
//   frontendStake:number;

//   constructor(obj: {
//     poolIdPubkey: StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     kickbackRate: BN;
//     registered: number;
//     frontendStake: BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.kickbackRate = obj.kickbackRate.toNumber();
//     this.registered = obj.registered >= 0;
//     this.frontendStake = obj.frontendStake.toNumber();
//   }
// }

// export class Deposit {
//   poolIdPubkey: StringPublicKey;
//   ownerPubkey: StringPublicKey;
//   initialValue: number;
//   frontendTag: StringPublicKey;

//   constructor(obj: {
//     poolIdPubkey: StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     initialValue: BN;
//     frontendTag: StringPublicKey;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.initialValue = obj.initialValue.toNumber();
//     this.frontendTag = obj.frontendTag);
//   }
// }
// export class Snapshots {
//   poolIdPubkey: StringPublicKey;
//   ownerPubkey: StringPublicKey;
//   s: number;
//   p: number;
//   g: number;
//   scale: number;
//   epoch: number;

//   constructor(obj: {
//     poolIdPubkey: StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     s: BN;
//     p: BN;
//     g: BN;
//     scale: BN;
//     epoch: BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.s = obj.s.toNumber();
//     this.p = obj.p.toNumber();
//     this.g = obj.g.toNumber();
//     this.scale = obj.scale.toNumber();
//     this.epoch = obj.epoch.toNumber();
//   }
// }

// export enum TroveStatus {
//   NonExistent = 0,
//   Active,
//   ClosedByOwner,
//   ClosedByLiquidation,
//   ClosedByRedemption,

// }
// export function getTroveStatusFrom(state:number){
//   switch (state) {
//     case 0:
//       return TroveStatus.NonExistent;
//     case 1:
//       return TroveStatus.Active;
//     case 2:
//       return TroveStatus.ClosedByOwner;
//     case 3:
//       return TroveStatus.ClosedByLiquidation;
//     case 4:
//       return TroveStatus.ClosedByRedemption;
//     default:
//       return TroveStatus.NonExistent;
//   }
// }

// export class Trove {
//   poolIdPubkey: StringPublicKey;
//   ownerPubkey: StringPublicKey;
//   debt: number;
//   coll: number;
//   stake: number;
//   status: TroveStatus;
//   array_index: number;

//   constructor(obj: {
//     poolIdPubkey: StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     debt: BN;
//     coll: BN;
//     stake: BN;
//     status: number;
//     array_index: BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.debt = obj.debt.toNumber();
//     this.coll = obj.coll.toNumber();
//     this.stake = obj.stake.toNumber();
//     this.status = getTroveStatusFrom(obj.status);
//     this.array_index = obj.array_index.toNumber();
//   }
// }

// export class RewardSnapshot {
//   poolIdPubkey: StringPublicKey;
//   ownerPubkey: StringPublicKey;
//   sol: number;
//   solusdDebt: number;

//   constructor(obj: {
//     poolIdPubkey: StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     sol: BN;
//     solusdDebt: BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.sol = obj.sol.toNumber();
//     this.solusdDebt = obj.solusdDebt.toNumber();
//   }
// }

// export class LocalVariablesOuterLiquidationFunction {
//   price: number;
//   solusdInStabPool: number;
//   recorveryModeAtStart: number;
//   liquidatedDebt: number;
//   liquidatedColl: number;

//   constructor(obj: {
//     price: BN;
//     solusdInStabPool: BN;
//     recorveryModeAtStart: number;
//     liquidatedDebt: BN;
//     liquidatedColl: BN;
//   }) {
//     this.price = obj.price.toNumber();
//     this.solusdInStabPool = obj.solusdInStabPool.toNumber();
//     this.recorveryModeAtStart = obj.recorveryModeAtStart;
//     this.liquidatedDebt = obj.liquidatedDebt.toNumber();
//     this.liquidatedColl = obj.liquidatedColl.toNumber();
//   }
// }

// export class LocalVariablesInnerSingleLiquidateFunction {
//   collToLiquidate: number;
//   pendingDebtReward: number;
//   pendingCollReward: number;

//   constructor(obj: {
//     collToLiquidate: BN;
//     pendingDebtReward: BN;
//     pendingCollReward: BN;
//   }) {
//     this.collToLiquidate = obj.collToLiquidate.toNumber();
//     this.pendingDebtReward = obj.pendingDebtReward.toNumber();
//     this.pendingCollReward = obj.pendingCollReward.toNumber();
//   }
// }

// export class LocalVariablesLiquidationSequence {
//   remainingSOLUSDInStabPool:number;
//   i:number;
//   icr:number;
//   user?:StringPublicKey;
//   backToNormalMode:number;
//   entireSystemDebt:number;
//   entireSystemColl:number;

//   constructor(obj: {
//     remainingSOLUSDInStabPool:BN;
//     i:BN;
//     icr:BN;
//     user?:StringPublicKey;
//     backToNormalMode:number;
//     entireSystemDebt:BN;
//     entireSystemColl:BN;
//   }) {
//     this.remainingSOLUSDInStabPool = obj.remainingSOLUSDInStabPool.toNumber();
//     this.i = obj.i.toNumber();
//     this.icr = obj.icr.toNumber();
//     if(obj.user === undefined || obj.user === null){
//       this.user = undefined;
//     }
//     else{
//       this.user = obj.user);
//     }

//     this.backToNormalMode = obj.backToNormalMode;
//     this.entireSystemDebt = obj.entireSystemDebt.toNumber();
//     this.entireSystemColl = obj.entireSystemColl.toNumber();
//   }
// }

// export class LiquidationValues {
//   entireTroveDebt:number;
//   entireTroveColl:number;
//   collGasCompensation:number;
//   solusdGasCompensation:number;
//   debtToOffset:number;
//   collToSendToSp:number;
//   debtToRedistribute:number;
//   collToRedistribute:number;
//   collSurplus:number;

//   constructor(obj: {
//     entireTroveDebt:BN;
//     entireTroveColl:BN;
//     collGasCompensation:BN;
//     solusdGasCompensation:BN;
//     debtToOffset:BN;
//     collToSendToSp:BN;
//     debtToRedistribute:BN;
//     collToRedistribute:BN;
//     collSurplus:BN;
//   }) {
//     this.entireTroveDebt = obj.entireTroveDebt.toNumber();
//     this.entireTroveColl = obj.entireTroveColl.toNumber();
//     this.collGasCompensation = obj.collGasCompensation.toNumber();
//     this.solusdGasCompensation = obj.solusdGasCompensation.toNumber();
//     this.debtToOffset = obj.debtToOffset.toNumber();
//     this.collToSendToSp = obj.collToSendToSp.toNumber();
//     this.debtToRedistribute = obj.debtToRedistribute.toNumber();
//     this.collToRedistribute = obj.collToRedistribute.toNumber();
//     this.collSurplus = obj.collSurplus.toNumber();
//   }
// }

// export class LiquidationTotals {
//   totalCollInSequence:number;
//   totalDebtInSequence:number;
//   totalCollGasCompensation:number;
//   totalSolusdGasCompensation:number;
//   totalDebtToOffset:number;
//   totalCollToSendToSp:number;
//   totalDebtToRedistribute:number;
//   totalCollToRedistribute:number;
//   totalCollSurplus:number;

//   constructor(obj: {
//     totalCollInSequence:BN;
//     totalDebtInSequence:BN;
//     totalCollGasCompensation:BN;
//     totalSolusdGasCompensation:BN;
//     totalDebtToOffset:BN;
//     totalCollToSendToSp:BN;
//     totalDebtToRedistribute:BN;
//     totalCollToRedistribute:BN;
//     totalCollSurplus:BN;
//   }) {
//     this.totalCollInSequence = obj.totalCollInSequence.toNumber();
//     this.totalDebtInSequence = obj.totalDebtInSequence.toNumber();
//     this.totalCollGasCompensation = obj.totalCollGasCompensation.toNumber();
//     this.totalSolusdGasCompensation = obj.totalSolusdGasCompensation.toNumber();
//     this.totalDebtToOffset = obj.totalDebtToOffset.toNumber();
//     this.totalCollToSendToSp = obj.totalCollToSendToSp.toNumber();
//     this.totalDebtToRedistribute = obj.totalDebtToRedistribute.toNumber();
//     this.totalCollToRedistribute = obj.totalCollToRedistribute.toNumber();
//     this.totalCollSurplus = obj.totalCollSurplus.toNumber();
//   }
// }

// export class RedemptionTotals {
//   remainingSolusd:number;
//   totalSolusdToRedeem:number;
//   totalSolDrawn:number;
//   solFee:number;
//   solToSendToRedeemer:number;
//   decayedBaseRate:number;
//   price:number;
//   totalSolusdSupplyAtStart:number;

//   constructor(obj: {
//     remainingSolusd:BN;
//     totalSolusdToRedeem:BN;
//     totalSolDrawn:BN;
//     solFee:BN;
//     solToSendToRedeemer:BN;
//     decayedBaseRate:BN;
//     price:BN;
//     totalSolusdSupplyAtStart:BN;
//   }) {
//     this.remainingSolusd = obj.remainingSolusd.toNumber();
//     this.totalSolusdToRedeem = obj.totalSolusdToRedeem.toNumber();
//     this.totalSolDrawn = obj.totalSolDrawn.toNumber();
//     this.solFee = obj.solFee.toNumber();
//     this.solToSendToRedeemer = obj.solToSendToRedeemer.toNumber();
//     this.decayedBaseRate = obj.decayedBaseRate.toNumber();
//     this.price = obj.price.toNumber();
//     this.totalSolusdSupplyAtStart = obj.totalSolusdSupplyAtStart.toNumber();
//   }
// }

// export class SingleRedemptionValues {
//   solusdLot:number;
//   solLot:number;
//   cancelledPartial:number;

//   constructor(obj: {
//     solusdLot:BN;
//     solLot:BN;
//     cancelledPartial:number;
//   }) {
//     this.solusdLot = obj.solusdLot.toNumber();
//     this.solLot = obj.solLot.toNumber();
//     this.cancelledPartial = obj.cancelledPartial;
//   }
// }

// export class ActivePool {
//   borrowerOperationsAddress: StringPublicKey;
//   troveManagerAddress: StringPublicKey;
//   stabilityPoolAddress: StringPublicKey;
//   defaultPoolAddress: StringPublicKey;
//   sol: number;
//   solusdDebt: number;

//   constructor(obj: {
//     borrowerOperationsAddress: StringPublicKey;
//     troveManagerAddress: StringPublicKey;
//     stabilityPoolAddress: StringPublicKey;
//     defaultPoolAddress: StringPublicKey;
//     sol: BN;
//     solusdDebt: BN;
//   }) {
//     this.borrowerOperationsAddress = obj.borrowerOperationsAddress);
//     this.troveManagerAddress = obj.troveManagerAddress);
//     this.stabilityPoolAddress = obj.stabilityPoolAddress);
//     this.defaultPoolAddress = obj.defaultPoolAddress);
//     this.sol = obj.sol.toNumber();
//     this.solusdDebt = obj.solusdDebt.toNumber();
//   }
// }

// export class CollSurplusPool {
//   borrowerOperationsAddress: StringPublicKey;
//   troveManagerAddress: StringPublicKey;
//   activePoolAddress: StringPublicKey;
//   sol: number;

//   constructor(obj: {
//     borrowerOperationsAddress: StringPublicKey;
//     troveManagerAddress: StringPublicKey;
//     activePoolAddress: StringPublicKey;
//     sol: BN;
//   }) {
//     this.borrowerOperationsAddress = obj.borrowerOperationsAddress);
//     this.troveManagerAddress = obj.troveManagerAddress);
//     this.activePoolAddress = obj.activePoolAddress);
//     this.sol = obj.sol.toNumber();
//   }
// }

// export class DefaultPool {
//   troveManagerAddress: StringPublicKey;
//   activePoolAddress: StringPublicKey;
//   sol: number;
//   solusdDebt: number;

//   constructor(obj: {
//     troveManagerAddress: StringPublicKey;
//     activePoolAddress: StringPublicKey;
//     sol: BN;
//     solusdDebt: BN;
//   }) {
//     this.troveManagerAddress = obj.troveManagerAddress);
//     this.activePoolAddress = obj.activePoolAddress);
//     this.sol = obj.sol.toNumber();
//     this.solusdDebt = obj.solusdDebt.toNumber();
//   }
// }

// export class LocalVariablesAdjustTrove {
//   poolIdPubkey:StringPublicKey;
//   ownerPubkey:StringPublicKey;
//   price:number;
//   collChange:number;
//   netDebtChange:number;
//   isCollIncrease:number;
//   debt:number;
//   coll:number;
//   oldIcr:number;
//   newIcr:number;
//   newTcr:number;
//   solusdFee:number;
//   newDebt:number;
//   newColl:number;
//   stake:number;

//   constructor(obj: {
//     poolIdPubkey:StringPublicKey;
//     ownerPubkey:StringPublicKey;
//     price:BN;
//     collChange:BN;
//     netDebtChange:BN;
//     isCollIncrease:number;
//     debt:BN;
//     coll:BN;
//     oldIcr:BN;
//     newIcr:BN;
//     newTcr:BN;
//     solusdFee:BN;
//     newDebt:BN;
//     newColl:BN;
//     stake:BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.price = obj.price.toNumber();
//     this.collChange = obj.collChange.toNumber();
//     this.netDebtChange = obj.netDebtChange.toNumber();
//     this.isCollIncrease = obj.isCollIncrease;
//     this.debt = obj.debt.toNumber();
//     this.coll = obj.coll.toNumber();
//     this.oldIcr = obj.oldIcr.toNumber();
//     this.newIcr = obj.newIcr.toNumber();
//     this.newTcr = obj.newTcr.toNumber();
//     this.solusdFee = obj.solusdFee.toNumber();
//     this.newDebt = obj.newDebt.toNumber();
//     this.newColl = obj.newColl.toNumber();
//     this.stake = obj.stake.toNumber();
//   }
// }

// export class LocalVariablesOpenTrove {
//   poolIdPubkey:StringPublicKey;
//   ownerPubkey:StringPublicKey;
//   price:number;
//   solusdFee:number;
//   newDebt:number;
//   compositDebt:number;
//   icr:number;
//   nicr:number;
//   stake:number;
//   arrayIndex:number;

//   constructor(obj: {
//     poolIdPubkey:StringPublicKey;
//     ownerPubkey:StringPublicKey;
//     price:BN;
//     solusdFee:BN;
//     newDebt:BN;
//     compositDebt:BN;
//     icr:BN;
//     nicr:BN;
//     stake:BN;
//     arrayIndex:BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.price = obj.price.toNumber();
//     this.solusdFee = obj.solusdFee.toNumber();
//     this.newDebt = obj.newDebt.toNumber();
//     this.compositDebt = obj.compositDebt.toNumber();
//     this.icr = obj.icr.toNumber();
//     this.nicr = obj.nicr.toNumber();
//     this.stake = obj.stake.toNumber();
//     this.arrayIndex = obj.arrayIndex.toNumber();
//   }
// }

// export class CommunityIssuance {
//   nonce:number;
//   tokenProgramPubkey:StringPublicKey;
//   solidTokenPubkey:StringPublicKey;
//   stabilityPoolPubkey:StringPublicKey;
//   totalSolidIssued:number;
//   deploymentTime:number;

//   constructor(obj: {
//     nonce:number;
//     tokenProgramPubkey:StringPublicKey;
//     solidTokenPubkey:StringPublicKey;
//     stabilityPoolPubkey:StringPublicKey;
//     totalSolidIssued:BN;
//     deploymentTime:BN;
//   }) {
//     this.nonce = obj.nonce;
//     this.tokenProgramPubkey = obj.tokenProgramPubkey);
//     this.solidTokenPubkey = obj.solidTokenPubkey);
//     this.stabilityPoolPubkey = obj.stabilityPoolPubkey);
//     this.totalSolidIssued = obj.totalSolidIssued.toNumber();
//     this.deploymentTime = obj.deploymentTime.toNumber();
//   }
// }

// export class UserDeposit {
//   poolIdPubkey:StringPublicKey;

//   /// owner pubkey
//   ownerPubkey:StringPublicKey;

//   /// deposited amount
//   depositAmount:number;

//   constructor(obj: {
//     poolIdPubkey:StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     depositAmount:BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.depositAmount = obj.depositAmount.toNumber();
//   }
// }

// export class Snapshot {
//   poolIdPubkey:StringPublicKey;
//   ownerPubkey:StringPublicKey;
//   fSolSnapshot:number;
//   fSolusdSnapshot:number;

//   constructor(obj: {
//     poolIdPubkey:StringPublicKey;
//     ownerPubkey: StringPublicKey;
//     fSolSnapshot:BN;
//     fSolusdSnapshot:BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.ownerPubkey = obj.ownerPubkey);
//     this.fSolSnapshot = obj.fSolSnapshot.toNumber();
//     this.fSolusdSnapshot = obj.fSolusdSnapshot.toNumber();
//   }
// }

// export class EpochToScale {
//   poolIdPubkey:StringPublicKey;
//   scale:number;
//   epoch:number;
//   epochToScaleToSum:number;
//   epochToScaleToG:number;

//   constructor(obj: {
//     poolIdPubkey:StringPublicKey;
//     scale:BN;
//     epoch:BN;
//     epochToScaleToSum:BN;
//     epochToScaleToG:BN;
//   }) {
//     this.poolIdPubkey = obj.poolIdPubkey);
//     this.scale = obj.scale.toNumber();
//     this.epoch = obj.epoch.toNumber();
//     this.epochToScaleToSum = obj.epochToScaleToSum.toNumber();
//     this.epochToScaleToG = obj.epochToScaleToG.toNumber();
//   }
// }

// export const LIQUITY_SCHEMA: Schema = new Map<any,any>([
//   [
//     StabilityPool,
//     {
//       kind: "struct",
//       fields: [
//         ["nonce", "u8"],
//         ["tokenProgramPubkey", [32]],
//         ["SOLUSDPoolTokenPubkey", [32]],
//         ["borrowerOperationsPubkey", [32]],
//         ["troveManagerPubkey", [32]],
//         ["communityIssuancePubkey", [32]],
//         ["totalSOLUSDDeposits", "u128"],
//         ["lastSOLIDError", "u128"],
//         ["lastSOLErrorOffset", "u128"],
//         ["lastSOLUSDLossErrorOffset", "u128"],
//         ["p", "u128"],
//         ["currentScale", "u128"],
//         ["currentEpoch", "u128"],
//         ["sol", "u128"],
//         ["oracleProgramId", [32]],
//         ["quoteCurrency", [32]],
//       ],
//     },
//   ],
//   [
//     Frontend,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["kickbackRate", "u128"],
//         ["registered", "u8"],
//         ["frontendStake", "u128"],
//       ],
//     },
//   ],
//   [
//     Deposit,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["initialValue", "u128"],
//         ["frontendTag", [32]],
//       ],
//     },
//   ],
//   [
//     Snapshots,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["s", "u128"],
//         ["p", "u128"],
//         ["g", "u128"],
//         ["scale", "u128"],
//         ["epoch", "u128"],
//       ],
//     },
//   ],
//   [
//     TroveManager,
//     {
//       kind: "struct",
//       fields: [
//         ["nonce",  "u8"],
//         ["borrowerOperationsId",  [32]],
//         ["stabilityPoolId",  [32]],
//         ["gasPoolId",  [32]],
//         ["collSurplusPoolId",  [32]],
//         ["SOLUSDTokenPubkey",  [32]],
//         ["SOLIDTokenPubkey",  [32]],
//         ["SOLIDStakingPubkey",  [32]],
//         ["tokenProgramId",  [32]],
//         ["defaultPoolId",  [32]],
//         ["activePoolId",  [32]],
//         ["oracleProgramId",  [32]],
//         ["pythProductId",  [32]],
//         ["pythPriceId",  [32]],
//         ["quoteCurrency",  [32]],
//         ["baseRate",  "u128"],
//         ["lasstFeeOperationTime",  "u128"],
//         ["totalStakes",  "u128"],
//         ["totalStakesSnapshot",  "u128"],
//         ["totalCollateralSnapshot",  "u128"],
//         ["lSol",  "u128"],
//         ["lSOLUSDDebt",  "u128"],
//         ["lastSOLErrorRedistribution",  "u128"],
//         ["lastSOLUSDDebtErrorRedistribution",  "u128"],
//       ],
//     },
//   ],
//   [
//     Trove,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["debt", "u128"],
//         ["coll", "u128"],
//         ["stake", "u128"],
//         ["status", "u8"],
//         ["array_index", "u128"],
//       ],
//     },
//   ],
//   [
//     RewardSnapshot,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["sol", "u128"],
//         ["solusdDebt", "u128"],
//       ],
//     },
//   ],
//   [
//     LocalVariablesOuterLiquidationFunction,
//     {
//       kind: "struct",
//       fields: [
//         ["price", "u128"],
//         ["solusdInStabPool", "u128"],
//         ["recorveryModeAtStart", "u8"],
//         ["liquidatedDebt", "u128"],
//         ["liquidatedColl", "u128"],
//       ],
//     },
//   ],
//   [
//     LocalVariablesInnerSingleLiquidateFunction,
//     {
//       kind: "struct",
//       fields: [
//         ["collToLiquidate", "u128"],
//         ["pendingDebtReward", "u128"],
//         ["pendingCollReward", "u128"],
//       ],
//     },
//   ],
//   [
//     LocalVariablesLiquidationSequence,
//     {
//       kind: "struct",
//       fields: [
//         ["remainingSOLUSDInStabPool", "u128"],
//         ["i", "u128"],
//         ["icr", "u128"],
//         ["user", [32]],
//         ["backToNormalMode", "u8"],
//         ["entireSystemDebt", "u128"],
//         ["entireSystemColl", "u128"],
//       ],
//     },
//   ],
//   [
//     LiquidationValues,
//     {
//       kind: "struct",
//       fields: [
//         ["entireTroveDebt", "u128"],
//         ["entireTroveColl", "u128"],
//         ["collGasCompensation", "u128"],
//         ["solusdGasCompensation", "u128"],
//         ["debtToOffset", "u128"],
//         ["collToSendToSp", "u128"],
//         ["debtToRedistribute", "u128"],
//         ["collToRedistribute", "u128"],
//         ["collSurplus", "u128"],
//       ],
//     },
//   ],
//   [
//     LiquidationTotals,
//     {
//       kind: "struct",
//       fields: [
//         ["totalCollInSequence", "u128"],
//         ["totalDebtInSequence", "u128"],
//         ["totalCollGasCompensation", "u128"],
//         ["totalSolusdGasCompensation", "u128"],
//         ["totalDebtToOffset", "u128"],
//         ["totalCollToSendToSp", "u128"],
//         ["totalDebtToRedistribute", "u128"],
//         ["totalCollToRedistribute", "u128"],
//         ["totalCollSurplus", "u128"],
//       ],
//     },
//   ],
//   [
//     RedemptionTotals,
//     {
//       kind: "struct",
//       fields: [
//         ["remainingSolusd", "u128"],
//         ["totalSolusdToRedeem", "u128"],
//         ["totalSolDrawn", "u128"],
//         ["solFee", "u128"],
//         ["solToSendToRedeemer", "u128"],
//         ["decayedBaseRate", "u128"],
//         ["price", "u128"],
//         ["totalSolusdSupplyAtStart", "u128"],
//       ],
//     },
//   ],
//   [
//     SingleRedemptionValues,
//     {
//       kind: "struct",
//       fields: [
//         ["solusdLot", "u128"],
//         ["solLot", "u128"],
//         ["cancelledPartial", "u8"],
//       ],
//     },
//   ],
//   [
//     ActivePool,
//     {
//       kind: "struct",
//       fields: [
//         ["borrowerOperationsAddress", [32]],
//         ["troveManagerAddress", [32]],
//         ["stabilityPoolAddress", [32]],
//         ["defaultPoolAddress", [32]],
//         ["sol", "u128"],
//         ["solusdDebt", "u128"],
//       ],
//     },
//   ],
//   [
//     CollSurplusPool,
//     {
//       kind: "struct",
//       fields: [
//         ["borrowerOperationsAddress", [32]],
//         ["troveManagerAddress", [32]],
//         ["activePoolAddress", [32]],
//         ["sol", "u128"],
//       ],
//     },
//   ],
//   [
//     DefaultPool,
//     {
//       kind: "struct",
//       fields: [
//         ["troveManagerAddress", [32]],
//         ["activePoolAddress", [32]],
//         ["sol", "u128"],
//         ["solusdDebt", "u128"],
//       ],
//     },
//   ],
//   [
//     BorrowerOperations,
//     {
//       kind: "struct",
//       fields: [
//         ["nonce", "u8"],
//         ["tokenProgramPubkey", [32]],
//         ["activePoolId", [32]],
//         ["defaultPoolId", [32]],
//         ["stabilityPoolId", [32]],
//         ["gasPoolId", [32]],
//         ["collSurplusPoolId", [32]],
//         ["solusdTokenId", [32]],
//         ["solidStakingId", [32]],
//         ["oracleProgramId", [32]],
//         ["pythProductId", [32]],
//         ["pythPriceId", [32]],
//         ["quoteCurrency", [32]],
//       ],
//     },
//   ],
//   [
//     LocalVariablesAdjustTrove,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["price", "u128"],
//         ["collChange", "u128"],
//         ["netDebtChange", "u128"],
//         ["isCollIncrease", "u8"],
//         ["debt", "u128"],
//         ["coll", "u128"],
//         ["oldIcr", "u128"],
//         ["newIcr", "u128"],
//         ["newTcr", "u128"],
//         ["solusdFee", "u128"],
//         ["newDebt", "u128"],
//         ["newColl", "u128"],
//         ["stake", "u128"],
//       ],
//     },
//   ],
//   [
//     LocalVariablesOpenTrove,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["price", "u128"],
//         ["solusdFee", "u128"],
//         ["newDebt", "u128"],
//         ["compositDebt", "u128"],
//         ["icr", "u128"],
//         ["nicr", "u128"],
//         ["stake", "u128"],
//         ["arrayIndex", "u128"],
//       ],
//     },
//   ],
//   [
//     CommunityIssuance,
//     {
//       kind: "struct",
//       fields: [
//         ["nonce", "u8"],
//         ["tokenProgramPubkey", [32]],
//         ["solidTokenPubkey", [32]],
//         ["stabilityPoolPubkey", [32]],
//         ["totalSolidIssued", "u128"],
//         ["deploymentTime", "u128"],
//       ],
//     },
//   ],
//   [
//     SOLIDStaking,
//     {
//       kind: "struct",
//       fields: [
//         ["nonce", "u8"],
//         ["tokenProgramPubkey", [32]],
//         ["solidPoolTokenPubkey", [32]],
//         ["troveManagerId", [32]],
//         ["borrowerOperationsId", [32]],
//         ["activePoolId", [32]],
//         ["totalStakedAmount", "u128"],
//         ["fSol", "u128"],
//         ["fSolusd", "u128"],
//       ],
//     },
//   ],
//   [
//     UserDeposit,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["depositAmount", "u64"],
//       ],
//     },
//   ],
//   [
//     Snapshot,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["ownerPubkey", [32]],
//         ["fSolSnapshot", "u64"],
//         ["fSolusdSnapshot", "u64"],
//       ],
//     },
//   ],
//   [
//     EpochToScale,
//     {
//       kind: "struct",
//       fields: [
//         ["poolIdPubkey", [32]],
//         ["scale", "u128"],
//         ["epoch", "u128"],
//         ["epochToScaleToSum", "u128"],
//         ["epochToScaleToG", "u128"],
//       ],
//     },
//   ],
// ]);

// export async function retrievSchema(connection: Connection,account: StringPublicKey, classType:any): Promise<any>{
//   let data = await connection.getAccountInfo(
//     account,
//     "processed"
//   );
//   if (data === null) {
//     throw new Error("Invalid account provided");
//   }
//   let res: any = deserializeUnchecked(
//     LIQUITY_SCHEMA,
//     classType,
//     data.data
//   );
//   return res;
// }
// export function parse(address: StringPublicKey, data: Buffer, classType:any): any {
//   let res: any = deserializeUnchecked(
//     LIQUITY_SCHEMA,
//     classType,
//     data
//   );
//   res.address = address;
//   return res;
// }
