import React from 'react';
import {
  Nav,
  NavLink,
  Bars,
  NavMenu,
} from './NavbarElements';
import { Wallet } from './web3/Wallet';
  
function Navbar() {
  return (
    <>
      <Nav>
        <Bars />
        <NavMenu>
          <NavLink to='/#'>
            Home
          </NavLink>
        </NavMenu>
        <Wallet/>
      </Nav>
    </>
  );
};
  
export default Navbar;