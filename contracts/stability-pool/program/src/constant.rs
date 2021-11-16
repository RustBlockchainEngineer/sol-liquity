pub const SCALE_FACTOR:u64 = 1_000_000_000;
pub const DECIMAL_PRECISION:u64 = 1_000_000_000_000_000_000;

pub const ZERO_ADDRESS:&str = "00000000000000000000000000000000000000000000";

pub const _100PCT: u64 = 1000000000000000000; // 1e18 == 100%

// Minimum collateral ratio for individual troves
pub const MCR: u64 = 1100000000000000000; // 110%

// Critical system collateral ratio. If the system's total collateral ratio (TCR) falls below the CCR, Recovery Mode is triggered.
pub const CCR: u64 = 1500000000000000000; // 150%

// Amount of SOLUSD to be locked in gas pool on opening troves
pub const LUSD_GAS_COMPENSATION: u64 = 200e18;

// Minimum amount of net LUSD debt a trove must have
//pub const MIN_NET_DEBT: u64 = 1800e18;
pub const MIN_NET_DEBT: u64 = 0;

pub const PERCENT_DIVISOR: u64 = 200; // dividing by 200 yields 0.5%

pub const BORROWING_FEE_FLOOR: u64 = DECIMAL_PRECISION / 1000 * 5; // 0.5%