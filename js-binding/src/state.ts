import { PublicKey, Connection } from "@solana/web3.js";
import BN from "bn.js";
import { Schema, deserializeUnchecked } from "borsh";
import { AccountLayout } from "@solana/spl-token";


export class StabilityPool {
  nonce: number;
  tokenProgramPubkey: PublicKey;
  SOLUSDPoolTokenPubkey: PublicKey;
  borrowerOperationsPubkey: PublicKey;
  troveManagerPubkey: PublicKey;
  communityIssuancePubkey: PublicKey;
  totalSOLUSDDeposits: number;
  lastSOLIDError: number;
  lastSOLErrorOffset: number;
  lastSOLUSDLossErrorOffset:number;
  p: number;
  currentScale: number;
  currentEpoch: number;
  sol: number;
  oracleProgramId: PublicKey;
  quoteCurrency: PublicKey

  constructor(obj: {
    nonce: number;
    tokenProgramPubkey: Uint8Array;
    SOLUSDPoolTokenPubkey: Uint8Array;
    borrowerOperationsPubkey: Uint8Array;
    troveManagerPubkey: Uint8Array;
    communityIssuancePubkey: Uint8Array;
    totalSOLUSDDeposits: BN;
    lastSOLIDError: BN;
    lastSOLErrorOffset: BN;
    lastSOLUSDLossErrorOffset: BN;
    p: BN;
    currentScale: BN;
    currentEpoch: BN;
    sol: BN;
    oracleProgramId: Uint8Array;
    quoteCurrency: Uint8Array;
  }) {
    this.nonce = obj.nonce;
    this.tokenProgramPubkey = new PublicKey(obj.tokenProgramPubkey);
    this.SOLUSDPoolTokenPubkey = new PublicKey(obj.SOLUSDPoolTokenPubkey);
    this.borrowerOperationsPubkey = new PublicKey(obj.borrowerOperationsPubkey);
    this.troveManagerPubkey = new PublicKey(obj.troveManagerPubkey);
    this.communityIssuancePubkey = new PublicKey(obj.communityIssuancePubkey);
    this.totalSOLUSDDeposits = obj.totalSOLUSDDeposits.toNumber();
    this.lastSOLIDError = obj.lastSOLIDError.toNumber();
    this.lastSOLErrorOffset = obj.lastSOLErrorOffset.toNumber();
    this.lastSOLUSDLossErrorOffset = obj.lastSOLUSDLossErrorOffset.toNumber();
    this.p = obj.p.toNumber();
    this.currentScale = obj.currentScale.toNumber();
    this.currentEpoch = obj.currentEpoch.toNumber();
    this.sol = obj.sol.toNumber();
    this.oracleProgramId = new PublicKey(obj.oracleProgramId);
    this.quoteCurrency = new PublicKey(obj.quoteCurrency);
  }

}

export class Frontend {
  poolIdPubkey: PublicKey;
  ownerPubkey: PublicKey;
  kickbackRate: number;
  registered: boolean;
  frontendStake:number;

  constructor(obj: {
    poolIdPubkey: Uint8Array;
    ownerPubkey: Uint8Array;
    kickbackRate: BN;
    registered: number;
    frontendStake: BN;
  }) {
    this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
    this.ownerPubkey = new PublicKey(obj.ownerPubkey);
    this.kickbackRate = obj.kickbackRate.toNumber();
    this.registered = obj.registered >= 0;
    this.frontendStake = obj.frontendStake.toNumber();
  }
}


export class Deposit {
  poolIdPubkey: PublicKey;
  ownerPubkey: PublicKey;
  initialValue: number;
  frontendTag: PublicKey;

  constructor(obj: {
    poolIdPubkey: Uint8Array;
    ownerPubkey: Uint8Array;
    initialValue: BN;
    frontendTag: Uint8Array;
  }) {
    this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
    this.ownerPubkey = new PublicKey(obj.ownerPubkey);
    this.initialValue = obj.initialValue.toNumber();
    this.frontendTag = new PublicKey(obj.frontendTag);
  }
}
export class Snapshots {
  poolIdPubkey: PublicKey;
  ownerPubkey: PublicKey;
  s: number;
  p: number;
  g: number;
  scale: number;
  epoch: number;

  constructor(obj: {
    poolIdPubkey: Uint8Array;
    ownerPubkey: Uint8Array;
    s: BN;
    p: BN;
    g: BN;
    scale: BN;
    epoch: BN;
  }) {
    this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
    this.ownerPubkey = new PublicKey(obj.ownerPubkey);
    this.s = obj.s.toNumber();
    this.p = obj.p.toNumber();
    this.g = obj.g.toNumber();
    this.scale = obj.scale.toNumber();
    this.epoch = obj.epoch.toNumber();
  }
}


export class TroveManager {
  nonce: number;
  borrowerOperationsId: PublicKey;
  stabilityPoolId: PublicKey;
  gasPoolId: PublicKey;
  collSurplusPoolId: PublicKey;
  SOLUSDTokenPubkey: PublicKey;
  SOLIDTokenPubkey: PublicKey;
  SOLIDStakingPubkey: PublicKey;
  tokenProgramId: PublicKey;
  defaultPoolId: PublicKey;
  activePoolId: PublicKey;
  oracleProgramId: PublicKey;
  pythProductId: PublicKey;
  pythPriceId: PublicKey;
  quoteCurrency: PublicKey;
  baseRate: number;
  lasstFeeOperationTime: number;
  totalStakes: number;
  totalStakesSnapshot: number;
  totalCollateralSnapshot: number;
  lSol: number;
  lSOLUSDDebt: number;
  lastSOLErrorRedistribution: number;
  lastSOLUSDDebtErrorRedistribution: number;

  constructor(obj: {
    nonce: number,
    borrowerOperationsId: Uint8Array;
    stabilityPoolId: Uint8Array;
    gasPoolId: Uint8Array;
    collSurplusPoolId: Uint8Array;
    SOLUSDTokenPubkey: Uint8Array;
    SOLIDTokenPubkey: Uint8Array;
    SOLIDStakingPubkey: Uint8Array;
    tokenProgramId: Uint8Array;
    defaultPoolId: Uint8Array;
    activePoolId: Uint8Array;
    oracleProgramId: Uint8Array;
    pythProductId: Uint8Array;
    pythPriceId: Uint8Array;
    quoteCurrency: Uint8Array;
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
    this.borrowerOperationsId = new PublicKey(obj.borrowerOperationsId);
    this.stabilityPoolId = new PublicKey(obj.stabilityPoolId);
    this.gasPoolId = new PublicKey(obj.gasPoolId);
    this.collSurplusPoolId = new PublicKey(obj.collSurplusPoolId);
    this.SOLUSDTokenPubkey = new PublicKey(obj.SOLUSDTokenPubkey);
    this.SOLIDTokenPubkey = new PublicKey(obj.SOLIDTokenPubkey);
    this.SOLIDStakingPubkey = new PublicKey(obj.SOLIDStakingPubkey);
    this.tokenProgramId = new PublicKey(obj.tokenProgramId);
    this.defaultPoolId = new PublicKey(obj.defaultPoolId);
    this.activePoolId = new PublicKey(obj.activePoolId);
    this.oracleProgramId = new PublicKey(obj.oracleProgramId);
    this.pythProductId = new PublicKey(obj.pythProductId);
    this.pythPriceId = new PublicKey(obj.pythPriceId);
    this.quoteCurrency = new PublicKey(obj.quoteCurrency);
    this.baseRate = obj.baseRate.toNumber();
    this.lasstFeeOperationTime = obj.lasstFeeOperationTime.toNumber();
    this.totalStakes = obj.totalStakes.toNumber();
    this.totalStakesSnapshot = obj.totalStakesSnapshot.toNumber();
    this.totalCollateralSnapshot = obj.totalCollateralSnapshot.toNumber();
    this.lSol = obj.lSol.toNumber();
    this.lSOLUSDDebt = obj.lSOLUSDDebt.toNumber();
    this.lastSOLErrorRedistribution = obj.lastSOLErrorRedistribution.toNumber();
    this.lastSOLUSDDebtErrorRedistribution = obj.lastSOLUSDDebtErrorRedistribution.toNumber();
  }
}


export enum TroveStatus {
  NonExistent = 0,
  Active,
  ClosedByOwner,
  ClosedByLiquidation,
  ClosedByRedemption,

}
export function getTroveStatusFrom(state:number){
  switch (state) {
    case 0:
      return TroveStatus.NonExistent;
      break;
    case 1:
      return TroveStatus.Active;
      break;
    case 2:
      return TroveStatus.ClosedByOwner;
      break;
    case 3:
      return TroveStatus.ClosedByLiquidation;
      break;
    case 4:
      return TroveStatus.ClosedByRedemption;
      break;
    default:
      return TroveStatus.NonExistent;
      break;
  }
}

export class Trove {
  poolIdPubkey: PublicKey;
  ownerPubkey: PublicKey;
  debt: number;
  coll: number;
  stake: number;
  status: TroveStatus;
  array_index: number;

  constructor(obj: {
    poolIdPubkey: Uint8Array;
    ownerPubkey: Uint8Array;
    debt: BN;
    coll: BN;
    stake: BN;
    status: number;
    array_index: BN;
  }) {
    this.poolIdPubkey = new PublicKey(obj.poolIdPubkey);
    this.ownerPubkey = new PublicKey(obj.ownerPubkey);
    this.debt = obj.debt.toNumber();
    this.coll = obj.coll.toNumber();
    this.stake = obj.stake.toNumber();
    this.status = getTroveStatusFrom(obj.status);
    this.array_index = obj.array_index.toNumber();
  }
}

export const LIQUITY_SCHEMA: Schema = new Map<any,any>([
  [
    StabilityPool,
    {
      kind: "struct",
      fields: [
        ["nonce", "u8"],
        ["tokenProgramPubkey", [32]],
        ["SOLUSDPoolTokenPubkey", [32]],
        ["borrowerOperationsPubkey", [32]],
        ["troveManagerPubkey", [32]],
        ["communityIssuancePubkey", [32]],
        ["totalSOLUSDDeposits", "u128"],
        ["lastSOLIDError", "u128"],
        ["lastSOLErrorOffset", "u128"],
        ["lastSOLUSDLossErrorOffset", "u128"],
        ["p", "u128"],
        ["currentScale", "u128"],
        ["currentEpoch", "u128"],
        ["sol", "u128"],
        ["oracleProgramId", [32]],
        ["quoteCurrency", [32]],
      ],
    },
  ],
  [
    Frontend,
    {
      kind: "struct",
      fields: [
        ["poolIdPubkey", [32]],
        ["ownerPubkey", [32]],
        ["kickbackRate", "u128"],
        ["registered", "u8"],
        ["frontendStake", "u128"],
      ],
    },
  ],
  [
    Deposit,
    {
      kind: "struct",
      fields: [
        ["poolIdPubkey", [32]],
        ["ownerPubkey", [32]],
        ["initialValue", "u128"],
        ["frontendTag", [32]],
      ],
    },
  ],
  [
    Snapshots,
    {
      kind: "struct",
      fields: [
        ["poolIdPubkey", [32]],
        ["ownerPubkey", [32]],
        ["s", "u128"],
        ["p", "u128"],
        ["g", "u128"],
        ["scale", "u128"],
        ["epoch", "u128"],
      ],
    },
  ],
  [
    TroveManager,
    {
      kind: "struct",
      fields: [
        ["nonce",  "u8"],
        ["borrowerOperationsId",  [32]],
        ["stabilityPoolId",  [32]],
        ["gasPoolId",  [32]],
        ["collSurplusPoolId",  [32]],
        ["SOLUSDTokenPubkey",  [32]],
        ["SOLIDTokenPubkey",  [32]],
        ["SOLIDStakingPubkey",  [32]],
        ["tokenProgramId",  [32]],
        ["defaultPoolId",  [32]],
        ["activePoolId",  [32]],
        ["oracleProgramId",  [32]],
        ["pythProductId",  [32]],
        ["pythPriceId",  [32]],
        ["quoteCurrency",  [32]],
        ["baseRate",  "u128"],
        ["lasstFeeOperationTime",  "u128"],
        ["totalStakes",  "u128"],
        ["totalStakesSnapshot",  "u128"],
        ["totalCollateralSnapshot",  "u128"],
        ["lSol",  "u128"],
        ["lSOLUSDDebt",  "u128"],
        ["lastSOLErrorRedistribution",  "u128"],
        ["lastSOLUSDDebtErrorRedistribution",  "u128"],
      ],
    },
  ],
  [
    Trove,
    {
      kind: "struct",
      fields: [
        ["poolIdPubkey", [32]],
        ["ownerPubkey", [32]],
        ["debt", "u128"],
        ["coll", "u128"],
        ["stake", "u128"],
        ["status", "u8"],
        ["array_index", "u128"],
      ],
    },
  ],
]);

export async function retrievSchema(connection: Connection,account: PublicKey, classType:any): Promise<any>{
  let data = await connection.getAccountInfo(
    account,
    "processed"
  );
  if (data === null) {
    throw new Error("Invalid account provided");
  }
  let res: any = deserializeUnchecked(
    LIQUITY_SCHEMA,
    classType,
    data.data
  );
  return res;
}
export function parse(address: PublicKey, data: Buffer, classType:any): any {
  let res: any = deserializeUnchecked(
    LIQUITY_SCHEMA,
    classType,
    data
  );
  res.address = address;
  return res;
}
