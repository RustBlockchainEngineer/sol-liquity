use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LiquityState {
    owner: Pubkey,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct TroveManager {
    /// account type
    pub account_type: u8, 

    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// BorrwoerOperations account
    pub borrower_operations_id: Pubkey,

    /// StabilityPool publickey
    pub stability_pool_id: Pubkey,

    /// Gas Pool publickey
    pub gas_pool_id: Pubkey,

    pub coll_surplus_pool_id: Pubkey,

    pub solusd_token_pubkey: Pubkey,

    pub solid_token_pubkey: Pubkey,

    pub solid_staking_pubkey: Pubkey,

    pub token_program_id: Pubkey,

    pub default_pool_id: Pubkey,

    pub active_pool_id: Pubkey,
    pub oracle_program_id: Pubkey,
    pub pyth_product_id: Pubkey,
    pub pyth_price_id: Pubkey,

    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

    pub base_rate:u128,

    pub last_fee_operation_time:u128,
    pub total_stakes:u128,
    pub total_stakes_snapshot:u128,
    pub total_collateral_snapshot:u128,
    pub l_sol:u128,
    pub l_solusd_debt:u128,
    pub last_sol_error_redistribution:u128,
    pub last_solusd_debt_error_redistribution:u128,

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


#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ActivePool {
    pub borrower_operations_address: Pubkey,
    pub trove_manager_address: Pubkey,
    pub stability_pool_address: Pubkey,
    pub default_pool_address: Pubkey,
    pub sol: u128,
    pub solusd_debt: u128,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct DefaultPool {
    pub trove_manager_address: Pubkey,
    pub active_pool_address: Pubkey,
    pub sol: u128,
    pub solusd_debt: u128,
}



#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct UserDeposit {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// deposited amount
    pub deposit_amount:u64,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Snapshot {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// SOL snapshot
    pub f_sol_snapshot:u64,

    /// solUSD snapshot
    pub f_solusd_snapshot:u64,
}


/// SOLID Staking struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct SOLIDStaking {
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// spl-token program pubkey
    pub token_program_pubkey: Pubkey,

    /// SOLID pool token account
    pub solid_pool_token_pubkey: Pubkey,

    /// TroveManager account
    pub trove_manager_id: Pubkey,

    /// BorrwoerOperations account
    pub borrower_operations_id: Pubkey,

    /// ActivePool account
    pub active_pool_id: Pubkey,

    /// total staked SOLID amount
    pub total_staked_amount:u128,

    /// Running sum of SOL fees per-SOLID-staked
    pub f_sol:u128,

    /// Running sum of SOLID fees per-SOLID-staked
    pub f_solusd:u128,
}