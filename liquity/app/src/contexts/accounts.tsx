import React, {
    useContext,
  } from 'react';
  const AccountsContext = React.createContext<any>(null);
  
  export const useAccountsContext = () => {
    const context = useContext(AccountsContext);
  
    return context;
  };
  