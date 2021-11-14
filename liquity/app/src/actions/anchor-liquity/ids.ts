import { PublicKey } from '@solana/web3.js';


export const WSOL_MINT_KEY = new PublicKey(
    'So11111111111111111111111111111111111111112',
  );

export const GLOBAL_STATE_TAG = "golbal-state-seed"
export const TOKEN_VAULT_TAG = "token-vault-seed"
export const USER_TROVE_TAG = "user-trove-seed"


import idl from './idl.json';

export const STABLE_POOL_PROGRAM_ID = new PublicKey(
  '7diveo3fmjU42AUXnRLVtJYXGNEnR6XXGAnBd9kxpTxf',
);
export const STABLE_POOL_IDL = idl;
