//! All instruction types
//! These instructions represent a function what will be processed by this program

// this allows many arguments for the function parameters
#![allow(clippy::too_many_arguments)]

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        // sysvar
    },
};

/// Instructions supported by the Trove Manager program
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum TroveManagerInstruction {
    Initialize {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,
    },
    ApplyPendingRewards,
    Liquidate,
    RedeemCollateral{
        #[allow(dead_code)]
        solusd_amount: u128,

        #[allow(dead_code)]
        partial_redemption_hint_nicr: u128,

        #[allow(dead_code)]
        max_iterations: u128,

        #[allow(dead_code)]
        max_fee_percentage: u128,

        #[allow(dead_code)]
        total_sol_drawn: u128,

        #[allow(dead_code)]
        total_solusd_to_redeem: u128,

        #[allow(dead_code)]
        nonce: u8,
    },
    LiquidateTroves{
        number:u128,
        total_coll_in_sequence:u128,
        total_debt_in_sequence:u128,
        total_coll_gas_compensation:u128,
        total_solusd_gas_compensation:u128,
        total_debt_to_offset:u128,
        total_coll_to_send_to_sp:u128,
        total_debt_to_redistribute:u128,
        total_coll_to_redistribute:u128,
        total_coll_surplus:u128,
    },
    

}
