use anchor_lang::prelude::*;

use crate::{
    states::*,
    error::*,
    constant::*,
    instructions::*
};

pub fn process_create_token_vault(ctx: Context<CreateTokenVault>, nonce: u8) -> ProgramResult {
    ctx.accounts.token_vault.vault_owner = *ctx.accounts.vault_owner.key;
    ctx.accounts.token_vault.token_coll = ctx.accounts.token_coll.key();
    ctx.accounts.token_vault.mint_usd = *ctx.accounts.mint_usd.key;
    Ok(())
}
