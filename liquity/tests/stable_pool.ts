import * as anchor from '@project-serum/anchor';
import { Keypair } from '@solana/web3.js';
import { StablePool } from '../target/types/stable_pool';
import {
  createGlobalState,
  createTokenVault,
  createUserTrove,
  depositCollateral,
  withdrawCollateral,
  borrowSOLUSD,
  repaySOLUSD,
  liquidateTrove,
  getTroveKeyFromOwner,
} from "./liquity";


anchor.setProvider(anchor.Provider.env());
const program = anchor.workspace.StablePool as anchor.Program<StablePool>;
const connection = program.provider.connection;
const wallet = program.provider.wallet;
const user1 = Keypair.generate();
const MIN_SOL_AMOUNT = 5 * 1000000000;

describe('liquity', () => {
  it('Setup', async () => {
    while(await connection.getBalance(user1.publicKey) < MIN_SOL_AMOUNT){
      await connection.requestAirdrop(user1.publicKey, 5 * 1000000000);
    }
    while(await connection.getBalance(wallet.publicKey) < MIN_SOL_AMOUNT){
      await connection.requestAirdrop(wallet.publicKey, 5 * 1000000000);
    }
  });
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

  it('user1: Create user trove', async () => {
    await createUserTrove(connection, user1);
  });
  it('user1: Deposit collateral', async () => {
    await depositCollateral(connection, user1, 0.1 * 1000000000);
  });
  it('user1: Borrow SOLUSD', async () => {
    await borrowSOLUSD(connection, user1, 10 * 1000000);
  });
  it('Liquidate trove', async () => {
    const troveKey = await getTroveKeyFromOwner(connection, wallet, user1.publicKey);
    await liquidateTrove(connection, wallet, troveKey);
  });
  it('user1: Repay SOLUSD', async () => {
    await repaySOLUSD(connection, user1, 10 * 1000000);
  });
  it('user1: Withdraw collateral', async () => {
    await withdrawCollateral(connection, user1, 0.1 * 1000000000);
  });
});
