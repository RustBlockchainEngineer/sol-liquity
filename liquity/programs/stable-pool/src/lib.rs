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

use crate::{
    states::*,
    error::*,
    constant::*,
    instructions::*,
    processor::*,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stable_pool {
    use super::*;

    pub fn create_global_state(ctx: Context<CreateGlobalState>, nonce: u8) -> ProgramResult { process_create_global_state(ctx, nonce) }
    pub fn create_token_vault(ctx: Context<CreateTokenVault>, nonce: u8) -> ProgramResult { process_create_token_vault(ctx, nonce)}
    pub fn create_user_trove(ctx: Context<CreateUserTrove>, nonce: u8) -> ProgramResult { process_create_user_trove(ctx, nonce) }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> ProgramResult { process_deposit_collateral(ctx, amount) }
    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, amount: u64) -> ProgramResult { process_withdraw_collateral(ctx, amount) }
    pub fn repay_collateral(ctx: Context<RepayCollateral>, amount: u64) -> ProgramResult { process_repay_collateral(ctx, amount) }

    pub fn borrow_usd(ctx: Context<BorrowUsd>, amount: u64) -> ProgramResult { process_borrow_usd(ctx, amount) }
    pub fn repay_usd(ctx: Context<RepayUsd>, amount: u64) -> ProgramResult { process_repay_usd(ctx, amount) }
}