/// liquity math functions

use crate::{
    constant::{
        DECIMAL_PRECISION,
        NICR_PRECISION,
    }
};
use std::u128::MAX;

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