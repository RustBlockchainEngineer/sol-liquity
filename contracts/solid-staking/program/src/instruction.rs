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

/// Instructions supported by the SOLID Staking program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum SOLIDStakingInstruction {
    ///   Initializes a new SOLID Staking.
    ///   These represent the parameters that will be included from client side
    ///   [w] - writable, [s] - signer
    /// 
    ///   0. `[w]` New SOLID Staking account to create.
    ///   1. `[]` authority to initialize this pool account
    ///   2. `[]` SOLID pool token account
    ///   3. `[]` Token program id
    ///   4. `[]` nonce
    ///   5. `[]` SOLIDStaking program id
    Initialize {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,
    },

    /// Provide to SOLID Staking
    ///
    /// If caller has a pre-existing stake, send any accumulated SOL and solUSD gains to them. 
    /// 
    ///   0. `[w]` SOLIDStaking account to stake
    ///   1. `[]` authority of this pool account
    ///   2. `[]` SOLID pool token account account
    ///   3. `[]` SOLID user token account account
    ///   4. `[]` user transfer authority
    ///   5. `[]` user deposit data account
    ///   6. `[]` snapshot account
    ///   7. `[]` Token program id
    ///   8. `[]` SOLIDStaking program id
    ///   9. `[]` amount
    Stake(u64),

    /// Withdraw from SOLID Staking
    ///
    /// Unstake the SOLID and send the it back to the caller, along with their accumulated solUSD & SOL gains. 
    //  If requested amount > stake, send their entire stake.
    /// 
    ///   0. `[w]` SOLIDStaking account to withdraw from.
    ///   1. `[]` authority of this pool account
    ///   2. `[]` SOLID pool token account account
    ///   3. `[]` SOLID user token account account
    ///   4. `[]` user transfer authority
    ///   5. `[]` user deposit data account
    ///   6. `[]` Token program id
    ///   7. `[]` SOLIDStaking program id
    ///   8. `[]` amount
    Unstake(u64),

}

// below functions are used to test above instructions in the rust test side
// Function's parameters

/// Creates an 'initialize' instruction.
pub fn initialize(
    pool_id: &Pubkey,
    authority: &Pubkey,
    solid_pool_token_pubkey: &Pubkey,
    token_program_id: &Pubkey,
    nonce: u8,
    solid_staking_program_id: &Pubkey,
) -> Instruction {
    
    let init_data = SOLIDStakingInstruction::Initialize{
        nonce,
    };
    
    let data = init_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(*pool_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*solid_pool_token_pubkey, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Instruction {
        program_id: *solid_staking_program_id,
        accounts,
        data,
    }
}

/// Creates instructions required to stake into a SOLID Staking pool
pub fn stake(
    pool_id: &Pubkey,
    authority: &Pubkey,
    solid_pool_token_pubkey: &Pubkey,
    solid_user_token_pubkey: &Pubkey,
    user_transfer_authority_pubkey: &Pubkey,
    user_deposit_pubkey: &Pubkey,
    snapshot_pubkey: &Pubkey,
    token_program_id: &Pubkey,
    solid_staking_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*pool_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*solid_pool_token_pubkey, false),
        AccountMeta::new(*solid_user_token_pubkey, false),
        AccountMeta::new(*user_transfer_authority_pubkey, false),
        AccountMeta::new(*user_deposit_pubkey, false),
        AccountMeta::new(*snapshot_pubkey, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Instruction {
        program_id: *solid_staking_program_id,
        accounts,
        data: SOLIDStakingInstruction::Stake(amount).try_to_vec().unwrap(),
    }
}

/// Creates instructions required to unstake from SOLID Staking pool
pub fn unstake(
    pool_id: &Pubkey,
    authority: &Pubkey,
    solid_pool_token_pubkey: &Pubkey,
    solid_user_token_pubkey: &Pubkey,
    user_transfer_authority_pubkey: &Pubkey,
    user_deposit_pubkey: &Pubkey,
    token_program_id: &Pubkey,
    solid_staking_program_id: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*pool_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*solid_pool_token_pubkey, false),
        AccountMeta::new(*solid_user_token_pubkey, false),
        AccountMeta::new(*user_transfer_authority_pubkey, false),
        AccountMeta::new(*user_deposit_pubkey, false),
        AccountMeta::new_readonly(*token_program_id, false),
    ];
    Instruction {
        program_id: *solid_staking_program_id,
        accounts,
        data: SOLIDStakingInstruction::Unstake(amount).try_to_vec().unwrap(),
    }
}
