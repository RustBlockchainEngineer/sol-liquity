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
import { createStabilityPool } from '../../actions';
import BN from 'bn.js';
import { AccountInfo, Connection, Keypair, PublicKey } from '@solana/web3.js';
import { getFilteredProgramAccounts } from '@solana/spl-name-service';
import { Link } from 'react-router-dom';
import {useConnection, useUserAccounts} from "@oyster/common";
import { createBorrowerOperations } from '../../actions/liquity/createBorrowerOperations';
import { createTroveManager } from '../../actions/liquity/createTroveManager';
import { createSolidStaking } from '../../actions/liquity/createSolidStaking';

export interface Schedule{
  createdTime:number,
  releaseTime: number;
  amount: number;
}

export const LiquityView = () => {
  const connection = useConnection();
  const wallet = useWallet();
  //const userTokenAccounts = useUserAccounts();

  async function createSP() {
    if(wallet.connected){
      const {txid} = await createStabilityPool(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  async function createBO() {
    if(wallet.connected){
      const {txid} = await createBorrowerOperations(connection, wallet);
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
  async function createSS() {
    if(wallet.connected){
      const {txid} = await createSolidStaking(connection, wallet);
      console.log(txid);
    }
    else{     console.log("connect your wallet");    }
  }
  return (
    <>
    <br />
    <br />
    <br />
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createSP()}>
      Create Stability Pool
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createBO()}>
      Create Borrower Operations
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createTM()}>
      Create Trove Manager
    </Button>
    <Button htmlType="submit" style={{marginLeft: 30 + 'px'}} onClick={e => createSS()}>
      Create Solid Staking
    </Button>

    </>
  );
};

