use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer, ID};

use crate::{
    constant::*,
    instructions::*
};

pub fn process_sp_withdraw(ctx: Context<SPWithdraw>, amount: u64, global_state_nonce: u8, sp_user_info_nonce: u8, stability_pool_nonce: u8) -> ProgramResult {
    msg!("withdrawing ...");
    
    let mut _amount = amount;
    if amount > ctx.accounts.sp_user_info.deposit_balance {
        _amount = ctx.accounts.sp_user_info.deposit_balance;
    }
    
    // transfer from pool to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.stability_solusd_pool.to_account_info(),
        to: ctx.accounts.user_solusd_token.to_account_info(),
        authority: ctx.accounts.global_state.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let signer_seeds = &[
        GLOBAL_STATE_TAG,
        &[global_state_nonce]
    ];
    let signer = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    msg!("transfering ...");
    token::transfer(cpi_ctx, _amount)?;

    msg!("updating ...");
    ctx.accounts.sp_user_info.deposit_balance -= _amount;

    Ok(())
}
