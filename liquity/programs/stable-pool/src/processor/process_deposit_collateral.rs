use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer};

use crate::{
    instructions::*
};

pub fn process_deposit_collateral(ctx: Context<DepositCollateral>, amount: u64, _token_vault_nonce: u8, _user_trove_nonce: u8, _token_coll_nonce: u8) -> ProgramResult {
    
    // transfer from user to pool
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_coll.to_account_info().clone(),
        to: ctx.accounts.pool_token_coll.to_account_info().clone(),
        authority: ctx.accounts.owner.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;

    ctx.accounts.token_vault.total_coll += amount;
    ctx.accounts.user_trove.coll += amount;

    Ok(())
}
