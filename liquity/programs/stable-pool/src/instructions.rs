use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount,Mint};


use crate::{
    states::*,
    constant::*,
};

#[derive(Accounts)]
#[instruction(global_state_nonce:u8, mint_usd_nonce:u8)]
pub struct CreateGlobalState <'info>{
    pub super_owner:  Signer<'info>,

    #[account(
    init,
    seeds = [GLOBAL_STATE_TAG],
    bump = global_state_nonce,
    payer = super_owner,
    )]
    pub global_state:Box<Account<'info, GlobalState>>,

    #[account(init,
        mint::decimals = SOLUSD_DECIMALS,
        mint::authority = global_state,
        seeds = [SOLUSD_MINT_TAG],
        bump = mint_usd_nonce,
        payer = super_owner)]
    pub mint_usd:Box<Account<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

}

#[derive(Accounts)]
#[instruction(token_vault_nonce:u8, global_state_nonce:u8, token_coll_nonce:u8)]
pub struct CreateTokenVault<'info> {
    pub payer:  Signer<'info>,
    #[account(
        init,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
        payer = payer,
        constraint = payer.key() == global_state.super_owner
    )]
    pub token_vault: Box<Account<'info, TokenVault>>,

    #[account(seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce)]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub mint_coll:Box<Account<'info, Mint>>,

    #[account(init,
        token::mint = mint_coll,
        token::authority = token_vault,
        seeds = [TOKEN_VAULT_POOL_TAG, token_vault.key().as_ref()],
        bump = token_coll_nonce,
        payer = payer)]
    pub token_coll:Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(user_trove_nonce:u8, token_vault_nonce:u8)]
pub struct CreateUserTrove<'info> {
    pub trove_owner:  Signer<'info>,
    #[account(
    init,
    seeds = [USER_TROVE_TAG,token_vault.key().as_ref(), trove_owner.key().as_ref()],
    bump = user_trove_nonce,
    payer = trove_owner,
    )]
    pub user_trove:Box<Account<'info, UserTrove>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
    )]
    pub token_vault:Box<Account<'info, TokenVault>>,
    #[account(mut,
        constraint = mint_coll.key() == token_vault.mint_coll)]
    pub mint_coll:Box<Account<'info, Mint>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, token_coll_nonce: u8)]
pub struct DepositCollateral<'info> {
    pub owner:  Signer<'info>,
    #[account(mut,
        seeds = [USER_TROVE_TAG,token_vault.key().as_ref(), owner.key().as_ref()],
        bump = user_trove_nonce)]
    pub user_trove:Box<Account<'info, UserTrove>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
    )]
    pub token_vault:Box<Account<'info, TokenVault>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_POOL_TAG,token_vault.key().as_ref()],
        bump = token_coll_nonce,
    )]
    pub pool_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = user_token_coll.owner == owner.key(),
        constraint = user_token_coll.mint == token_vault.mint_coll)]
    pub user_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = mint_coll.key() == token_vault.mint_coll)]
    pub mint_coll:Box<Account<'info, Mint>>,
    pub token_program:Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, token_coll_nonce: u8)]
pub struct WithdrawCollateral<'info> {
    pub owner:  Signer<'info>,
    #[account(mut,
        seeds = [USER_TROVE_TAG,token_vault.key().as_ref(), owner.key().as_ref()],
        bump = user_trove_nonce)]
    pub user_trove:Box<Account<'info, UserTrove>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
    )]
    pub token_vault:Box<Account<'info, TokenVault>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_POOL_TAG,token_vault.key().as_ref()],
        bump = token_coll_nonce,
    )]
    pub pool_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = user_token_coll.owner == owner.key(),
        constraint = user_token_coll.mint == token_vault.mint_coll)]
    pub user_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = mint_coll.key() == token_vault.mint_coll)]
    pub mint_coll:Box<Account<'info, Mint>>,
    pub token_program:Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, token_coll_nonce: u8)]
pub struct RepayCollateral<'info> {
    pub owner:  Signer<'info>,
    #[account(mut,
        seeds = [USER_TROVE_TAG,token_vault.key().as_ref(), owner.key().as_ref()],
        bump = user_trove_nonce)]
    pub user_trove:Box<Account<'info, UserTrove>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
    )]
    pub token_vault:Box<Account<'info, TokenVault>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_POOL_TAG,token_vault.key().as_ref()],
        bump = token_coll_nonce,
    )]
    pub pool_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = user_token_coll.owner == owner.key(),
        constraint = user_token_coll.mint == token_vault.mint_coll)]
    pub user_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = mint_coll.key() == token_vault.mint_coll)]
    pub mint_coll:Box<Account<'info, Mint>>,
    pub token_program:Program<'info, Token>,
}


#[derive(Accounts)]
#[instruction(amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, global_state_nonce: u8, token_coll_nonce: u8, mint_usd_nonce: u8)]
pub struct BorrowUsd<'info> {
    pub owner:  Signer<'info>,
    #[account(mut,
        seeds = [USER_TROVE_TAG,token_vault.key().as_ref(), owner.key().as_ref()],
        bump = user_trove_nonce)]
    pub user_trove:Box<Account<'info, UserTrove>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
    )]
    pub token_vault:Box<Account<'info, TokenVault>>,
    #[account(seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce)]
    pub global_state: Box<Account<'info, GlobalState>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_POOL_TAG,token_vault.key().as_ref()],
        bump = token_coll_nonce,
    )]
    pub pool_token_coll:Box<Account<'info, TokenAccount>>,
    #[account(
        seeds = [SOLUSD_MINT_TAG],
        bump = mint_usd_nonce)]
    pub mint_usd:Box<Account<'info, Mint>>,
    #[account(mut,
        constraint = user_token_usd.owner == owner.key(),
        constraint = user_token_usd.mint == global_state.mint_usd)]
    pub user_token_usd:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = mint_coll.key() == token_vault.mint_coll)]
    pub mint_coll:Box<Account<'info, Mint>>,
    pub token_program:Program<'info, Token>,
}


#[derive(Accounts)]
#[instruction(amount: u64, token_vault_nonce: u8, user_trove_nonce: u8, global_state_nonce: u8, mint_usd_nonce: u8)]
pub struct RepayUsd<'info> {
    pub owner:  Signer<'info>,
    #[account(mut,
        seeds = [USER_TROVE_TAG,token_vault.key().as_ref(), owner.key().as_ref()],
        bump = user_trove_nonce)]
    pub user_trove:Box<Account<'info, UserTrove>>,
    #[account(mut,
        seeds = [TOKEN_VAULT_TAG,mint_coll.key().as_ref()],
        bump = token_vault_nonce,
    )]
    pub token_vault:Box<Account<'info, TokenVault>>,
    #[account(seeds = [GLOBAL_STATE_TAG],
        bump = global_state_nonce)]
    pub global_state: Box<Account<'info, GlobalState>>,
    #[account(
        seeds = [SOLUSD_MINT_TAG],
        bump = mint_usd_nonce)]
    pub mint_usd:Box<Account<'info, Mint>>,
    #[account(mut,
        constraint = user_token_usd.owner == owner.key(),
        constraint = user_token_usd.mint == global_state.mint_usd)]
    pub user_token_usd:Box<Account<'info, TokenAccount>>,
    #[account(mut,
        constraint = mint_coll.key() == token_vault.mint_coll)]
    pub mint_coll:Box<Account<'info, Mint>>,
    pub token_program:Program<'info, Token>,
}
