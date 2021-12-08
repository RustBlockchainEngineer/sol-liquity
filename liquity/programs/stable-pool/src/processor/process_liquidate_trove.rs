use anchor_lang::prelude::*;

use crate::{
    constant::*,
    instructions::*,
    utils::*,
    states::*
};

pub fn process_liquidate_trove(ctx: Context<LiquidateTrove>, _global_state_nonce: u8, _token_vault_nonce: u8, _user_trove_nonce: u8) -> ProgramResult {

    let market_price = get_market_price(
        *ctx.accounts.oracle_program.key,
        &ctx.accounts.pyth_product,
        &ctx.accounts.pyth_price,
        &ctx.accounts.clock
    )?;
    let recovery_mode = ctx.accounts.token_vault.check_recovery_mode(market_price);
    let mut totals = LiquidationTotals::new();
    
    if recovery_mode {
        get_total_from_batch_liquidate_recovery_mode(&mut ctx.accounts.global_state, &mut ctx.accounts.token_vault, &mut ctx.accounts.user_trove, &mut totals, market_price)?;
    }
    else {
        get_total_from_batch_liquidate_normal_mode(&mut ctx.accounts.global_state, &mut ctx.accounts.token_vault, &mut ctx.accounts.user_trove, &mut totals, market_price)?;
    }

    ctx.accounts.global_state.sp_solusd_amount -= totals.total_debt_to_offset;
    ctx.accounts.token_vault.active_total_debt -= totals.total_debt_to_offset;
    ctx.accounts.global_state.sp_sol_amount += totals.total_coll_to_send_to_sp;

    Ok(())
}

pub fn get_total_from_batch_liquidate_recovery_mode(global_state:&mut GlobalState, token_vault:&mut TokenVault, user_trove:&mut UserTrove, totals:&mut LiquidationTotals, price: u64) -> ProgramResult {
    let coll = user_trove.coll;
    let debt = user_trove.debt;
    let solusd_in_stab_pool = global_state.sp_solusd_amount;

    let icr = compute_cr(coll, debt, price);
    let tcr = compute_cr(token_vault.total_coll, token_vault.total_debt, price);

    if icr <= _100PCT {
        totals.total_debt_to_offset = 0;
        totals.total_coll_to_send_to_sp = 0;
        totals.total_debt_to_redistribute = debt;
        totals.total_coll_to_redistribute = coll;

        user_trove.close();
    }
    else if icr > _100PCT && icr < MCR {
        totals.total_debt_to_offset = if debt < solusd_in_stab_pool {debt} else {solusd_in_stab_pool};
        totals.total_coll_to_send_to_sp = coll * totals.total_debt_to_offset / debt;
        totals.total_debt_to_redistribute = debt - totals.total_debt_to_offset;
        totals.total_coll_to_redistribute = coll - totals.total_coll_to_send_to_sp;

        user_trove.close();
    }
    else if icr >= MCR && icr < tcr && solusd_in_stab_pool >= debt {
        let capped_coll_portion = debt * MCR / price;

        totals.total_debt_to_offset = debt;
        totals.total_coll_to_send_to_sp = capped_coll_portion;
        totals.total_coll_surplus = coll - capped_coll_portion;
        totals.total_debt_to_redistribute = 0;
        totals.total_coll_to_redistribute = 0;

        user_trove.close();
    }
    else {
        totals.total_debt_to_offset = 0;
        totals.total_coll_to_send_to_sp = 0;
        totals.total_debt_to_redistribute = 0;
        totals.total_coll_to_redistribute = 0;
    }
    Ok(())

}
pub fn get_total_from_batch_liquidate_normal_mode(global_state:&mut GlobalState, token_vault:&mut TokenVault, user_trove:&mut UserTrove, totals:&mut LiquidationTotals, price: u64) -> ProgramResult {

    let coll = user_trove.coll;
    let debt = user_trove.debt;
    let solusd_in_stab_pool = global_state.sp_solusd_amount;

    let icr = compute_cr(coll, debt, price);
    let _tcr = compute_cr(token_vault.total_coll, token_vault.total_debt, price);

    if icr < MCR {
        let coll_to_liquidate = coll - totals.total_coll_gas_compensation;
        
        totals.total_debt_to_offset = if debt < solusd_in_stab_pool {debt} else {solusd_in_stab_pool};
        totals.total_coll_to_send_to_sp = coll_to_liquidate * totals.total_debt_to_offset / debt;
        totals.total_debt_to_redistribute = debt - totals.total_debt_to_offset;
        totals.total_coll_to_redistribute = coll_to_liquidate - totals.total_coll_to_send_to_sp;

        user_trove.close();
    }
    else {
        totals.total_debt_to_offset = 0;
        totals.total_coll_to_send_to_sp = 0;
        totals.total_debt_to_redistribute = 0;
        totals.total_coll_to_redistribute = 0;
    }
    Ok(())
}
