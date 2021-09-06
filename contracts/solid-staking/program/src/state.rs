//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
};

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
    pub total_staked_amount:u64,

    /// Running sum of SOL fees per-SOLID-staked
    pub f_sol:u64,

    /// Running sum of SOLID fees per-SOLID-staked
    pub f_solusd:u64,

    
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Deposit {
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