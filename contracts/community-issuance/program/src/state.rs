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
    constant::{
        SOLID_SUPPLY_CAP,
        DECIMAL_PRECISION,
        SECONDS_IN_ONE_MINUTE,
        ISSUANCE_FACTOR
    }
};
use std::convert::TryFrom;

/// Community Issuance struct
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
impl CommunityIssuance {
    pub fn issue_solid(&mut self, cur_timestamp:u64) -> u64 {
        let cumulative_issuance_fraction = self.get_cumulative_issuance_fraction(cur_timestamp);
        let latest_total_solid_issued = u64::try_from(SOLID_SUPPLY_CAP * (cumulative_issuance_fraction as u128) / (DECIMAL_PRECISION as u128)).unwrap();
        let issuance = latest_total_solid_issued - self.total_solid_issued;
        self.total_solid_issued = latest_total_solid_issued;
        return issuance;
    }
    pub fn get_cumulative_issuance_fraction(&self, cur_timestamp:u64) -> u64 {
        // Get the time passed since deployment
        let time_passed_in_minutes:u32 = u32::try_from(( cur_timestamp - self.deployment_time ) / SECONDS_IN_ONE_MINUTE).unwrap();

        // f^t
        let power = u64::pow(ISSUANCE_FACTOR,time_passed_in_minutes);

        //  (1 - f^t)
        let cumulative_issuance_fraction = DECIMAL_PRECISION - power;

        if cumulative_issuance_fraction > DECIMAL_PRECISION {
            return 0;
        }

        return cumulative_issuance_fraction;
    }
}
