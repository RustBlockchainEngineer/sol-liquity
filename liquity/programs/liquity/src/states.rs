use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LiquityState {
    owner: Pubkey,
}


/// BorrowerOperations struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BorrowerOperations {
    pub nonce: u8,
    pub token_program_pubkey: Pubkey,
    pub trove_manager_id: Pubkey,
    pub active_pool_id: Pubkey,
    pub default_pool_id: Pubkey,
    pub stability_pool_id: Pubkey,
    pub gas_pool_id: Pubkey,
    pub coll_surplus_pool_id: Pubkey,
    pub solusd_token_id: Pubkey,
    pub solid_staking_id: Pubkey,

    pub oracle_program_id: Pubkey,
    pub pyth_product_id: Pubkey,
    pub pyth_price_id: Pubkey,
    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

    
}


/// Stability Pool struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StabilityPool {
    /// account type
    pub account_type: u8, 

    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// solUSD token's mint address
    pub token_program_pubkey: Pubkey,

    /// solUSD token's mint address
    pub sol_usd_pool_token_pubkey: Pubkey,

    /// Borrower Operations pubkey
    pub borrower_operations_pubkey: Pubkey,

    /// Borrower Operations pubkey
    pub trove_manager_pubkey: Pubkey,

    pub community_issuance_pubkey: Pubkey,

    /// Tracker for solUSD held in the pool. Changes when users deposit/withdraw, and when Trove debt is offset.
    pub total_sol_usd_deposits: u128,

    // Error tracker for the error correction in the SOLID issuance calculation
    pub last_solid_error:u128,

    // Error trackers for the error correction in the offset calculation
    pub last_sol_error_offset:u128,

    pub last_solusd_loss_error_offset:u128,

    pub p:u128,

    pub current_scale:u128,

    pub current_epoch:u128,

    // deposited sol tracker
    pub sol: u128,

    /// Oracle (Pyth) program id
    pub oracle_program_id: Pubkey,
    
    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

}