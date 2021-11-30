use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer, ID};

use crate::{
    instructions::*
};

pub fn process_sp_deposit(ctx: Context<SPDeposit>, amount: u64, global_state_nonce: u8, sp_user_info_nonce: u8, stability_pool_nonce: u8) -> ProgramResult {
    
    // transfer from user to pool
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_solusd_token.to_account_info().clone(),
        to: ctx.accounts.stability_solusd_pool.to_account_info().clone(),
        authority: ctx.accounts.owner.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    ctx.accounts.sp_user_info.deposit_balance += amount;

    Ok(())
}
