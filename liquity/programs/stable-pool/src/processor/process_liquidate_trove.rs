use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Burn, ID};

use crate::{
    constant::*,
    instructions::*,
    utils::*
};

pub fn process_liquidate_trove(ctx: Context<LiquidateTrove>, global_state_nonce: u8, token_vault_nonce: u8, user_trove_nonce: u8) -> ProgramResult {

    let market_price = get_market_price(
        *ctx.accounts.oracle_program.key,
        ctx.accounts.pyth_product,
        ctx.accounts.pyth_price,
        &ctx.accounts.clock
    )?;
    let recovery_mode = ctx.accounts.token_vault.check_recovery_mode(market_price);
    let mut (total_debt_to_offset, total_coll_to_send_to_sp) = (0,0);
    if recovery_mode {
        (total_debt_to_offset, total_coll_to_send_to_sp) = get_total_from_batch_liquidate_recovery_mode();
    }
    else {
        (total_debt_to_offset, total_coll_to_send_to_sp) = get_total_from_batch_liquidate_normal_mode();
    }

    //offset
    //redistribute

    Ok(())
}
