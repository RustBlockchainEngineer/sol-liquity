import {
  Keypair,
  Connection,
  PublicKey,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, MintLayout, MintInfo } from "@solana/spl-token";
import {
  createAssociatedTokenAccount,
  findAssociatedTokenAddress,
  Numberu64,
} from "../utils";
import {
  initializeInstruction,
} from "../instructions/stability-pool";
import { StabilityPool } from "../state";
import BN from "bn.js";

///////////////////////////////////////////////////////

// mainnet;
export const SP_PROGRAM_ID = new PublicKey(
  "perpke6JybKfRDitCmnazpCrGN5JRApxxukhA9Js6E6"
);

export type PrimedTransaction = [Keypair[], TransactionInstruction[]];

///////////////////////////////////////////////////////

export async function createStabilityPool(
  connection: Connection,
  adminAccount: PublicKey,
  feePayer: PublicKey,
  marketSymbol: string,
  quoteMint: PublicKey,
  vCoinDecimals: number,
  initial_v_quote_amount: Numberu64
): Promise<PrimedTransaction> {
  let balance = await connection.getMinimumBalanceForRentExemption(
    MARKET_STATE_SPACE
  );
  let marketAccount = new Keypair();
  let createMarketAccount = SystemProgram.createAccount({
    fromPubkey: feePayer,
    lamports: balance,
    newAccountPubkey: marketAccount.publicKey,
    programId: PERPS_PROGRAM_ID,
    space: MARKET_STATE_SPACE,
  });

  let [vaultSigner, vaultSignerNonce] = await PublicKey.findProgramAddress(
    [marketAccount.publicKey.toBuffer()],
    PERPS_PROGRAM_ID
  );

  let quoteMintAccount = await connection.getAccountInfo(quoteMint);
  if (!quoteMintAccount) {
    throw "Could not retrieve quote mint account";
  }

  let quoteMintInfo: MintInfo = MintLayout.decode(quoteMintAccount.data);

  let marketVault = await findAssociatedTokenAddress(vaultSigner, quoteMint);

  let createVaultAccount = await createAssociatedTokenAccount(
    feePayer,
    vaultSigner,
    quoteMint
  );

  let oraclePriceAccount = marketAccount.publicKey;

  let createMarket = new createMarketInstruction({
    signerNonce: vaultSignerNonce,
    marketSymbol,
    initialVPcAmount: initial_v_quote_amount,
    coinDecimals: quoteMintInfo.decimals,
    quoteDecimals: vCoinDecimals,
  }).getInstruction(
    PERPS_PROGRAM_ID,
    marketAccount.publicKey,
    oraclePriceAccount,
    adminAccount,
    marketVault
  );

  let instructions = [createMarketAccount, createVaultAccount, createMarket];

  return [[marketAccount], instructions];
}