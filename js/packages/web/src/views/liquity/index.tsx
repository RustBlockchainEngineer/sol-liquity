import React from 'react';
import {
  useConnection,
  useUserAccounts,
} from '@oyster/common';
import { useWallet } from '@solana/wallet-adapter-react';

export const LiquityView = () => {
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

