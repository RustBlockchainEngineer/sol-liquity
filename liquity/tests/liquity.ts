import * as anchor from "@project-serum/anchor";
import { initializeAccount, closeAccount } from '@project-serum/serum/lib/token-instructions'
import { GLOBAL_STATE_TAG, LIQUITY_PROGRAM_ID, PYTH_PRICE_SOL, PYTH_PRODUCT_SOL, PYTH_PROGRAM_ID, SOLUSD_MINT_TAG, SOL_MINT_ADDRESS, STABILITY_POOL_TAG, SYSTEM_PROGRAM_ID, SYSVAR_CLOCK_PUBKEY, SYSVAR_RENT_PUBKEY, TOKEN_PROGRAM_ID, TOKEN_VAULT_POOL_TAG, TOKEN_VAULT_TAG, USER_TROVE_TAG } from "./ids";
import idl from "../target/idl/stable_pool.json";
import { StablePool } from "../target/types/stable_pool";
import { AccountLayout } from "@solana/spl-token";
import { SystemProgram } from "@solana/web3.js";
import { checkWalletATA, createTokenAccountIfNotExist, sendTransaction } from "./web3";

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


export async function depositCollateral(
  connection: anchor.web3.Connection,
  wallet: any,
  amount: number,
  collateralTokenMint: anchor.web3.PublicKey = SOL_MINT_ADDRESS,
  userTokenAccount: anchor.web3.PublicKey = undefined,
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
  let [tokenVaultPoolKey, tokenVaultPoolKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_POOL_TAG), tokenVaultKey.toBuffer()],
      program.programId
    );
  const signers = []
  const instructions = []
  const transaction = new anchor.web3.Transaction();
  if (collateralTokenMint.toBase58() === SOL_MINT_ADDRESS.toBase58()) {
    let accountRentExempt = await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
      );
      const wsolUserAccount = anchor.web3.Keypair.generate();
      signers.push(wsolUserAccount)
      userTokenAccount = wsolUserAccount.publicKey;
      instructions.push(
        SystemProgram.createAccount({
          fromPubkey: wallet.publicKey,
          newAccountPubkey: userTokenAccount,
          lamports: accountRentExempt + amount,
          space: AccountLayout.span,
          programId: program.programId
        })
      )
      instructions.push(
        initializeAccount({
          account: userTokenAccount,
          mint: collateralTokenMint,
          owner: wallet.publicKey
        })
      )
  }
  else if(!userTokenAccount){
    console.log("user doesn't have any collateral");
    return;
  }
  instructions.push(
    program.instruction.depositCollateral(
      new anchor.BN(amount),
      userTroveKeyNonce,
      tokenVaultKeyNonce,
      tokenVaultPoolKeyNonce,
      {
        accounts: {
          owner: wallet.publicKey,
          userTrove: userTroveKey,
          tokenVault: tokenVaultKey,
          poolTokenColl: tokenVaultPoolKey,
          mintColl: collateralTokenMint,
          userTokenColl: userTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID
        },
      }
    )
  )
  if (collateralTokenMint.toBase58() === SOL_MINT_ADDRESS.toBase58()) {
    instructions.push(
      closeAccount({
        source: userTokenAccount,
        destination: wallet.publicKey,
        owner:wallet.publicKey
      })
    )
  }
  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("depositCollateral txid = ", tx);
}


export async function withdrawCollateral(
  connection: anchor.web3.Connection,
  wallet: any,
  amount: number,
  userTokenAccount: anchor.web3.PublicKey = undefined,
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
  let [tokenVaultPoolKey, tokenVaultPoolKeyNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_POOL_TAG), tokenVaultKey.toBuffer()],
      program.programId
    );
  const signers = []
  const instructions = []
  const transaction = new anchor.web3.Transaction();
  if (!userTokenAccount) {
    let accountRentExempt = await connection.getMinimumBalanceForRentExemption(
      AccountLayout.span
      );
      const wsolUserAccount = anchor.web3.Keypair.generate();
      signers.push(wsolUserAccount)
      userTokenAccount = wsolUserAccount.publicKey;
      instructions.push(
        SystemProgram.createAccount({
          fromPubkey: wallet.publicKey,
          newAccountPubkey: userTokenAccount,
          lamports: accountRentExempt,
          space: AccountLayout.span,
          programId: program.programId
        })
      )
      instructions.push(
        initializeAccount({
          account: userTokenAccount,
          mint: collateralTokenMint,
          owner: wallet.publicKey
        })
      )
  }
  instructions.push(
    program.instruction.withdrawCollateral(
      new anchor.BN(amount),
      userTroveKeyNonce,
      tokenVaultKeyNonce,
      tokenVaultPoolKeyNonce,
      {
        accounts: {
          owner: wallet.publicKey,
          userTrove: userTroveKey,
          tokenVault: tokenVaultKey,
          poolTokenColl: tokenVaultPoolKey,
          mintColl: collateralTokenMint,
          userTokenColl: userTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID
        },
      }
    )
  )
  if (collateralTokenMint.toBase58() === SOL_MINT_ADDRESS.toBase58()) {
    instructions.push(
      closeAccount({
        source: userTokenAccount,
        destination: wallet.publicKey,
        owner:wallet.publicKey
      })
    )
  }
  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("withdrawCollateral txid = ", tx);
}


export async function borrowSOLUSD(
  connection: anchor.web3.Connection,
  wallet: any,
  amount:number,
  mintCollKey:anchor.web3.PublicKey = SOL_MINT_ADDRESS,
) {
  const program = getLiquityProgram(connection, wallet);

  const [globalStateKey, globalStateNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
    
  const [tokenVaultKey, tokenVaultNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey, userTroveNonce] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );
  const [mintUsdKey, mintUsdNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(SOLUSD_MINT_TAG)],
      program.programId,
    );

  const globalState = await program.account.globalState.fetch(globalStateKey);
  const tokenVault = await program.account.tokenVault.fetch(tokenVaultKey);

  const paramUserUsdTokenKey = await checkWalletATA(connection, wallet.publicKey,globalState.mintUsd.toBase58());

  const transaction = new anchor.web3.Transaction()
  let instructions = [];
  const signers = [];

  const userUsdTokenKey = await createTokenAccountIfNotExist(
    connection, 
    paramUserUsdTokenKey, 
    wallet.publicKey, 
    globalState.mintUsd.toBase58(),
    null,
    transaction,
    signers
  )
  
  const tx = await program.rpc.borrowUsd(
    new anchor.BN(amount), 
    tokenVaultNonce,
    userTroveNonce,
    globalStateNonce,
    mintUsdNonce,
    {
      accounts: {
        owner: wallet.publicKey,
        tokenVault: tokenVaultKey,
        userTrove: userTroveKey,
        globalState: globalStateKey,
        mintUsd: mintUsdKey,
        userTokenUsd: userUsdTokenKey,
        mintColl: mintCollKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        oracleProgram: tokenVault.oracleProgram,
        pythProduct: tokenVault.pythProduct,
        pythPrice: tokenVault.pythPrice,
        clock: SYSVAR_CLOCK_PUBKEY,
      },
      instructions: transaction.instructions,
      signers
    }
  );
  
  console.log("tx id->",tx);

}

export async function repaySOLUSD(
  connection: anchor.web3.Connection,
  wallet: any,
  amount:number,
  mintCollKey:anchor.web3.PublicKey = SOL_MINT_ADDRESS,
) {
  const program = getLiquityProgram(connection, wallet);

  const [globalStateKey, globalStateNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
    
  const [tokenVaultKey, tokenVaultNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), mintCollKey.toBuffer()],
      program.programId,
    );
  const [userTroveKey, userTroveNonce] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );
  const [mintUsdKey, mintUsdNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(SOLUSD_MINT_TAG)],
      program.programId,
    );

  const globalState = await program.account.globalState.fetch(globalStateKey);

  const paramUserUsdTokenKey = await checkWalletATA(connection, wallet.publicKey,globalState.mintUsd.toBase58());

  const transaction = new anchor.web3.Transaction()
  const instructions = [];
  const signers = [];

  if(!paramUserUsdTokenKey){
    console.log("user doesn't have any solusd")
    return;
  }
  
  const repayInstruction = await program.instruction.repayUsd(
    new anchor.BN(amount), 
    tokenVaultNonce,
    userTroveNonce,
    globalStateNonce,
    mintUsdNonce,
    {
      accounts: {
        owner: wallet.publicKey,
        tokenVault: tokenVaultKey,
        userTrove: userTroveKey,
        globalState: globalStateKey,
        mintUsd: mintUsdKey,
        userTokenUsd: paramUserUsdTokenKey,
        mintColl: mintCollKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    }
  );
  instructions.push(repayInstruction);

  instructions.forEach((instruction)=>{
    transaction.add(instruction);
  })
  
  let tx = await sendTransaction(connection, wallet, transaction, signers);
  console.log("tx id->",tx);
}

export async function liquidateTrove(
  connection: anchor.web3.Connection,
  wallet: any,
  vaultToLiquidte:anchor.web3.PublicKey,
) {
  const program = getLiquityProgram(connection, wallet);

  const [globalStateKey, globalStateNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(GLOBAL_STATE_TAG)],
      program.programId,
    );
  const tokenVault = await program.account.tokenVault.fetchNullable(vaultToLiquidte)
  
  const [tokenVaultKey, tokenVaultNonce] =
    await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(TOKEN_VAULT_TAG), tokenVault.mintColl.toBuffer()],
      program.programId,
    );
  const [userTroveKey, userTroveNonce] =
  await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
    program.programId,
  );
  const userTrove = await program.account.userTrove.fetchNullable(userTroveKey)
  const globalState = await program.account.globalState.fetchNullable(globalStateKey)
  const tx = await program.rpc.liquidateTrove(
    globalStateNonce,
    tokenVaultNonce,
    userTroveNonce,
    {
      accounts: {
        liquidator: wallet.publicKey,
        tokenVault: tokenVaultKey,
        userTrove: userTroveKey,
        userTroveOwner: userTrove.owner,
        globalState: globalStateKey,
        mintColl: tokenVault.mintColl,
        stabilitySolusdPool: globalState.stabilitySolusdPool,
        oracleProgram: tokenVault.oracleProgram,
        pythProduct: tokenVault.pythProduct,
        pythPrice: tokenVault.pythPrice,
        clock: SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
    }
  );
  console.log("tx id->",tx);
}

// export async function spDeposit(
//   connection: anchor.web3.Connection,
//   wallet: any,
//   amount: number,
// ) {
//   const program = getLiquityProgram(connection, wallet);

//   const [globalStateKey, globalStateNonce] =
//     await anchor.web3.PublicKey.findProgramAddress(
//       [Buffer.from(GLOBAL_STATE_TAG)],
//       program.programId,
//     );
//   const tokenVault = await program.account.tokenVault.fetchNullable(vaultToLiquidte)
  
//   const [tokenVaultKey, tokenVaultNonce] =
//     await anchor.web3.PublicKey.findProgramAddress(
//       [Buffer.from(TOKEN_VAULT_TAG), tokenVault.mintColl.toBuffer()],
//       program.programId,
//     );
//   const [userTroveKey, userTroveNonce] =
//   await anchor.web3.PublicKey.findProgramAddress(
//     [Buffer.from(USER_TROVE_TAG), tokenVaultKey.toBuffer(),wallet.publicKey.toBuffer()],
//     program.programId,
//   );
//   const userTrove = await program.account.userTrove.fetchNullable(userTroveKey)
//   const globalState = await program.account.globalState.fetchNullable(globalStateKey)
//   const tx = await program.rpc.liquidateTrove(
//     new anchor.BN(amount),
//     globalStateNonce,
//     tokenVaultNonce,
//     userTroveNonce,
//     {
//       accounts: {
//         liquidator: wallet.publicKey,
//         tokenVault: tokenVaultKey,
//         userTrove: userTroveKey,
//         userTroveOwner: userTrove.owner,
//         globalState: globalStateKey,
//         mintColl: tokenVault.mintColl,
//         stabilitySolusdPool: globalState.stabilitySolusdPool,
//         oracleProgram: tokenVault.oracleProgram,
//         pythProduct: tokenVault.pythProduct,
//         pythPrice: tokenVault.pythPrice,
//         clock: SYSVAR_CLOCK_PUBKEY,
//         tokenProgram: TOKEN_PROGRAM_ID,
//       },
//     }
//   );
//   console.log("tx id->",tx);
// }
