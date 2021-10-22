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

export interface Schedule{
  createdTime:number,
  releaseTime: number;
  amount: number;
}

export const LiquityView = () => {
  const connection = useConnection();
  const wallet = useWallet();
  const userTokenAccounts = useUserAccounts();

  const [contractSeed, setContractSeed] = useState('');
  const [mintAddress, setMintAddress] = useState('');
  const [destWallet, setDestWallet] = useState('');
  const [sourceToken, setSourceToken] = useState('');
  const [destToken, setDestToken] = useState('');
  const [legacyMode, setLegacyMode] = useState(false);
  const [schedules, setSchedules] = useState([] as Schedule[]);

  async function createSP() {
    if(wallet.connected){
      const {txid} = await createStabilityPool(connection, wallet);
      console.log(txid);
    }
    else{
      console.log("connect your wallet")
    }
    
  }
  return (
    <>
    <br />
    <br />
    <br />
    <Button htmlType="submit" style={{marginTop: 30 + 'px',marginLeft: 30 + 'px'}} onClick={e => createSP()}>
      Create Stability Pool
    </Button>

    </>
  );
};

