pub const DECIMAL_PRECISION:u64 = 1_000_000_000_000_000_000;
// Minimum collateral ratio for individual troves
pub const MCR: u64 = 1100000000000000000; // 110%

// Critical system collateral ratio. If the system's total collateral ratio (TCR) falls below the CCR, Recovery Mode is triggered.
pub const CCR: u64 = 1500000000000000000; // 150%
pub const PERCENT_DIVISOR:u64 = 200; // dividing by 200 yields 0.5%

// Amount of solusd to be locked in gas pool on opening troves
pub const SOLUSD_GAS_COMPENSATION:u128 = 200_000_000_000_000_000_000;

pub const _100PCT: u64 = 1000000000000000000; // 1e18 == 100%

pub const MINUTE_DECAY_FACTOR:u64 = 999037758833783000;
pub const REDEMPTION_FEE_FLOOR:u64 = DECIMAL_PRECISION / 1000 * 5; // 0.5%
pub const MAX_BORROWING_FEE:u64 = DECIMAL_PRECISION / 100 * 5; // 5%