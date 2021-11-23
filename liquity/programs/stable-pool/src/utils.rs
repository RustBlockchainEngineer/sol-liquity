use anchor_lang::prelude::*;
use crate::{
    pyth::*,
    error::*,
    constant::*,
};
use std::convert::TryInto;
use std::convert::TryFrom;
use spl_math::{precise_number::PreciseNumber};

pub fn get_market_price()->u64 {
    253
}

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

pub fn _get_market_price(
    oracle_program_id:Pubkey,
    pyth_product_info:&AccountInfo,
    pyth_price_info:&AccountInfo,
    clock:&Clock
)->Result<u64>{
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
    
    Ok(u64::try_from(market_price.to_imprecise().ok_or(StablePoolError::MathOverflow)?).unwrap_or(0))
}


pub fn assert_debt_allowed(locked_coll_balance: u64, user_debt: u64, amount: u64, market_price: u64)-> ProgramResult{
    
    let debt_limit = precise_number_u64(market_price)
        .checked_mul(&precise_number_u64(locked_coll_balance)).ok_or(StablePoolError::PreciseError.into())?
        .checked_mul(&precise_number_128(PERCENT_DIVIDER)).ok_or(StablePoolError::PreciseError.into())?
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


pub fn liquidate_normal_mode(
    trove_manager: &mut TroveManager,
    active_pool:&mut ActivePool,
    default_pool:&mut DefaultPool,
    borrower_trove:&mut Trove,
    reward_snapshots:&mut RewardSnapshot,
    _solusd_in_stab_pool:u128,
)->LiquidationValues{
    let mut vars = LocalVariablesInnerSingleLiquidateFunction::new();
    let mut single_liquidation = LiquidationValues::new();

    //if (TroveOwners.length <= 1) {return singleLiquidation;} // don't liquidate if last trove
    let (entire_trove_debt, entire_trove_coll, pending_debt_reward, pending_coll_reward) = get_entire_debt_and_coll(trove_manager, borrower_trove, reward_snapshots);
    single_liquidation.entire_trove_debt = entire_trove_debt;
    single_liquidation.entire_trove_coll = entire_trove_coll;
    vars.pending_debt_reward = pending_debt_reward;
    vars.pending_coll_reward = pending_coll_reward;

    move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
    remove_stake(trove_manager,borrower_trove);

    single_liquidation.coll_gas_compensation = get_coll_gas_compensation(single_liquidation.entire_trove_coll);
    single_liquidation.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;
    let coll_to_liquidate = single_liquidation.entire_trove_coll - single_liquidation.coll_gas_compensation;

    let (_debt_to_offset, _coll_to_send_to_sp, _debt_to_redistribute, _coll_to_liquidate) = get_offset_and_redistribution_vals(single_liquidation.entire_trove_debt, coll_to_liquidate, _solusd_in_stab_pool);
    single_liquidation.debt_to_offset = _debt_to_offset;
    single_liquidation.coll_to_send_to_sp = _coll_to_send_to_sp;
    single_liquidation.debt_to_redistribute = _debt_to_redistribute;
    single_liquidation.coll_to_redistribute = _coll_to_liquidate;

    //_closeTrove(_borrower, Status.closedByLiquidation); --in frontend
    return single_liquidation;

}
pub fn liquidate_recovery_mode(
    trove_manager: &mut TroveManager,
    active_pool:&mut ActivePool,
    default_pool:&mut DefaultPool,
    borrower_trove:&mut Trove,
    reward_snapshots:&mut RewardSnapshot,
    _icr:u128,
    _solusd_in_stab_pool:u128,
    _tcr:u128,
    _price:u128

)->LiquidationValues{
    let mut vars = LocalVariablesInnerSingleLiquidateFunction::new();
    let mut single_liquidation = LiquidationValues::new();

    //if (TroveOwners.length <= 1) {return singleLiquidation;} // don't liquidate if last trove
    let (entire_trove_debt, entire_trove_coll, pending_debt_reward, pending_coll_reward) = get_entire_debt_and_coll(trove_manager, borrower_trove, reward_snapshots);
    single_liquidation.entire_trove_debt = entire_trove_debt;
    single_liquidation.entire_trove_coll = entire_trove_coll;
    vars.pending_debt_reward = pending_debt_reward;
    vars.pending_coll_reward = pending_coll_reward;

    single_liquidation.coll_gas_compensation = get_coll_gas_compensation(single_liquidation.entire_trove_coll);
    single_liquidation.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;
    vars.coll_to_liquidate = single_liquidation.entire_trove_coll - single_liquidation.coll_gas_compensation;

    // If ICR <= 100%, purely redistribute the Trove across all active Troves
    if _icr <= _100PCT {
        move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
        remove_stake(trove_manager,borrower_trove);

        single_liquidation.debt_to_offset = 0;
        single_liquidation.coll_to_send_to_sp = 0;
        single_liquidation.debt_to_redistribute = single_liquidation.entire_trove_debt;
        single_liquidation.coll_to_redistribute = vars.coll_to_liquidate;

        close_trove(borrower_trove, reward_snapshots);
    }
    else if (_icr > _100PCT) && (_icr < MCR) {
        move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
        remove_stake(trove_manager,borrower_trove);

        let (_debt_to_offset, _coll_to_send_to_sp, _debt_to_redistribute, _coll_to_liquidate) = get_offset_and_redistribution_vals(single_liquidation.entire_trove_debt, vars.coll_to_liquidate, _solusd_in_stab_pool);
        //_closeTrove(_borrower, Status.closedByLiquidation); -- in frontend
    }
    /*
    * If 110% <= ICR < current TCR (accounting for the preceding liquidations in the current sequence)
    * and there is SOLUSD in the Stability Pool, only offset, with no redistribution,
    * but at a capped rate of 1.1 and only if the whole debt can be liquidated.
    * The remainder due to the capped rate will be claimable as collateral surplus.
    */
    else if (_icr >= MCR) && (_icr < _tcr) && (single_liquidation.entire_trove_debt <= _solusd_in_stab_pool) {
        move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
        //assert(_LUSDInStabPool != 0);
        remove_stake(trove_manager,borrower_trove);
        get_capped_offset_vals(&mut single_liquidation, _price);

        //_closeTrove(_borrower, Status.closedByLiquidation); -- in frontend
        if single_liquidation.coll_surplus > 0 {
            //collSurplusPool.accountSurplus(_borrower, singleLiquidation.collSurplus); --in frontend
        }
    }
    else{
        let zero_vals = LiquidationValues::new();
        return zero_vals;
    }
    return single_liquidation;

}