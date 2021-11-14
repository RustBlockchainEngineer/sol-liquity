use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Burn, ID};

use crate::{
    states::*,
    error::*,
    constant::*,
    instructions::*,
    utils::*,
};

pub fn process_repay_usd(ctx: Context<RepayUsd>, amount: u64) -> ProgramResult {

    let mut _amount = amount;
    if ctx.accounts.user_trove.debt < amount {
        _amount = ctx.accounts.user_trove.debt;
    }
    // burn
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint_usd.clone(),
        to: ctx.accounts.user_token_usd.clone(),
        authority: ctx.accounts.global_state.to_account_info().clone(),
    };

    let cpi_program = ctx.accounts.token_program.clone();
    
    let seeds = [GLOBAL_STATE_TAG];
    let (global_state_key, bump) = Pubkey::find_program_address(&seeds, ctx.program_id);

    let signer_seeds = &[
        GLOBAL_STATE_TAG,
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    
    token::burn(cpi_ctx, _amount)?;

    ctx.accounts.token_vault.total_debt -= _amount;
    ctx.accounts.user_trove.debt -= _amount;

    Ok(())
}
