import * as anchor from "@project-serum/anchor";
import { GLOBAL_STATE_TAG, LIQUITY_PROGRAM_ID, PYTH_PRICE_SOL, PYTH_PRODUCT_SOL, PYTH_PROGRAM_ID, SOLUSD_MINT_TAG, SOL_MINT_ADDRESS, STABILITY_POOL_TAG, SYSTEM_PROGRAM_ID, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY, TOKEN_PROGRAM_ID, TOKEN_VAULT_POOL_TAG, TOKEN_VAULT_TAG, USER_TROVE_TAG } from "./ids";
import idl from "../target/idl/stable_pool.json";
import { StablePool } from "../target/types/stable_pool";

export function getLiquityProgram(
  connection: anchor.web3.Connection,
  wallet: any,
  liquityProgramId = LIQUITY_PROGRAM_ID
) {
  if (!wallet.publicKey) throw new Error("Miss connection");
  const provider = new anchor.Provider(
    connection,
    wallet,
    anchor.Provider.defaultOptions()
  );
  const program = new anchor.Program<StablePool>(
    idl as any,
    liquityProgramId,
    provider
  );
  return program;
}

export async function createGlobalState(
  connection: anchor.web3.Connection,
  wallet: any
) {
  const program = getLiquityProgram(connection, wallet);

  let [globalStateKey, globalStateKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId
    );
  let [mintUsdKey, mintUsdKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(SOLUSD_MINT_TAG)],
      program.programId
    );
  let [stabilityPoolKey, stabilityPoolKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(STABILITY_POOL_TAG)],
      program.programId
    );

  const globalState = await program.account.globalState.fetchNullable(globalStateKey);
  if(globalState){
    console.log("already created!");
    return;
  }

  const tx = await program.rpc.createGlobalState(
    globalStateKeyNonce,
    mintUsdKeyNonce,
    stabilityPoolKeyNonce,
    {
      accounts: {
        superOwner: wallet.publicKey,
        globalState: globalStateKey,
        mintUsd: mintUsdKey,
        stabilitySolusdPool: stabilityPoolKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
        clock: SYSVAR_CLOCK_PUBKEY,
      },
    }
  );
  console.log("createGlobalState txid = ", tx);
}

export async function createTokenVault(
  connection: anchor.web3.Connection,
  wallet: any,
  collateralTokenMint: anchor.web3.PublicKey = SOL_MINT_ADDRESS,
  pythProductKey: anchor.web3.PublicKey = PYTH_PRODUCT_SOL,
  pythPriceKey: anchor.web3.PublicKey = PYTH_PRICE_SOL,
) {
  const program = getLiquityProgram(connection, wallet);

  let [globalStateKey, globalStateKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId
    );
    
  let [tokenVaultKey, tokenVaultKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), collateralTokenMint.toBuffer()],
      program.programId
    );
  let [tokenVaultPoolKey, tokenVaultPoolKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_POOL_TAG), tokenVaultKey.toBuffer()],
      program.programId
    );
    
  const tokenVault = await program.account.tokenVault.fetchNullable(tokenVaultKey);
  if(tokenVault){
    console.log("already created!");
    return;
  }

  const tx = await program.rpc.createTokenVault(
    tokenVaultKeyNonce,
    globalStateKeyNonce,
    tokenVaultPoolKeyNonce,
    {
      accounts: {
        payer: wallet.publicKey,
        tokenVault: tokenVaultKey,
        globalState: globalStateKey,
        mintColl: collateralTokenMint,
        tokenColl: tokenVaultPoolKey,
        oracleProgram: PYTH_PROGRAM_ID,
        pythProduct: pythProductKey,
        pythPrice: pythPriceKey,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
    }
  );
  console.log("createTokenVault txid = ", tx);
}

export async function createUserTrove(
  connection: anchor.web3.Connection,
  wallet: any,
  collateralTokenMint: anchor.web3.PublicKey = SOL_MINT_ADDRESS,
) {
  const program = getLiquityProgram(connection, wallet);

  let [tokenVaultKey, tokenVaultKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), collateralTokenMint.toBuffer()],
      program.programId
    );
  let [userTroveKey, userTroveKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(), wallet.publicKey.toBuffer()],
      program.programId
    );
  const userTrove = await program.account.userTrove.fetchNullable(userTroveKey);
  if(userTrove){
    console.log("already created!");
    return;
  }

  const tx = await program.rpc.createUserTrove(
    userTroveKeyNonce,
    tokenVaultKeyNonce,
    {
      accounts: {
        troveOwner: wallet.publicKey,
        userTrove: userTroveKey,
        tokenVault: tokenVaultKey,
        mintColl: collateralTokenMint,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
    }
  );
  console.log("createUserTrove txid = ", tx);
}
