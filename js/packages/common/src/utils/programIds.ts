import {
  BORROWER_OPERATIONS_KEY,
  SOLID_STAKING_KEY,
  STABILITY_POOL_PROGRAM_KEY,
  TROVE_MANAGER_KEY,
} from '..';
import {
  TOKEN_PROGRAM_ID,
  SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
  BPF_UPGRADE_LOADER_ID,
  SYSTEM,
} from './ids';

export const programIds = () => {
  return {
    token: TOKEN_PROGRAM_ID,
    associatedToken: SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID,
    bpf_upgrade_loader: BPF_UPGRADE_LOADER_ID,
    system: SYSTEM,
    stabilityPool: STABILITY_POOL_PROGRAM_KEY,
    borrowerOperations: BORROWER_OPERATIONS_KEY,
    troveManager: TROVE_MANAGER_KEY,
    solidStaking: SOLID_STAKING_KEY,
  };
};
