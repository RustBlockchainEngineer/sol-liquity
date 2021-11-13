import { PublicKey } from '@solana/web3.js';
import idl from './../../../../../liquity/target/idl/stable_pool.json';

export const TEST_KEY = new PublicKey(
  '99cUC38KwU1qn7jVpKfSpvZafn9AKxLW9BfJMWASXDJj',
);

export const STABLE_POOL_PROGRAM_ID = new PublicKey(
  'GmWiGFfV35KfSTS68fruWKFtc2rnR7wg1NtdMUQHyhQ1',
);
export const STABLE_POOL_IDL = idl;
