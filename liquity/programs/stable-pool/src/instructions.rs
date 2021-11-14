use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount,Mint, ID};


use crate::{
    states::*,
    error::*,
    constant::*,
    processor::*,
};

#[derive(Accounts)]
#[instruction(nonce:u8)]
pub struct CreateGlobalState <'info>{
    #[account(signer)]
    pub super_owner:  AccountInfo<'info>,
    #[account(
    init,
    seeds = [b"golbal-state-seed"],
    bump = nonce,
    payer = super_owner,
    )]
    pub global_state:ProgramAccount<'info, GlobalState>,
    pub mint_usd:Account<'info, Mint>,
    pub system_program: AccountInfo<'info>,

}

#[derive(Accounts)]
#[instruction(nonce:u8)]
pub struct CreateTokenVault<'info> {
    #[account(signer)]
    pub payer:  AccountInfo<'info>,
    #[account(
    init,
    seeds = [b"token-vault-seed",mint_coll.key.as_ref()],
    bump = nonce,
    payer = payer,
    )]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    pub global_state:ProgramAccount<'info, GlobalState>,
    #[account(mut)]
    pub mint_coll:AccountInfo<'info>,
    pub token_coll:Account<'info, TokenAccount>,
    pub system_program: AccountInfo<'info>,


}

#[derive(Accounts)]
#[instruction(nonce:u8)]
pub struct CreateUserTrove<'info> {
    #[account(signer)]
    pub trove_owner:  AccountInfo<'info>,
    #[account(
    init,
    seeds = [b"user-trove-seed",token_vault.key().as_ref(),trove_owner.key.as_ref()],
    bump = nonce,
    payer = trove_owner,
    )]
    pub user_trove:ProgramAccount<'info, UserTrove>,
    #[account(mut)]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DepositCollateral<'info> {
    #[account(signer)]
    pub owner:  AccountInfo<'info>,
    #[account(mut)]
    pub user_trove:ProgramAccount<'info, UserTrove>,
    #[account(mut)]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    #[account(mut)]
    pub pool_token_coll:Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_coll:Account<'info, TokenAccount>,
    pub token_program:AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawCollateral<'info> {
    #[account(signer)]
    pub owner:  AccountInfo<'info>,
    #[account(mut)]
    pub user_trove:ProgramAccount<'info, UserTrove>,
    #[account(mut)]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    #[account(mut)]
    pub pool_token_coll:Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_coll:Account<'info, TokenAccount>,
    pub token_program:AccountInfo<'info>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct RepayCollateral<'info> {
    #[account(signer)]
    pub owner:  AccountInfo<'info>,
    #[account(mut)]
    pub user_trove:ProgramAccount<'info, UserTrove>,
    #[account(mut)]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    #[account(mut)]
    pub pool_token_coll:Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_coll:Account<'info, TokenAccount>,
    pub token_program:AccountInfo<'info>,
}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BorrowUsd<'info> {
    #[account(signer)]
    pub owner:  AccountInfo<'info>,
    #[account(mut)]
    pub user_trove:ProgramAccount<'info, UserTrove>,
    #[account(mut)]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    #[account(mut)]
    pub global_state:ProgramAccount<'info, GlobalState>,
    #[account(mut)]
    pub pool_token_coll:Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_usd:AccountInfo<'info>,
    #[account(mut)]
    pub user_token_usd:AccountInfo<'info>,

    pub token_program:AccountInfo<'info>,
}


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct RepayUsd<'info> {
    #[account(signer)]
    pub owner:  AccountInfo<'info>,
    #[account(mut)]
    pub user_trove:ProgramAccount<'info, UserTrove>,
    #[account(mut)]
    pub token_vault:ProgramAccount<'info, TokenVault>,
    #[account(mut)]
    pub global_state:ProgramAccount<'info, GlobalState>,
    #[account(mut)]
    pub mint_usd:AccountInfo<'info>,
    #[account(mut)]
    pub user_token_usd:AccountInfo<'info>,

    pub token_program:AccountInfo<'info>,
}
