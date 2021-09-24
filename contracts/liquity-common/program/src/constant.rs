pub const DECIMAL_PRECISION:u128 = 1_000_000_000_000_000_000;
// Minimum collateral ratio for individual troves
pub const MCR: u128 = 1100000000000000000; // 110%

// Critical system collateral ratio. If the system's total collateral ratio (TCR) falls below the CCR, Recovery Mode is triggered.
pub const CCR: u128 = 1500000000000000000; // 150%
pub const PERCENT_DIVISOR:u128 = 200; // dividing by 200 yields 0.5%

// Amount of solusd to be locked in gas pool on opening troves
pub const SOLUSD_GAS_COMPENSATION:u128 = 200_000_000_000_000_000_000;

pub const _100PCT: u128 = 1000000000000000000; // 1e18 == 100%

pub const MINUTE_DECAY_FACTOR:u128 = 999037758833783000;
pub const REDEMPTION_FEE_FLOOR:u128 = DECIMAL_PRECISION / 1000 * 5; // 0.5%
pub const MAX_BORROWING_FEE:u128 = DECIMAL_PRECISION / 100 * 5; // 5%
pub const SECONDS_IN_ONE_MINUTE:u128 = 60;

/* Precision for Nominal ICR (independent of price). Rationale for the value:
*
* - Making it “too high” could lead to overflows.
* - Making it “too low” could lead to an ICR equal to zero, due to truncation from Solidity floor division. 
*
* This value of 1e20 is chosen for safety: the NICR will only overflow for numerator > ~1e39 ETH,
* and will only truncate to 0 if the denominator is at least 1e20 times greater than the numerator.
*
*/
pub const NICR_PRECISION:u128 = 100_000_000_000_000_000_000;

/*
* BETA: 18 digit decimal. Parameter by which to divide the redeemed fraction, in order to calc the new base rate from a redemption.
* Corresponds to (1 / ALPHA) in the white paper.
*/
pub const BETA:u128 = 2;

pub const SCALE_FACTOR:u128 = 1_000_000_000;

// Amount of SOLUSD to be locked in gas pool on opening troves
pub const LUSD_GAS_COMPENSATION: u128 = 200_000_000_000_000_000_000;

// Minimum amount of net LUSD debt a trove must have
//pub const MIN_NET_DEBT: u128 = 1800e18;
pub const MIN_NET_DEBT: u128 = 0;

pub const BORROWING_FEE_FLOOR: u128 = DECIMAL_PRECISION / 1000 * 5; // 0.5%

pub const ZERO_ADDRESS:&str = "00000000000000000000000000000000000000000000";


pub const ISSUANCE_FACTOR:u128 = 999998681227695000;
pub const SOLID_SUPPLY_CAP:u128 = 32_000_000_000_000_000_000_000_000;