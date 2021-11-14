import React, { useState } from 'react';
import * as anchor from '@project-serum/anchor';
import { useWallet, WalletContextState } from '@solana/wallet-adapter-react';
import { borrowSOLUSD, createGlobalState, createTokenVault, createUserTrove, depositCollateral, repayCollateral, repaySOLUSD, withdrawCollateral } from '../actions';
import { Button } from '@material-ui/core';
const connection = new anchor.web3.Connection('https://api.devnet.solana.com');

const PageHome : React.FC = () => {
  const wallet:WalletContextState = useWallet();
  const [dispInfo, setDispInfo] = useState('transaction result:');

  async function createGlobalStateUI() {
    if(wallet.connected){
      const demoLog = await createGlobalState(connection, wallet);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function createTokenVaultUI() {
    if(wallet.connected){
      const demoLog = await createTokenVault(connection, wallet);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function createUserTroveUI() {
    if(wallet.connected){
      const demoLog = await createUserTrove(connection, wallet);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function depositCollateralUI() {
    if(wallet.connected){
      const demoLog = await depositCollateral(connection, wallet, 1 * 1000000000);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function repayCollateralUI() {
    if(wallet.connected){
      const demoLog = await repayCollateral(connection, wallet, 0.2 * 1000000000);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function withdrawCollateralUI() {
    if(wallet.connected){
      const demoLog = await withdrawCollateral(connection, wallet, 1 * 1000000000);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function borrowSOLUSDUI() {
    if(wallet.connected){
      const demoLog = await borrowSOLUSD(connection, wallet, 50 * 1000000);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  async function repaySOLUSDUI() {
    if(wallet.connected){
      const demoLog = await repaySOLUSD(connection, wallet,  20 * 1000000);
      setDispInfo(demoLog);
    }
    else{     setDispInfo("connect your wallet");    }
  }
  
  return (
    <div
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
    <br />
    <br />
    <Button size="medium" color="primary" variant="outlined" onClick={e => depositCollateralUI()}>
      Deposit Collateral
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => repayCollateralUI()}>
      Repay Collateral
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => withdrawCollateralUI()}>
      Withdraw Collateral
    </Button>
    <br />
    <br />
    <Button size="medium" color="primary" variant="outlined" onClick={e => borrowSOLUSDUI()}>
      Borrow SOLUSD
    </Button>
    <Button size="medium" color="primary" variant="outlined" onClick={e => repaySOLUSDUI()}>
      Repay SOLUSD
    </Button>
    <br />
    <br />
    {dispInfo}
    </div>

  );
};
  
export default PageHome;