pub const GLOBAL_STATE_TAG:&[u8] = b"golbal-state-seed";
pub const TOKEN_VAULT_TAG:&[u8] = b"token-vault-seed";
pub const USER_TROVE_TAG:&[u8] = b"user-trove-seed";
pub const SOLUSD_MINT_TAG:&[u8] = b"solusd-mint";
pub const TOKEN_VAULT_POOL_TAG:&[u8] = b"token-vault-pool";
pub const STABILITY_POOL_TAG:&[u8] = b"stability-pool";
pub const SP_USER_INFO:&[u8] = b"sp-user-info";

pub const SOLUSD_DECIMALS: u8 = 6;

// Minimum collateral ratio for individual troves
pub const MCR: u64 = 1_100_000_000; // 110%

pub const _100PCT: u64 = 1_000_000_000; // 110%

// Critical system collateral ratio. If the system's total collateral ratio (TCR) falls below the CCR, Recovery Mode is triggered.
pub const CCR: u64 = 1_500_000_000; // 150%

pub const DECIMAL_PRECISION:u64 = 1_000_000_000;
pub const NICR_PRECISION:u64 = 100_000_000;