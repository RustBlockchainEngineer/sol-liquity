use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer, ID};

use crate::{
    constant::*,
    instructions::*
};

pub fn process_withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, token_coll_nonce: u8) -> ProgramResult {
    msg!("withdrawing ...");
    
    let mut _amount = amount;
    if amount > ctx.accounts.user_trove.locked_coll_balance {
        _amount = ctx.accounts.user_trove.locked_coll_balance;
    }
    
    // transfer from pool to user
    let cpi_accounts = Transfer {
        from: ctx.accounts.pool_token_coll.to_account_info(),
        to: ctx.accounts.user_token_coll.to_account_info(),
        authority: ctx.accounts.token_vault.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let signer_seeds = &[
        TOKEN_VAULT_TAG,
        ctx.accounts.token_vault.mint_coll.as_ref(),
        &[token_vault_nonce]
    ];
    let signer = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    msg!("transfering ...");
    token::transfer(cpi_ctx, _amount)?;

    msg!("updating ...");
    ctx.accounts.token_vault.total_coll -= amount;
    ctx.accounts.user_trove.locked_coll_balance -= _amount;

    Ok(())
}
