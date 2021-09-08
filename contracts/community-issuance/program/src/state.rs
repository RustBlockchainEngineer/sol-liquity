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
pub struct CommunityIssuance {
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// spl-token program pubkey
    pub token_program_pubkey: Pubkey,

    /// SOLID token account pubkey
    pub solid_token_pubkey: Pubkey,

    /// stability pool account pubkey
    pub stability_pool_pubkey: Pubkey,

    pub total_solid_issued: u64,

    pub deployment_time:u64,

}
