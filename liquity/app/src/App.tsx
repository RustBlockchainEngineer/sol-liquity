import { SnackbarProvider } from "notistack";
import React from "react";
import { WalletProviders } from "./components/web3/WalletProviders";
import MyRouter from "./routers/index";

function App() {
  return (
    <SnackbarProvider>
      <WalletProviders>
      <div className="bg-white text-base dark:bg-neutral-900 text-neutral-900 dark:text-neutral-200">
        <MyRouter />
      </div>
    </WalletProviders>
   </SnackbarProvider>
  );
}
export default App;