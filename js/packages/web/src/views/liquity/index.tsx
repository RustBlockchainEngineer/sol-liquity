import React, { useEffect, useState } from 'react';
import {
  Steps,
  Button,
  Input,
  Form,
  Select,
  Switch,
  DatePicker
} from 'antd';
import { useWallet } from '@solana/wallet-adapter-react';
import moment from 'moment';
import { adjustTrove, applyPendingRewards, closeTrove, createStabilityPool, liquidate, liquidateTroves, openTrove, provideToSP, redeemCollateral, withdrawFromSP } from '../../actions';
import BN from 'bn.js';
import { AccountInfo, Connection, Keypair, PublicKey } from '@solana/web3.js';
import { getFilteredProgramAccounts } from '@solana/spl-name-service';
import { Link } from 'react-router-dom';
import {SOLID_TOKEN_MINT_KEY, SOLUSD_TOKEN_MINT_KEY, useConnection, useUserAccounts} from "@oyster/common";
import { createBorrowerOperations } from '../../actions/liquity/createBorrowerOperations';
import { createTroveManager } from '../../actions/liquity/createTroveManager';
import { createSolidStaking } from '../../actions/liquity/createSolidStaking';
import { stakeSS } from '../../actions/liquity/stakeSS';
import { unstakeSS } from '../../actions/liquity/unstakeSS';

export interface Schedule{
  createdTime:number,
  releaseTime: number;
  amount: number;
}

export const LiquityView = () => {
  const connection = useConnection();
  const wallet = useWallet();
  const userTokenAccounts = useUserAccounts();

  
  async function createBO() {
    if(wallet.connected){
      const {txid} = await createBorrowerOperations(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function openTroveBO() {
    if(wallet.connected){
      const {txid} = await openTrove(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function repayTroveBO() {
    if(wallet.connected){
      const {txid} = await adjustTrove(connection, wallet, userTokenAccounts.accountByMint.get(SOLUSD_TOKEN_MINT_KEY.toBase58())?.pubkey,1);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function withdrawTroveBO() {
    if(wallet.connected){
      const {txid} = await adjustTrove(connection, wallet, userTokenAccounts.accountByMint.get(SOLUSD_TOKEN_MINT_KEY.toBase58())?.pubkey,0);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function closeTroveBO() {
    if(wallet.connected){
      const {txid} = await closeTrove(connection, wallet, userTokenAccounts.accountByMint.get(SOLUSD_TOKEN_MINT_KEY.toBase58())?.pubkey);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }


  async function createSP() {
    if(wallet.connected){
      const {txid} = await createStabilityPool(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function provideSP() {
    if(wallet.connected){
      const {txid} = await provideToSP(connection, wallet, userTokenAccounts.accountByMint.get(SOLUSD_TOKEN_MINT_KEY.toBase58())?.pubkey);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function withdrawSP() {
    if(wallet.connected){
      const {txid} = await withdrawFromSP(connection, wallet, userTokenAccounts.accountByMint.get(SOLUSD_TOKEN_MINT_KEY.toBase58())?.pubkey);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  
  async function withdrawSOLGainToTrove() {
    if(wallet.connected){
      const {txid} = await createStabilityPool(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function registerFrontend() {
    if(wallet.connected){
      const {txid} = await createStabilityPool(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }

  
  async function createSS() {
    if(wallet.connected){
      const {txid} = await createSolidStaking(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function stakeSolidStaking() {
    if(wallet.connected){
      const {txid} = await stakeSS(connection, wallet, userTokenAccounts.accountByMint.get(SOLID_TOKEN_MINT_KEY.toBase58())?.pubkey);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function unstakeSolidStaking() {
    if(wallet.connected){
      const {txid} = await unstakeSS(connection, wallet, userTokenAccounts.accountByMint.get(SOLID_TOKEN_MINT_KEY.toBase58())?.pubkey);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }

  async function createTM() {
    if(wallet.connected){
      const {txid} = await createTroveManager(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function applyPendingRewardsTM() {
    if(wallet.connected){
      const {txid} = await applyPendingRewards(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function liquidateTM() {
    if(wallet.connected){
      const {txid} = await liquidate(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function redeemCollateralTM() {
    if(wallet.connected){
      const {txid} = await redeemCollateral(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function liquidateTrovesTM() {
    if(wallet.connected){
      const {txid} = await liquidateTroves(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }

  
  return (
    <>
    <br />
    <br />
    <br />
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createBO()}>
      Create Borrower Operations
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => openTroveBO()}>
      Open Trove
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => repayTroveBO()}>
      Repay Trove
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => withdrawTroveBO()}>
      Withdraw Trove
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => closeTroveBO()}>
      Close Trove
    </Button>

    <br /><br />
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createSP()}>
      Create Stability Pool
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => provideSP()}>
      Provide To Pool
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => withdrawSP()}>
      Withdraw From Pool
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => withdrawSOLGainToTrove()}>
      Withdraw SOL Gain to Trove
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => registerFrontend()}>
      Register Frontend
    </Button>
    
    {/* <br /><br />
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createTM()}>
      Create Trove Manager
    </Button> */}

    <br /><br />
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createSS()}>
      Create Solid Staking
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => stakeSolidStaking()}>
      Stake
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => unstakeSolidStaking()}>
      Unstake
    </Button>

    <br /><br />
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createTM()}>
      Create Trove Manager
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => applyPendingRewardsTM()}>
      Apply Pending Rewards
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => liquidateTM()}>
      Liquidate
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => redeemCollateralTM()}>
      Redeem Collateral
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => liquidateTrovesTM()}>
      Liquidate Troves
    </Button>

    </>
  );
};

