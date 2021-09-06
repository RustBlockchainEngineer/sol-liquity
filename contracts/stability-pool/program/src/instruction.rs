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
    ///   2. `[]` Token program id
    ///   3. `[]` nonce
    ///   4. `[]` StabilityPool program id
    Initialize {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,
    },

    ///  provideToSP():
    ///
    /// - Triggers a SOLID issuance, based on time passed since the last issuance. The SOLID issuance is shared between *all* depositors and front ends
    /// - Tags the deposit with the provided front end tag param, if it's a new deposit
    /// - Sends depositor's accumulated gains (SOLID, SOL) to depositor
    /// - Sends the tagged front end's accumulated SOLID gains to the tagged front end
    /// - Increases deposit and tagged front end's stake, and takes new snapshots for each.
    /// 
    ///   0. `[w]` StabilityPool account to provide to.
    ///   1. `[]` authority of this pool account
    ///   2. `[]` Token program id
    ///   3. `[]` StabilityPool program id
    ///   4. `[]` amount
    ProvideToSP(u64),

}

// below functions are used to test above instructions in the rust test side
// Function's parameters

/// Creates an 'initialize' instruction.
pub fn initialize(
    pool_id: &Pubkey,
    authority: &Pubkey,
    token_program_id: &Pubkey,
    nonce: u8,
    stability_pool_program_id: &Pubkey,
) -> Instruction {
    
    let init_data = StabilityPoolInstruction::Initialize{
        nonce,
    };
    
    let data = init_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(*pool_id, false),
        AccountMeta::new(*authority, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Instruction {
        program_id: *stability_pool_program_id,
        accounts,
        data,
    }
}

/// Creates instructions required to deposit into a farm pool, given a farm
/// account owned by the user.
pub fn deposit(
    pool_id: &Pubkey,
    authority: &Pubkey,
    token_program_id: &Pubkey,
    stability_pool_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*pool_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*token_program_id, false),
    ];
    Instruction {
        program_id: *stability_pool_program_id,
        accounts,
        data: StabilityPoolInstruction::ProvideToSP(amount).try_to_vec().unwrap(),
    }
}
