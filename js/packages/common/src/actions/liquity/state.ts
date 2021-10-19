import { deserializeUnchecked } from 'borsh';
import BN from 'bn.js';
import { StringPublicKey } from '../../utils';

export const SOLUSD_TOKEN_MINT = '';

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

export class CreateStabilityPoolArgs {
  instruction: number = 0;
  nonce: number;
  constructor(args: { nonce: number }) {
    this.nonce = args.nonce;
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
    CreateStabilityPoolArgs,
    {
      kind: 'struct',
      fields: [
        ['instruction', 'u8'],
        ['nonce', 'u8'],
      ],
    },
  ],
]);

// export class Frontend {
//   poolIdPubkey: PublicKey;
//   ownerPubkey: PublicKey;
//   kickbackRate: number;
//   registered: boolean;
//   frontendStake:number;

//   constructor(obj: {
//     poolIdPubkey: Uint8Array;
//     ownerPubkey: Uint8Array;
//     kickbackRate: BN;
//     registered: number;
//     frontendStake: BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
//     this.kickbackRate = obj.kickbackRate.toNumber();
//     this.registered = obj.registered >= 0;
//     this.frontendStake = obj.frontendStake.toNumber();
//   }
// }

// export class Deposit {
//   poolIdPubkey: PublicKey;
//   ownerPubkey: PublicKey;
//   initialValue: number;
//   frontendTag: PublicKey;

//   constructor(obj: {
//     poolIdPubkey: Uint8Array;
//     ownerPubkey: Uint8Array;
//     initialValue: BN;
//     frontendTag: Uint8Array;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
//     this.initialValue = obj.initialValue.toNumber();
//     this.frontendTag = new PublicKey(obj.frontendTag);
//   }
// }
// export class Snapshots {
//   poolIdPubkey: PublicKey;
//   ownerPubkey: PublicKey;
//   s: number;
//   p: number;
//   g: number;
//   scale: number;
//   epoch: number;

//   constructor(obj: {
//     poolIdPubkey: Uint8Array;
//     ownerPubkey: Uint8Array;
//     s: BN;
//     p: BN;
//     g: BN;
//     scale: BN;
//     epoch: BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
//     this.s = obj.s.toNumber();
//     this.p = obj.p.toNumber();
//     this.g = obj.g.toNumber();
//     this.scale = obj.scale.toNumber();
//     this.epoch = obj.epoch.toNumber();
//   }
// }

// export class TroveManager {
//   nonce: number;
//   borrowerOperationsId: PublicKey;
//   stabilityPoolId: PublicKey;
//   gasPoolId: PublicKey;
//   collSurplusPoolId: PublicKey;
//   SOLUSDTokenPubkey: PublicKey;
//   SOLIDTokenPubkey: PublicKey;
//   SOLIDStakingPubkey: PublicKey;
//   tokenProgramId: PublicKey;
//   defaultPoolId: PublicKey;
//   activePoolId: PublicKey;
//   oracleProgramId: PublicKey;
//   pythProductId: PublicKey;
//   pythPriceId: PublicKey;
//   quoteCurrency: PublicKey;
//   baseRate: number;
//   lasstFeeOperationTime: number;
//   totalStakes: number;
//   totalStakesSnapshot: number;
//   totalCollateralSnapshot: number;
//   lSol: number;
//   lSOLUSDDebt: number;
//   lastSOLErrorRedistribution: number;
//   lastSOLUSDDebtErrorRedistribution: number;

//   constructor(obj: {
//     nonce: number,
//     borrowerOperationsId: Uint8Array;
//     stabilityPoolId: Uint8Array;
//     gasPoolId: Uint8Array;
//     collSurplusPoolId: Uint8Array;
//     SOLUSDTokenPubkey: Uint8Array;
//     SOLIDTokenPubkey: Uint8Array;
//     SOLIDStakingPubkey: Uint8Array;
//     tokenProgramId: Uint8Array;
//     defaultPoolId: Uint8Array;
//     activePoolId: Uint8Array;
//     oracleProgramId: Uint8Array;
//     pythProductId: Uint8Array;
//     pythPriceId: Uint8Array;
//     quoteCurrency: Uint8Array;
//     baseRate: BN;
//     lasstFeeOperationTime: BN;
//     totalStakes: BN;
//     totalStakesSnapshot: BN;
//     totalCollateralSnapshot: BN;
//     lSol: BN;
//     lSOLUSDDebt: BN;
//     lastSOLErrorRedistribution: BN;
//     lastSOLUSDDebtErrorRedistribution: BN;
//   }) {
//     this.nonce = obj.nonce;
//     this.borrowerOperationsId = new PublicKey(obj.borrowerOperationsId);
//     this.stabilityPoolId = new PublicKey(obj.stabilityPoolId);
//     this.gasPoolId = new PublicKey(obj.gasPoolId);
//     this.collSurplusPoolId = new PublicKey(obj.collSurplusPoolId);
//     this.SOLUSDTokenPubkey = new PublicKey(obj.SOLUSDTokenPubkey);
//     this.SOLIDTokenPubkey = new PublicKey(obj.SOLIDTokenPubkey);
//     this.SOLIDStakingPubkey = new PublicKey(obj.SOLIDStakingPubkey);
//     this.tokenProgramId = new PublicKey(obj.tokenProgramId);
//     this.defaultPoolId = new PublicKey(obj.defaultPoolId);
//     this.activePoolId = new PublicKey(obj.activePoolId);
//     this.oracleProgramId = new PublicKey(obj.oracleProgramId);
//     this.pythProductId = new PublicKey(obj.pythProductId);
//     this.pythPriceId = new PublicKey(obj.pythPriceId);
//     this.quoteCurrency = new PublicKey(obj.quoteCurrency);
//     this.baseRate = obj.baseRate.toNumber();
//     this.lasstFeeOperationTime = obj.lasstFeeOperationTime.toNumber();
//     this.totalStakes = obj.totalStakes.toNumber();
//     this.totalStakesSnapshot = obj.totalStakesSnapshot.toNumber();
//     this.totalCollateralSnapshot = obj.totalCollateralSnapshot.toNumber();
//     this.lSol = obj.lSol.toNumber();
//     this.lSOLUSDDebt = obj.lSOLUSDDebt.toNumber();
//     this.lastSOLErrorRedistribution = obj.lastSOLErrorRedistribution.toNumber();
//     this.lastSOLUSDDebtErrorRedistribution = obj.lastSOLUSDDebtErrorRedistribution.toNumber();
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
//   poolIdPubkey: PublicKey;
//   ownerPubkey: PublicKey;
//   debt: number;
//   coll: number;
//   stake: number;
//   status: TroveStatus;
//   array_index: number;

//   constructor(obj: {
//     poolIdPubkey: Uint8Array;
//     ownerPubkey: Uint8Array;
//     debt: BN;
//     coll: BN;
//     stake: BN;
//     status: number;
//     array_index: BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
//     this.debt = obj.debt.toNumber();
//     this.coll = obj.coll.toNumber();
//     this.stake = obj.stake.toNumber();
//     this.status = getTroveStatusFrom(obj.status);
//     this.array_index = obj.array_index.toNumber();
//   }
// }

// export class RewardSnapshot {
//   poolIdPubkey: PublicKey;
//   ownerPubkey: PublicKey;
//   sol: number;
//   solusdDebt: number;

//   constructor(obj: {
//     poolIdPubkey: Uint8Array;
//     ownerPubkey: Uint8Array;
//     sol: BN;
//     solusdDebt: BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
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
//   user?:PublicKey;
//   backToNormalMode:number;
//   entireSystemDebt:number;
//   entireSystemColl:number;

//   constructor(obj: {
//     remainingSOLUSDInStabPool:BN;
//     i:BN;
//     icr:BN;
//     user?:Uint8Array;
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
//       this.user = new PublicKey(obj.user);
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
//   borrowerOperationsAddress: PublicKey;
//   troveManagerAddress: PublicKey;
//   stabilityPoolAddress: PublicKey;
//   defaultPoolAddress: PublicKey;
//   sol: number;
//   solusdDebt: number;

//   constructor(obj: {
//     borrowerOperationsAddress: Uint8Array;
//     troveManagerAddress: Uint8Array;
//     stabilityPoolAddress: Uint8Array;
//     defaultPoolAddress: Uint8Array;
//     sol: BN;
//     solusdDebt: BN;
//   }) {
//     this.borrowerOperationsAddress = new PublicKey(obj.borrowerOperationsAddress);
//     this.troveManagerAddress = new PublicKey(obj.troveManagerAddress);
//     this.stabilityPoolAddress = new PublicKey(obj.stabilityPoolAddress);
//     this.defaultPoolAddress = new PublicKey(obj.defaultPoolAddress);
//     this.sol = obj.sol.toNumber();
//     this.solusdDebt = obj.solusdDebt.toNumber();
//   }
// }

// export class CollSurplusPool {
//   borrowerOperationsAddress: PublicKey;
//   troveManagerAddress: PublicKey;
//   activePoolAddress: PublicKey;
//   sol: number;

//   constructor(obj: {
//     borrowerOperationsAddress: Uint8Array;
//     troveManagerAddress: Uint8Array;
//     activePoolAddress: Uint8Array;
//     sol: BN;
//   }) {
//     this.borrowerOperationsAddress = new PublicKey(obj.borrowerOperationsAddress);
//     this.troveManagerAddress = new PublicKey(obj.troveManagerAddress);
//     this.activePoolAddress = new PublicKey(obj.activePoolAddress);
//     this.sol = obj.sol.toNumber();
//   }
// }

// export class DefaultPool {
//   troveManagerAddress: PublicKey;
//   activePoolAddress: PublicKey;
//   sol: number;
//   solusdDebt: number;

//   constructor(obj: {
//     troveManagerAddress: Uint8Array;
//     activePoolAddress: Uint8Array;
//     sol: BN;
//     solusdDebt: BN;
//   }) {
//     this.troveManagerAddress = new PublicKey(obj.troveManagerAddress);
//     this.activePoolAddress = new PublicKey(obj.activePoolAddress);
//     this.sol = obj.sol.toNumber();
//     this.solusdDebt = obj.solusdDebt.toNumber();
//   }
// }

// export class BorrowerOperations {
//   nonce: number;
//   tokenProgramPubkey: PublicKey;
//   troveManagerId: PublicKey;
//   activePoolId: PublicKey;
//   defaultPoolId: PublicKey;
//   stabilityPoolId: PublicKey;
//   gasPoolId: PublicKey;
//   collSurplusPoolId: PublicKey;
//   solusdTokenId: PublicKey;
//   solidStakingId: PublicKey;

//   oracleProgramId: PublicKey;
//   pythProductId: PublicKey;
//   pythPriceId: PublicKey;
//   quoteCurrency: PublicKey;

//   constructor(obj: {
//     nonce: number;
//     tokenProgramPubkey: Uint8Array;
//     troveManagerId: Uint8Array;
//     activePoolId: Uint8Array;
//     defaultPoolId: Uint8Array;
//     stabilityPoolId: Uint8Array;
//     gasPoolId: Uint8Array;
//     collSurplusPoolId: Uint8Array;
//     solusdTokenId: Uint8Array;
//     solidStakingId: Uint8Array;

//     oracleProgramId: Uint8Array;
//     pythProductId: Uint8Array;
//     pythPriceId: Uint8Array;
//     quoteCurrency: Uint8Array;
//   }) {
//     this.nonce = obj.nonce;
//     this.tokenProgramPubkey = new PublicKey(obj.tokenProgramPubkey);
//     this.troveManagerId = new PublicKey(obj.troveManagerId);
//     this.activePoolId = new PublicKey(obj.activePoolId);
//     this.defaultPoolId = new PublicKey(obj.defaultPoolId);
//     this.stabilityPoolId = new PublicKey(obj.stabilityPoolId);
//     this.gasPoolId = new PublicKey(obj.gasPoolId);
//     this.collSurplusPoolId = new PublicKey(obj.collSurplusPoolId);
//     this.solusdTokenId = new PublicKey(obj.solusdTokenId);
//     this.solidStakingId = new PublicKey(obj.solidStakingId);
//     this.oracleProgramId = new PublicKey(obj.oracleProgramId);
//     this.pythProductId = new PublicKey(obj.pythProductId);
//     this.pythPriceId = new PublicKey(obj.pythPriceId);
//     this.quoteCurrency = new PublicKey(obj.quoteCurrency);
//   }
// }

// export class LocalVariablesAdjustTrove {
//   poolIdPubkey:PublicKey;
//   ownerPubkey:PublicKey;
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
//     poolIdPubkey:Uint8Array;
//     ownerPubkey:Uint8Array;
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
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
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
//   poolIdPubkey:PublicKey;
//   ownerPubkey:PublicKey;
//   price:number;
//   solusdFee:number;
//   newDebt:number;
//   compositDebt:number;
//   icr:number;
//   nicr:number;
//   stake:number;
//   arrayIndex:number;

//   constructor(obj: {
//     poolIdPubkey:Uint8Array;
//     ownerPubkey:Uint8Array;
//     price:BN;
//     solusdFee:BN;
//     newDebt:BN;
//     compositDebt:BN;
//     icr:BN;
//     nicr:BN;
//     stake:BN;
//     arrayIndex:BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
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
//   tokenProgramPubkey:PublicKey;
//   solidTokenPubkey:PublicKey;
//   stabilityPoolPubkey:PublicKey;
//   totalSolidIssued:number;
//   deploymentTime:number;

//   constructor(obj: {
//     nonce:number;
//     tokenProgramPubkey:Uint8Array;
//     solidTokenPubkey:Uint8Array;
//     stabilityPoolPubkey:Uint8Array;
//     totalSolidIssued:BN;
//     deploymentTime:BN;
//   }) {
//     this.nonce = obj.nonce;
//     this.tokenProgramPubkey = new PublicKey(obj.tokenProgramPubkey);
//     this.solidTokenPubkey = new PublicKey(obj.solidTokenPubkey);
//     this.stabilityPoolPubkey = new PublicKey(obj.stabilityPoolPubkey);
//     this.totalSolidIssued = obj.totalSolidIssued.toNumber();
//     this.deploymentTime = obj.deploymentTime.toNumber();
//   }
// }

// export class SOLIDStaking {
//   nonce:number;
//   tokenProgramPubkey:PublicKey;
//   solidPoolTokenPubkey: PublicKey;
//   troveManagerId: PublicKey;
//   borrowerOperationsId: PublicKey;
//   activePoolId: PublicKey;
//   totalStakedAmount:number;
//   fSol:number;
//   fSolusd:number;

//   constructor(obj: {
//     nonce:number;
//     tokenProgramPubkey:Uint8Array;
//     solidPoolTokenPubkey: Uint8Array;
//     troveManagerId: Uint8Array;
//     borrowerOperationsId: Uint8Array;
//     activePoolId: Uint8Array;
//     totalStakedAmount:BN;
//     fSol:BN;
//     fSolusd:BN;
//   }) {
//     this.nonce = obj.nonce;
//     this.tokenProgramPubkey = new PublicKey(obj.tokenProgramPubkey);
//     this.solidPoolTokenPubkey = new PublicKey(obj.solidPoolTokenPubkey);
//     this.troveManagerId = new PublicKey(obj.troveManagerId);
//     this.borrowerOperationsId = new PublicKey(obj.borrowerOperationsId);
//     this.activePoolId = new PublicKey(obj.activePoolId);
//     this.totalStakedAmount = obj.totalStakedAmount.toNumber();
//     this.fSol = obj.fSol.toNumber();
//     this.fSolusd = obj.fSolusd.toNumber();
//   }
// }

// export class UserDeposit {
//   poolIdPubkey:PublicKey;

//   /// owner pubkey
//   ownerPubkey:PublicKey;

//   /// deposited amount
//   depositAmount:number;

//   constructor(obj: {
//     poolIdPubkey:Uint8Array;
//     ownerPubkey: Uint8Array;
//     depositAmount:BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
//     this.depositAmount = obj.depositAmount.toNumber();
//   }
// }

// export class Snapshot {
//   poolIdPubkey:PublicKey;
//   ownerPubkey:PublicKey;
//   fSolSnapshot:number;
//   fSolusdSnapshot:number;

//   constructor(obj: {
//     poolIdPubkey:Uint8Array;
//     ownerPubkey: Uint8Array;
//     fSolSnapshot:BN;
//     fSolusdSnapshot:BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
//     this.ownerPubkey = new PublicKey(obj.ownerPubkey);
//     this.fSolSnapshot = obj.fSolSnapshot.toNumber();
//     this.fSolusdSnapshot = obj.fSolusdSnapshot.toNumber();
//   }
// }

// export class EpochToScale {
//   poolIdPubkey:PublicKey;
//   scale:number;
//   epoch:number;
//   epochToScaleToSum:number;
//   epochToScaleToG:number;

//   constructor(obj: {
//     poolIdPubkey:Uint8Array;
//     scale:BN;
//     epoch:BN;
//     epochToScaleToSum:BN;
//     epochToScaleToG:BN;
//   }) {
//     this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
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

// export async function retrievSchema(connection: Connection,account: PublicKey, classType:any): Promise<any>{
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
// export function parse(address: PublicKey, data: Buffer, classType:any): any {
//   let res: any = deserializeUnchecked(
//     LIQUITY_SCHEMA,
//     classType,
//     data
//   );
//   res.address = address;
//   return res;
// }
