use anchor_lang::prelude::*;
use crate::{
    pyth::*,
    error::*,
    constant::*,
};
use std::u64::MAX;
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
    
    Ok(market_price.to_u64()?)
}


pub fn assert_debt_allowed(locked_coll_balance: u64, user_debt: u64, amount: u64, market_price: u64, coll_decimals: u8, usd_decimals: u8)-> ProgramResult{
    msg!("market sol price = {}", market_price);
    let debt_limit = market_price
        .checked_mul(locked_coll_balance).unwrap()
        .checked_mul(DECIMAL_PRECISION).unwrap()
        .checked_div(MCR)
        .checked_mul(pow(10, usd_decimals))
        .checked_div(pow(10, coll_decimals))
        .unwrap();
    msg!("debt_limit = {}", debt_limit);
    msg!("user_debt + amount = {}", user_debt + amount);
    if debt_limit < (user_debt + amount) as u64 {
        return Err(StablePoolError::NotAllowed.into())
    }
    Ok(())
}

pub fn min(a: u64, b: u64)-> u64{
    if a < b {a} else {b}
}
pub fn max(a: u64, b: u64)-> u64{
    if a >= b {a} else {b}
}

/* 
* Multiply two decimal numbers and use normal rounding rules:
* -round product up if 19'th mantissa digit >= 5
* -round product down if 19'th mantissa digit < 5
*
* Used only inside the exponentiation, _decPow().
*/
pub fn dec_mul(x : u64, y :u64)-> u64{
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
pub fn dec_pow(base:u64, minutes:u64)->u64{
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

pub fn get_absolute_difference(a: u64, b: u64)->u64{
    if a >= b {a - b} else {b - a}
}

pub fn compute_nominal_cr(coll: u64, debt: u64)->u64{
    if debt > 0 {
        coll * NICR_PRECISION / debt
    }
    else {
        MAX
    }
}
pub fn compute_cr(coll: u64, debt: u64, price: u64)->u64{
    if debt > 0 {
        let new_coll_ratio = coll * price / debt;
        return new_coll_ratio;
    }
    // Return the maximal value for uint256 if the Trove has a debt of 0. Represents "infinite" CR.
    else {// if (_debt == 0)
        return MAX;
    }
}



pub trait ToPrecise {
    fn to_precise(&self)-> Result<PreciseNumber>;
}

pub trait ToU64U128 {
    fn to_u64(&self) -> Result<u64>;
    fn to_u128(&self) -> Result<u128>;
}

impl ToPrecise for u64 {
    fn to_precise(&self)-> Result<PreciseNumber> {
        Ok(PreciseNumber::new(*self as u128).ok_or(StablePoolError::PreciseError)?)
    }
}

impl ToPrecise for u128 {
    fn to_precise(&self)-> Result<PreciseNumber> {
        Ok(PreciseNumber::new(*self).ok_or(StablePoolError::PreciseError)?)
    }
}
impl ToU64U128 for PreciseNumber {
    fn to_u64(&self) -> Result<u64> {
        Ok(u64::try_from(self.to_imprecise().ok_or(StablePoolError::PreciseError)?).unwrap_or(0))
    }
    fn to_u128(&self) -> Result<u128> {
        Ok(self.to_imprecise().ok_or(StablePoolError::PreciseError)?)
    }
}


pub fn pow(x:u64, y:u64)->u64{
    let mut result = x;
    let mut index = y;
    while y > 1 {
        result *= x;
        y -= 1;
    }
    return result;
}