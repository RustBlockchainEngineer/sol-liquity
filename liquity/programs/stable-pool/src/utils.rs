use anchor_lang::prelude::*;
use crate::{
    pyth::*,
    error::*,
    constant::*,
    states::*,
};
use std::u128::MAX;
use std::convert::TryInto;
use std::convert::TryFrom;
use spl_math::{precise_number::PreciseNumber};

pub fn get_pyth_product_quote_currency(pyth_product: &Product) -> Result<[u8; 32]> {
    const LEN: usize = 14;
    const KEY: &[u8; LEN] = b"quote_currency";

    let mut start = 0;
    while start < PROD_ATTR_SIZE {
        let mut length = pyth_product.attr[start] as usize;
        start += 1;

        if length == LEN {
            let mut end = start + length;
            if end > PROD_ATTR_SIZE {
                msg!("Pyth product attribute key length too long");
                return Err(StablePoolError::InvalidOracleConfig.into());
            }

            let key = &pyth_product.attr[start..end];
            if key == KEY {
                start += length;
                length = pyth_product.attr[start] as usize;
                start += 1;

                end = start + length;
                if length > 32 || end > PROD_ATTR_SIZE {
                    msg!("Pyth product quote currency value too long");
                    return Err(StablePoolError::InvalidOracleConfig.into());
                }

                let mut value = [0u8; 32];
                value[0..length].copy_from_slice(&pyth_product.attr[start..end]);
                return Ok(value);
            }
        }

        start += length;
        start += 1 + pyth_product.attr[start] as usize;
    }

    msg!("Pyth product quote currency not found");
    Err(StablePoolError::InvalidOracleConfig.into())
}

pub fn get_pyth_price(pyth_price_info: &AccountInfo, clock: &Clock) -> Result<PreciseNumber> {
    const STALE_AFTER_SLOTS_ELAPSED: u64 = 5;

    let pyth_price_data = pyth_price_info.try_borrow_data()?;
    let pyth_price = load::<Price>(&pyth_price_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    if pyth_price.ptype != PriceType::Price {
        msg!("Oracle price type is invalid");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }

    let slots_elapsed = clock
        .slot
        .checked_sub(pyth_price.valid_slot)
        .ok_or(StablePoolError::MathOverflow)?;
    if slots_elapsed >= STALE_AFTER_SLOTS_ELAPSED {
        msg!("Oracle price is stale");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }

    let price: u64 = pyth_price.agg.price.try_into().map_err(|_| {
        msg!("Oracle price cannot be negative");
        StablePoolError::InvalidOracleConfig
    })?;

    let market_price = if pyth_price.expo >= 0 {
        let exponent = pyth_price
            .expo
            .try_into()
            .map_err(|_| StablePoolError::MathOverflow)?;
        let zeros = 10u64
            .checked_pow(exponent)
            .ok_or(StablePoolError::MathOverflow)?;
        PreciseNumber::new(price as u128)
            .ok_or(StablePoolError::MathOverflow)?
            .checked_mul(&PreciseNumber::new(zeros as u128).ok_or(StablePoolError::MathOverflow)?).ok_or(StablePoolError::MathOverflow)?
    } else {
        let exponent = pyth_price
            .expo
            .checked_abs()
            .ok_or(StablePoolError::MathOverflow)?
            .try_into()
            .map_err(|_| StablePoolError::MathOverflow)?;
        let decimals = 10u64
            .checked_pow(exponent)
            .ok_or(StablePoolError::MathOverflow)?;
        PreciseNumber::new(price as u128)
            .ok_or(StablePoolError::MathOverflow)?
            .checked_div(&PreciseNumber::new(decimals as u128).ok_or(StablePoolError::MathOverflow)?).ok_or(StablePoolError::MathOverflow)?
    };

    Ok(market_price)
}

pub fn get_market_price(
    oracle_program_id:Pubkey,
    pyth_product_info:&AccountInfo,
    pyth_price_info:&AccountInfo,
    clock:&Clock
)->Result<u128>{
    // get market price
    if &oracle_program_id != pyth_product_info.owner {
        msg!("Pyth product account provided is not owned by the lending market oracle program");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }
    if &oracle_program_id != pyth_price_info.owner {
        msg!("Pyth price account provided is not owned by the lending market oracle program");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }

    let pyth_product_data = pyth_product_info.try_borrow_data()?;
    let pyth_product = load::<Product>(&pyth_product_data)
        .map_err(|_| ProgramError::InvalidAccountData)?;
    if pyth_product.magic != MAGIC {
        msg!("Pyth product account provided is not a valid Pyth account");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }
    if pyth_product.ver != VERSION_2 {
        msg!("Pyth product account provided has a different version than expected");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }
    if pyth_product.atype != AccountType::Product as u32 {
        msg!("Pyth product account provided is not a valid Pyth product account");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }

    let pyth_price_pubkey_bytes: &[u8; 32] = pyth_price_info
        .key
        .as_ref()
        .try_into()
        .map_err(|_| StablePoolError::InvalidAccountInput)?;
    if &pyth_product.px_acc.val != pyth_price_pubkey_bytes {
        msg!("Pyth product price account does not match the Pyth price provided");
        return Err(StablePoolError::InvalidOracleConfig.into());
    }

    let _quote_currency = get_pyth_product_quote_currency(pyth_product)?;
    // if quote_currency != _quote_currency {
    //     msg!("Lending market quote currency does not match the oracle quote currency");
    //     return Err(StablePoolError::InvalidOracleConfig.into());
    // }

    let market_price = get_pyth_price(pyth_price_info, clock)?;
    
    Ok(market_price.to_imprecise().ok_or(StablePoolError::MathOverflow)?)
}


pub fn assert_debt_allowed(locked_coll_balance: u64, user_debt: u64, amount: u64, market_price: u64)-> ProgramResult{
    
    let debt_limit = precise_number_u64(market_price)
        .checked_mul(&precise_number_u64(locked_coll_balance)).ok_or(StablePoolError::PreciseError.into())?
        .checked_mul(&precise_number_128(DECIMAL_PRECISION)).ok_or(StablePoolError::PreciseError.into())?
        .checked_div(&precise_number_128(MCR)).ok_or(StablePoolError::PreciseError.into())?.to_imprecise().ok_or(StablePoolError::PreciseError.into())?;

    if debt_limit < (user_debt + amount) as u128 {
        return Err(StablePoolError::NotAllowed.into())
    }
    Ok(())
}

pub fn precise_number_u64(num: u64)->&PreciseNumber{
    &PreciseNumber::new(num as u128);
}

pub fn precise_number_u128(num: u128)->&PreciseNumber{
    &PreciseNumber::new(num);
}


pub fn min(a: u128, b: u128)-> u128{
    if a < b {a} else {b}
}
pub fn max(a: u128, b: u128)-> u128{
    if a >= b {a} else {b}
}

/* 
* Multiply two decimal numbers and use normal rounding rules:
* -round product up if 19'th mantissa digit >= 5
* -round product down if 19'th mantissa digit < 5
*
* Used only inside the exponentiation, _decPow().
*/
pub fn dec_mul(x : u128, y :u128)-> u128{
    let prod_xy = x * y;
    let dec_prod = (prod_xy + DECIMAL_PRECISION / 2) / DECIMAL_PRECISION;
    return dec_prod;
}

/* 
* _decPow: Exponentiation function for 18-digit decimal base, and integer exponent n.
* 
* Uses the efficient "exponentiation by squaring" algorithm. O(log(n)) complexity. 
* 
* Called by two functions that represent time in units of minutes:
* 1) TroveManager._calcDecayedBaseRate
* 2) CommunityIssuance._getCumulativeIssuanceFraction 
* 
* The exponent is capped to avoid reverting due to overflow. The cap 525600000 equals
* "minutes in 1000 years": 60 * 24 * 365 * 1000
* 
* If a period of > 1000 years is ever used as an exponent in either of the above functions, the result will be
* negligibly different from just passing the cap, since: 
*
* In function 1), the decayed base rate will be 0 for 1000 years or > 1000 years
* In function 2), the difference in tokens issued at 1000 years and any time > 1000 years, will be negligible
*/
pub fn dec_pow(base:u128, minutes:u128)->u128{
    let mut _minutes = minutes;
    let mut _base = base;
    if _minutes > 525600000 {
        // cap to avoid overflow
        _minutes = 525600000;
    }
    if _minutes == 0 {
        return DECIMAL_PRECISION;
    }
    let mut y = DECIMAL_PRECISION;
    let mut x = _base;
    let mut n = _minutes;

    // Exponentiation-by-squaring
    while n > 1 {
        if n % 2 == 0 {
            x = dec_mul(x, x);
            n = n / 2;
        }
        else { // if (n % 2 != 0)
            y = dec_mul(x, y);
            x = dec_mul(x, x);
            n = (n - 1) / 2;
        }
    }

    return dec_mul(x, y);
}

pub fn get_absolute_difference(a: u128, b: u128)->u128{
    if a >= b {a - b} else {b - a}
}

pub fn compute_nominal_cr(coll: u128, debt: u128)->u128{
    if debt > 0 {
        coll * NICR_PRECISION / debt
    }
    else {
        MAX
    }
}
pub fn compute_cr(coll: u128, debt: u128, price: u128)->u128{
    if debt > 0 {
        let new_coll_ratio = coll * price / debt;
        return new_coll_ratio;
    }
    // Return the maximal value for uint256 if the Trove has a debt of 0. Represents "infinite" CR.
    else {// if (_debt == 0)
        return MAX;
    }
}

pub fn get_total_from_batch_liquidate_normal_mode(
    trove_manager_data:&mut TroveManager,
    active_pool:&mut ActivePool,
    default_pool:&mut DefaultPool,
    price:u128,
    solusd_in_stab_pool: u128,
    borrower_address:&Pubkey,
    borrower_trove:&mut Trove,
    reward_snapshot:&mut RewardSnapshot
)->LiquidationTotals{
    let mut vars = LocalVariablesLiquidationSequence::new();
    let mut single_liquidation = LiquidationValues::new();

    vars.remaining_solusd_in_stab_pool = solusd_in_stab_pool;
    
    vars.user = Option::from(*borrower_address);

    let mut totals = LiquidationTotals::new();

    if borrower_trove.is_active() {
        vars.icr = get_current_icr(trove_manager_data, borrower_trove, reward_snapshot, price);
        
        if vars.icr < MCR {
            single_liquidation = liquidate_normal_mode(
                trove_manager_data, 
                active_pool, 
                default_pool, 
                borrower_trove, 
                reward_snapshot, 
                vars.remaining_solusd_in_stab_pool, 
                );
        
            vars.remaining_solusd_in_stab_pool -= single_liquidation.debt_to_offset;

            // Add liquidation values to their respective running totals
            add_liquidation_values_to_totals(&mut totals, &single_liquidation);
        }
    }

    totals
}
pub fn redistribute_debt_and_coll(
    trove_manager:&mut TroveManager,
    active_pool:&mut ActivePool,
    default_pool:&mut DefaultPool,
    debt:u128,
    coll:u128
){
    if debt == 0 {
        return;
    }

    /*
    * Add distributed coll and debt rewards-per-unit-staked to the running totals. Division uses a "feedback"
    * error correction, to keep the cumulative error low in the running totals l_sol and l_solusd_debt:
    *
    * 1) Form numerators which compensate for the floor division errors that occurred the last time this
    * function was called.
    * 2) Calculate "per-unit-staked" ratios.
    * 3) Multiply each ratio back by its denominator, to reveal the current floor division error.
    * 4) Store these errors for use in the next correction when this function is called.
    * 5) Note: static analysis tools complain about this "division before multiplication", however, it is intended.
    */
    let sol_numerator = coll * DECIMAL_PRECISION + trove_manager.last_sol_error_redistribution;
    let solusd_debt_numerator = debt * DECIMAL_PRECISION + trove_manager.last_solusd_debt_error_redistribution;

    // Get the per-unit-staked terms
    let sol_reward_per_unit_staked = sol_numerator / trove_manager.total_stakes;
    let solusd_debt_reward_per_unit_staked = solusd_debt_numerator / trove_manager.total_stakes;

    trove_manager.last_sol_error_redistribution = sol_numerator - sol_reward_per_unit_staked * trove_manager.total_stakes;
    trove_manager.last_solusd_debt_error_redistribution = solusd_debt_numerator - solusd_debt_reward_per_unit_staked * trove_manager.total_stakes;

    // Add per-unit-staked terms to the running totals
    trove_manager.l_sol += sol_reward_per_unit_staked;
    trove_manager.l_solusd_debt += solusd_debt_reward_per_unit_staked;

    active_pool.decrease_solusd_debt(debt);
    default_pool.increase_solusd_debt(debt);

    active_pool.sol -= coll;
    default_pool.sol += coll;

}
pub fn get_total_from_batch_liquidate_recovery_mode(
    trove_manager_data:&mut TroveManager,
    active_pool:&mut ActivePool,
    default_pool:&mut DefaultPool,
    price:u128,
    solusd_in_stab_pool: u128,
    borrower_address:&Pubkey,
    borrower_trove:&mut Trove,
    reward_snapshot:&mut RewardSnapshot
)->LiquidationTotals{
    let mut vars = LocalVariablesLiquidationSequence::new();
    let mut single_liquidation = LiquidationValues::new();

    vars.remaining_solusd_in_stab_pool = solusd_in_stab_pool;
    vars.back_to_normal_mode = 0;
    vars.entire_system_debt = active_pool.solusd_debt + default_pool.solusd_debt;
    vars.entire_system_coll = active_pool.sol + default_pool.sol;
    vars.user = Option::from(*borrower_address);

    let mut totals = LiquidationTotals::new();

    if borrower_trove.is_active() {
        vars.icr = get_current_icr(trove_manager_data, borrower_trove, reward_snapshot, price);
        
        if vars.back_to_normal_mode == 0 {
            if vars.icr < MCR || vars.remaining_solusd_in_stab_pool > 0 {
                let tcr = compute_cr(vars.entire_system_coll, vars.entire_system_debt, price);
                single_liquidation = liquidate_recovery_mode(
                    trove_manager_data, 
                    active_pool, 
                    default_pool, 
                    borrower_trove, 
                    reward_snapshot, 
                    vars.icr, 
                    vars.remaining_solusd_in_stab_pool, 
                    tcr, 
                    price);
                
                    // update aggregate trackers
                    vars.remaining_solusd_in_stab_pool -= single_liquidation.debt_to_offset;
                    vars.entire_system_debt -= single_liquidation.debt_to_offset;
                    vars.entire_system_coll -= vars.entire_system_coll - single_liquidation.coll_to_send_to_sp - single_liquidation.coll_gas_compensation - single_liquidation.coll_surplus;

                    // Add liquidation values to their respective running totals
                    
                    add_liquidation_values_to_totals(&mut totals, &single_liquidation);
                    vars.back_to_normal_mode = check_potential_not_recovery_mode(trove_manager_data, vars.entire_system_coll, vars.entire_system_debt, price);
                    
            }
            else if vars.back_to_normal_mode == 1 && vars.icr < MCR {
                single_liquidation = liquidate_normal_mode(trove_manager_data, active_pool, default_pool, borrower_trove, reward_snapshot, vars.remaining_solusd_in_stab_pool);
                vars.remaining_solusd_in_stab_pool -= single_liquidation.debt_to_offset;

                // Add liquidation values to their respective running totals
                add_liquidation_values_to_totals(&mut totals, &single_liquidation);

            }
        }
    }

    totals
}

pub fn check_potential_not_recovery_mode(trove_manager:&TroveManager, entire_system_coll:u128, entire_system_debt:u128, price:u128)->u8{
    let tcr = compute_cr(entire_system_coll, entire_system_debt, price);
    return if tcr < CCR {0} else {1};
}
pub fn add_liquidation_values_to_totals(totals:&mut LiquidationTotals, single_liquidation:&LiquidationValues){
    totals.total_coll_gas_compensation += single_liquidation.coll_gas_compensation;
    totals.total_solusd_gas_compensation += single_liquidation.solusd_gas_compensation;
    totals.total_debt_in_sequence += single_liquidation.entire_trove_debt;
    totals.total_coll_in_sequence += single_liquidation.entire_trove_coll;
    totals.total_debt_to_offset += single_liquidation.debt_to_offset;
    totals.total_coll_to_send_to_sp += single_liquidation.coll_to_send_to_sp;
    totals.total_debt_to_redistribute += single_liquidation.debt_to_redistribute;
    totals.total_coll_to_redistribute += single_liquidation.coll_to_redistribute;
    totals.total_coll_surplus += single_liquidation.coll_surplus;
}