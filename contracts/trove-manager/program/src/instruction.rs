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
        solusd_amount: u64,

        #[allow(dead_code)]
        partial_redemption_hint_nicr: u64,

        #[allow(dead_code)]
        max_iterations: u64,

        #[allow(dead_code)]
        max_fee_percentage: u64,
    },
    LiquidateTroves(u64),
    

}
