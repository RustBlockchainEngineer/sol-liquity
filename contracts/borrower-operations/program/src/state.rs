//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
};

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesAdjustTrove {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub price:u64,
    pub coll_change:u64,
    pub net_debt_change:u64,
    pub is_coll_increase:u8,
    pub debt:u64,
    pub coll:u64,
    pub old_icr:u64,
    pub new_icr:u64,
    pub new_tcr:u64,
    pub solusd_fee:u64,
    pub new_debt:u64,
    pub new_coll:u64,
    pub stake:u64,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesOpenTrove {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub price:u64,
    pub solusd_fee:u64,
    pub net_debt:u64,
    pub composit_debt:u64,
    pub icr:u64,
    pub nicr:u64,
    pub stake:u64,
    pub array_index:u64,
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ContractsCache {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// trove manager pubkey
    pub trove_manager_pubkey:Pubkey,

    /// active pool pubkey
    pub active_pool_pubkey:Pubkey,

    /// solUSD token pubkey
    pub solusd_token_pubkey:Pubkey,

}
