use anchor_lang::prelude::*;
use crate::{
    pyth::*,
    error::*,
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
    
    let debt_limit = market_price * locked_coll_balance;
    if debt_limit < user_debt + amount {
        return Err(StablePoolError::NotAllowed.into())
    }
    Ok(())
}



pub fn trigger_borrowing_fee<'a>(
    borrower_operation_info:&AccountInfo<'a>,
    authority_info:&AccountInfo<'a>,
    trove_manager_info:&AccountInfo<'a>,
    solusd_token_info: &AccountInfo<'a>,
    token_program_info: &AccountInfo<'a>,
    solid_staking_info: &AccountInfo<'a>,
    solid_staking_token_pool_info: &AccountInfo<'a>,
    nonce:u8,
    solusd_amount: u128,
    max_fee_percentage: u128
)->Result<u128, ProgramError> {
    let trove_manager = TroveManager::try_from_slice(&trove_manager_info.data.borrow_mut())?;
    decay_base_rate_from_borrowing(&trove_manager, borrower_operation_info)?;
    let solusd_fee:u128 = get_borrowing_fee(&trove_manager, solusd_amount);
    if !(solusd_fee * DECIMAL_PRECISION / solusd_amount <= max_fee_percentage){
        return Err(LiquityError::FeeExceeded.into());
    }

    let mut solid_staking = SOLIDStaking::try_from_slice(&solid_staking_info.data.borrow_mut())?;
    solid_staking.increase_f_solusd(solusd_fee);

    token_mint_to(            
        borrower_operation_info.key,
        token_program_info.clone(),
        solusd_token_info.clone(),
        solid_staking_token_pool_info.clone(),
        authority_info.clone(),
        nonce,
        solusd_fee as u64,
    )?;

    trove_manager.serialize(&mut &mut trove_manager_info.data.borrow_mut()[..])?;
    solid_staking.serialize(&mut &mut solid_staking_info.data.borrow_mut()[..])?;

    Ok(solusd_fee)
}