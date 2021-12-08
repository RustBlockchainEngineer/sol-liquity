import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { StablePool } from '../target/types/stable_pool';
import {createGlobalState, createTokenVault, createUserTrove} from './liquity';


anchor.setProvider(anchor.Provider.env());
const program = anchor.workspace.StablePool as Program<StablePool>;
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
});
