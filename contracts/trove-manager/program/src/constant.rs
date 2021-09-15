pub const DECIMAL_PRECISION:u64 = 1_000_000_000_000_000_000;
// Minimum collateral ratio for individual troves
pub const MCR: u64 = 1100000000000000000; // 110%

// Critical system collateral ratio. If the system's total collateral ratio (TCR) falls below the CCR, Recovery Mode is triggered.
pub const CCR: u64 = 1500000000000000000; // 150%