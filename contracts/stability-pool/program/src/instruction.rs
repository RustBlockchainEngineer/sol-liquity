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

/// Instructions supported by the Stability Pool program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum StabilityPoolInstruction {
    ///   Initializes a new Stability Pool.
    ///   These represent the parameters that will be included from client side
    ///   [w] - writable, [s] - signer
    /// 
    ///   0. `[w]` New Stability Pool account to create.
    ///   1. `[]` authority to initialize this pool account
    ///   2. `[]` solUSD pool token account account
    ///   3. `[]` Token program id
    ///   4. `[]` nonce
    ///   5. `[]` StabilityPool program id
    Initialize {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,
    },

    /// Provide to stability pool
    ///
    /// - Triggers a SOLID issuance, based on time passed since the last issuance. The SOLID issuance is shared between *all* depositors and front ends
    /// - Tags the deposit with the provided front end tag param, if it's a new deposit
    /// - Sends depositor's accumulated gains (SOLID, SOL) to depositor
    /// - Sends the tagged front end's accumulated SOLID gains to the tagged front end
    /// - Increases deposit and tagged front end's stake, and takes new snapshots for each.
    /// 
    ///   0. `[w]` StabilityPool account to provide to.
    ///   1. `[]` authority of this pool account
    ///   2. `[]` solUSD pool token account account
    ///   3. `[]` solUSD user token account account
    ///   4. `[]` user transfer authority
    ///   5. `[]` user deposit data account
    ///   6. `[]` frontend account
    ///   7. `[]` Token program id
    ///   8. `[]` StabilityPool program id
    ///   9. `[]` amount
    ProvideToSP(u64),

    /// Withdraw from stability pool
    ///
    /// - Triggers a SOLID issuance, based on time passed since the last issuance. The SOLID issuance is shared between *all* depositors and front ends
    /// - Tags the deposit with the provided front end tag param, if it's a new deposit
    /// - Sends depositor's accumulated gains (SOLID, SOL) to depositor
    /// - Sends the tagged front end's accumulated SOLID gains to the tagged front end
    /// - Increases deposit and tagged front end's stake, and takes new snapshots for each.
    /// 
    ///   0. `[w]` StabilityPool account to withdraw from.
    ///   1. `[]` authority of this pool account
    ///   2. `[]` solUSD pool token account account
    ///   3. `[]` solUSD user token account account
    ///   4. `[]` user transfer authority
    ///   5. `[]` user deposit data account
    ///   6. `[]` Token program id
    ///   7. `[]` StabilityPool program id
    ///   8. `[]` amount
    WithdrawFromSP(u64),

    /* withdrawSOLGainToTrove:
    * - Triggers a SOLID issuance, based on time passed since the last issuance. The SOLID issuance is shared between *all* depositors and front ends
    * - Sends all depositor's SOLID gain to  depositor
    * - Sends all tagged front end's SOLID gain to the tagged front end
    * - Transfers the depositor's entire ETH gain from the Stability Pool to the caller's trove
    * - Leaves their compounded deposit in the Stability Pool
    * - Updates snapshots for deposit and tagged front end stake */
    WithdrawSOLGainToTrove,
    
    RegisterFrontEnd(u64),

}
