import React, { useEffect, useState } from 'react';
import {
  Steps,
  Button,
  Input,
  Form,
  Select,
} from 'antd';
import { QUOTE_MINT } from '../../constants';
import {
  useConnection,
  useUserAccounts,
} from '@oyster/common';
import { useWallet } from '@solana/wallet-adapter-react';
import moment from 'moment';
import {  } from '../../actions';
import BN from 'bn.js';
import { AccountInfo, Connection, PublicKey } from '@solana/web3.js';
import { getFilteredProgramAccounts } from '@solana/spl-name-service';
import Countdown from '../../components/Countdown';
import { Link } from 'react-router-dom';

export const Liquity = () => {
  const connection = useConnection();
  const wallet = useWallet();
  const userTokenAccounts = useUserAccounts();
  
  return (
    <>
      {
        ""        
      }
    </>
  );
};

