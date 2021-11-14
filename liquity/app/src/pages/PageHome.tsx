import React from 'react';
import * as anchor from '@project-serum/anchor';
import { useWallet, WalletContextState } from '@solana/wallet-adapter-react';
import { borrowSOLUSD, createGlobalState, createTokenVault, createUserTrove, depositCollateral, repayCollateral, repaySOLUSD, withdrawCollateral } from '../actions';
import { Button } from '@material-ui/core';

const connection = new anchor.web3.Connection('https://api.devnet.solana.com');

const PageHome : React.FC = () => {
  const wallet:WalletContextState = useWallet();
  
  async function createGlobalStateUI() {
    if(wallet.connected){
      await createGlobalState(connection, wallet);
    }
    else{     console.log("connect your wallet");    }
  }
  async function createTokenVaultUI() {
    if(wallet.connected){
      await createTokenVault(connection, wallet);
    }
    else{     console.log("connect your wallet");    }
  }
  async function createUserTroveUI() {
    if(wallet.connected){
      await createUserTrove(connection, wallet);
    }
    else{     console.log("connect your wallet");    }
  }
  async function depositCollateralUI() {
    if(wallet.connected){
      await depositCollateral(connection, wallet, 1 * 1000000000);
    }
    else{     console.log("connect your wallet");    }
  }
  async function repayCollateralUI() {
    if(wallet.connected){
      await repayCollateral(connection, wallet, 0.2 * 1000000000);
    }
    else{     console.log("connect your wallet");    }
  }
  async function withdrawCollateralUI() {
    if(wallet.connected){
      await withdrawCollateral(connection, wallet, 1 * 1000000000);
    }
    else{     console.log("connect your wallet");    }
  }
  async function borrowSOLUSDUI() {
    if(wallet.connected){
      await borrowSOLUSD(connection, wallet);
    }
    else{     console.log("connect your wallet");    }
  }
  async function repaySOLUSDUI() {
    if(wallet.connected){
      await repaySOLUSD(connection, wallet);
    }
    else{     console.log("connect your wallet");    }
  }
  
  return (
    <div
      style={{
        display: 'flex',
        justifyContent: 'Left',
        alignItems: 'Left',
      }}
    >
    <br />
    <br />
    <Button size="medium" color="primary" variant="outlined" onClick={e => createGlobalStateUI()}>
      Create Program State
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => createTokenVaultUI()}>
      Create Token Vault
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => createUserTroveUI()}>
      Create User Trove
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => depositCollateralUI()}>
      Deposit Collateral
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => repayCollateralUI()}>
      Repay Collateral
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => withdrawCollateralUI()}>
      Withdraw Collateral
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => borrowSOLUSDUI()}>
      Borrow SOLUSD
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => repaySOLUSDUI()}>
      Repay SOLUSD
    </Button>
    </div>
  );
};
  
export default PageHome;