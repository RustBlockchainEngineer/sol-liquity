use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Burn, ID};

use crate::{
    constant::*,
    instructions::*,
    utils::*,
    states::*
};

pub fn process_liquidate_trove(ctx: Context<LiquidateTrove>, global_state_nonce: u8, token_vault_nonce: u8, user_trove_nonce: u8) -> ProgramResult {

    let market_price = get_market_price(
        *ctx.accounts.oracle_program.key,
        ctx.accounts.pyth_product,
        ctx.accounts.pyth_price,
        &ctx.accounts.clock
    )?;
    let recovery_mode = ctx.accounts.token_vault.check_recovery_mode(market_price);
    let mut totals = LiquidationTotals::new();
    
    if recovery_mode {
        get_total_from_batch_liquidate_recovery_mode(ctx, &mut totals, market_price)?;
    }
    else {
        get_total_from_batch_liquidate_normal_mode(ctx, &mut totals, market_price)?;
    }

    //offset
    //redistribute

    Ok(())
}


pub fn get_total_from_batch_liquidate_recovery_mode(ctx: Context<LiquidateTrove>, totals:&mut LiquidationTotals, price: u128) -> ProgramResult {
    let coll = user_trove.coll;
    let debt = user_trove.debt;
    let solusd_in_stab_pool = ctx.accounts.global_state.total_solusd_amount;

    let icr = compute_cr(coll, debt, price);
    let tcr = compute_cr(ctx.accounts.token_vault.total_coll, ctx.accounts.token_vault.total_debt, price);

    if icr <= _100PCT {

        totals.debt_to_offset = 0;
        totals.coll_to_send_to_sp = 0;
        totals.debt_to_redistribute = debt;
        totals.coll_to_redistribute = coll;

        ctx.accounts.user_trove.close();
    }
    else if icr > _100PCT && icr < MCR {
        totals.debt_to_offset = if debt < solusd_in_stab_pool {debt} else {solusd_in_stab_pool};
        totals.coll_to_send_to_sp = coll * debt_to_offset / debt;
        totals.debt_to_redistribute = debt - debt_to_offset;
        totals.coll_to_redistribute = coll - coll_to_send_to_sp;

        ctx.accounts.user_trove.close();
    }
    else if icr >= MCR && icr < tcr && solusd_in_stab_pool >= debt {
        let capped_coll_portion = debt * MCR / price;

        // single_liquidation.coll_gas_compensation = get_coll_gas_compensation(capped_coll_portion);
        // single_liquidation.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;

        totals.debt_to_offset = debt;
        totals.coll_to_send_to_sp = capped_coll_portion - single_liquidation.coll_gas_compensation;
        totals.coll_surplus = coll - capped_coll_portion;
        totals.debt_to_redistribute = 0;
        totals.coll_to_redistribute = 0;

        ctx.accounts.user_trove.close();
    }
    else {
        totals.debt_to_offset = 0;
        totals.coll_to_send_to_sp = 0;
        totals.debt_to_redistribute = 0;
        totals.coll_to_redistribute = 0;
    }
    Ok(())

}
pub fn get_total_from_batch_liquidate_normal_mode(ctx: Context<LiquidateTrove>, totals:&mut LiquidationTotals, price: u128) -> ProgramResult {

    let coll = user_trove.coll;
    let debt = user_trove.debt;
    let solusd_in_stab_pool = ctx.accounts.global_state.total_solusd_amount;

    let icr = compute_cr(coll, debt, price);
    let tcr = compute_cr(ctx.accounts.token_vault.total_coll, ctx.accounts.token_vault.total_debt, price);

    if icr < MCR {
        // totals.coll_gas_compensation = get_coll_gas_compensation(single_liquidation.entire_trove_coll);
        // totals.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;

        let coll_to_liquidate = coll - totals.coll_gas_compensation;
        
        totals.debt_to_offset = if debt < solusd_in_stab_pool {debt} else {solusd_in_stab_pool};
        totals.coll_to_send_to_sp = coll_to_liquidate * debt_to_offset / debt;
        totals.debt_to_redistribute = debt - debt_to_offset;
        totals.coll_to_redistribute = coll_to_liquidate - coll_to_send_to_sp;

        ctx.accounts.user_trove.close();
    }
    else {
        totals.debt_to_offset = 0;
        totals.coll_to_send_to_sp = 0;
        totals.debt_to_redistribute = 0;
        totals.coll_to_redistribute = 0;
    }
    Ok(())
}
