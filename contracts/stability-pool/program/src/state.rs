//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
};
use crate::{
    liquitiy_math::{
        DECIMAL_PRECISION,
    },
    constant::{
        SCALE_FACTOR
    }
};
/// Stability Pool struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StabilityPool {
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
    pub total_sol_usd_deposits: u64,

    // Error tracker for the error correction in the SOLID issuance calculation
    pub last_solid_error:u64,

    // Error trackers for the error correction in the offset calculation
    pub last_sol_error_offset:u64,

    pub last_solusd_loss_error_offset:u64,

    pub p:u64,

    pub current_scale:u128,

    pub current_epoch:u128,
}
impl StabilityPool{
    
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct FrontEnd {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// kickback rate
    pub kickback_rate:u64,

    /// flag for registered frontend
    pub registered: u8,
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Deposit {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// initial value
    pub initial_value:u64,

    /// tag public key of this frontend
    pub front_end_tag: Pubkey,
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Snapshots {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// S
    pub s:u64,

    /// P
    pub p:u64,

    /// G
    pub g:u64,

    /// scale
    pub scale: u128,

    /// epoch
    pub epoch: u128,
}
