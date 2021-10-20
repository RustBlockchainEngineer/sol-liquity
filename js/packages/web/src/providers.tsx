import {
  AccountsProvider,
  ConnectionProvider,
  WalletProvider,
} from '@oyster/common';
import { FC } from 'react';
import { ConfettiProvider } from './components/Confetti';
import { AppLayout } from './components/Layout';
import { MetaProvider } from './contexts/meta';
import { CoingeckoProvider } from './contexts/coingecko';

export const Providers: FC = ({ children }) => {
  return (
    <ConnectionProvider>
      <WalletProvider>
          <AccountsProvider>
            <CoingeckoProvider>
              <AppLayout>{children}</AppLayout>
            </CoingeckoProvider>
          </AccountsProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};
