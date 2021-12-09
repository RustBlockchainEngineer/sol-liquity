import * as anchor from '@project-serum/anchor';
import { StablePool } from '../target/types/stable_pool';
import {
  createGlobalState,
  createTokenVault,
  createUserTrove,
  depositCollateral,
  withdrawCollateral,
  borrowSOLUSD,
  repaySOLUSD,
} from "./liquity";


anchor.setProvider(anchor.Provider.env());
const program = anchor.workspace.StablePool as anchor.Program<StablePool>;
const connection = program.provider.connection;
const wallet = program.provider.wallet;

describe('liquity', () => {
  it('Create global state', async () => {
    await createGlobalState(connection, wallet);
  });
  it('Create token vault', async () => {
    await createTokenVault(connection, wallet);
  });
  it('Create user trove', async () => {
    await createUserTrove(connection, wallet);
  });
  it('Deposit collateral', async () => {
    await depositCollateral(connection, wallet, 0.1 * 1000000000);
  });
  it('Borrow SOLUSD', async () => {
    await borrowSOLUSD(connection, wallet, 10 * 1000000);
  });
  it('Repay SOLUSD', async () => {
    await repaySOLUSD(connection, wallet, 10 * 1000000);
  });
  it('Withdraw collateral', async () => {
    await withdrawCollateral(connection, wallet, 0.1 * 1000000000);
  });
});
