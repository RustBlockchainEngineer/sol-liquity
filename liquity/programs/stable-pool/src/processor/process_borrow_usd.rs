use anchor_lang::prelude::*;
use anchor_spl::token::{self,  MintTo, ID};

use crate::{
    error::*,
    constant::*,
    instructions::*,
    utils::*,
};

pub fn process_borrow_usd(ctx: Context<BorrowUsd>, amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, global_state_nonce: u8, mint_usd_nonce: u8) -> ProgramResult {

    assert_debt_allowed(ctx.accounts.user_trove.locked_coll_balance, ctx.accounts.user_trove.debt, amount)?;
    // mint to user
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_usd.to_account_info().clone(),
        to: ctx.accounts.user_token_usd.to_account_info().clone(),
        authority: ctx.accounts.global_state.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info().clone();
    
    let signer_seeds = &[
        GLOBAL_STATE_TAG,
        &[global_state_nonce],
    ];
    let signer = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::mint_to(cpi_ctx, amount)?;

    ctx.accounts.token_vault.total_debt += amount;
    ctx.accounts.user_trove.debt += amount;

    Ok(())
}

fn assert_debt_allowed(locked_coll_balance: u64, user_debt: u64, amount: u64)-> ProgramResult{
    let market_price = get_market_price();
    let debt_limit = market_price * locked_coll_balance;
    if debt_limit < user_debt + amount {
        return Err(StablePoolError::NotAllowed.into())
    }
    Ok(())
}