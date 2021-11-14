import {
    AccountInfo,
    Keypair,
    PublicKey,
    TransactionInstruction,
  } from '@solana/web3.js';
  
  import { AccountInfo as TokenAccountInfo, Token, TOKEN_PROGRAM_ID } from '@solana/spl-token';
  
  export interface TokenAccount {
    pubkey: string;
    account: AccountInfo<Buffer>;
    info: TokenAccountInfo;
  }
  
  export interface ParsedDataAccount {
    amount: number;
    rawAmount: string;
    parsedAssetAddress: string;
    parsedAccount: any;
    assetDecimals: number;
    assetIcon: any;
    name: string;
    symbol: string;
    sourceAddress: string;
    targetAddress: string;
  }
  
  
  export function approve(
    instructions: TransactionInstruction[],
    cleanupInstructions: TransactionInstruction[],
    account: PublicKey,
    owner: PublicKey,
    amount: number,
    autoRevoke = true,
  
    // if delegate is not passed ephemeral transfer authority is used
    delegate?: PublicKey,
    existingTransferAuthority?: Keypair,
  ): Keypair {
    const tokenProgram = TOKEN_PROGRAM_ID;
  
    const transferAuthority = existingTransferAuthority || Keypair.generate();
    //const delegateKey = delegate ?? transferAuthority.publicKey;
  
    instructions.push(
      Token.createApproveInstruction(
        tokenProgram,
        account,
        delegate ?? transferAuthority.publicKey,
        owner,
        [],
        amount,
      ),
    );
  
    if (autoRevoke) {
      cleanupInstructions.push(
        Token.createRevokeInstruction(tokenProgram, account, owner, []),
      );
    }
  
    return transferAuthority;
  }
  