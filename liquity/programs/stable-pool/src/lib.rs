use anchor_lang::prelude::*;

/// states
pub mod states;
///processor
pub mod processor;
/// error
pub mod error;
/// constant
pub mod constant;
/// instructions
pub mod instructions;
/// utils
pub mod utils;
/// pyth
pub mod pyth;

use crate::{
    instructions::*,
    processor::*,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stable_pool {
    use super::*;

    pub fn create_global_state(ctx: Context<CreateGlobalState>, global_state_nonce:u8, mint_usd_nonce:u8, stability_pool_nonce:u8) -> ProgramResult { 
        process_create_global_state(ctx, global_state_nonce, mint_usd_nonce, stability_pool_nonce) 
    }
    pub fn create_token_vault(ctx: Context<CreateTokenVault>, token_vault_nonce:u8, global_state_nonce:u8, token_coll_nonce:u8) -> ProgramResult { 
        process_create_token_vault(ctx, token_vault_nonce, global_state_nonce, token_coll_nonce)
    }
    pub fn create_user_trove(ctx: Context<CreateUserTrove>, user_trove_nonce:u8, token_vault_nonce:u8) -> ProgramResult { 
        process_create_user_trove(ctx, user_trove_nonce, token_vault_nonce) 
    }
    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, token_coll_nonce: u8) -> ProgramResult { 
        process_deposit_collateral(ctx, amount, token_vault_nonce, user_trove_nonce, token_coll_nonce) 
    }
    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, token_coll_nonce: u8) -> ProgramResult { 
        process_withdraw_collateral(ctx, amount, token_vault_nonce, user_trove_nonce, token_coll_nonce) 
    }
    pub fn borrow_usd(ctx: Context<BorrowUsd>, amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, global_state_nonce: u8, mint_usd_nonce: u8) -> ProgramResult { 
        process_borrow_usd(ctx, amount, token_vault_nonce, user_trove_nonce, global_state_nonce, mint_usd_nonce) 
    }
    pub fn repay_usd(ctx: Context<RepayUsd>, amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, global_state_nonce: u8, mint_usd_nonce: u8) -> ProgramResult { 
        process_repay_usd(ctx, amount, token_vault_nonce, user_trove_nonce, global_state_nonce, mint_usd_nonce) 
    }
    pub fn liquidate_trove(ctx: Context<LiquidateTrove>, global_state_nonce: u8, token_vault_nonce: u8, user_trove_nonce: u8) -> ProgramResult { 
        process_liquidate_trove(ctx, global_state_nonce, token_vault_nonce, user_trove_nonce) 
    }
    pub fn sp_deposit(ctx: Context<SPDeposit>, amount: u64, global_state_nonce: u8, sp_user_info_nonce: u8, stability_pool_nonce: u8) -> ProgramResult { 
        process_sp_deposit(ctx, amount, global_state_nonce, sp_user_info_nonce, stability_pool_nonce) 
    }
    pub fn sp_withdraw(ctx: Context<SPWithdraw>, amount: u64, global_state_nonce: u8, sp_user_info_nonce: u8, stability_pool_nonce: u8) -> ProgramResult { 
        process_sp_withdraw(ctx, amount, global_state_nonce, sp_user_info_nonce, stability_pool_nonce) 
    }
    pub fn sp_sol_gain_to_trove(ctx: Context<SPGainToTrove>, amount: u64, global_state_nonce: u8, sp_user_info_nonce: u8, stability_pool_nonce: u8) -> ProgramResult { 
        process_sp_sol_gain_to_trove(ctx, amount, global_state_nonce, sp_user_info_nonce, stability_pool_nonce) 
    }
}