import { PublicKey } from '@solana/web3.js';
import idl from './idl.json';

export const WSOL_MINT_KEY = new PublicKey(
    'So11111111111111111111111111111111111111112',
  );

export const GLOBAL_STATE_TAG = "golbal-state-seed"
export const TOKEN_VAULT_TAG = "token-vault-seed"
export const USER_TROVE_TAG = "user-trove-seed"

export const SOLUSD_MINT_TAG = "solusd-mint";
export const TOKEN_VAULT_POOL_TAG = "token-vauld-pool";

export const STABLE_POOL_PROGRAM_ID = new PublicKey(
  '6AtVTqgzFmBRbmCn3ixvp7y3dqYxvHqaCg5GbQCGLfUd',
);

export const STABLE_POOL_IDL = idl;
export const SOLUSD_DECIMALS = 6; 