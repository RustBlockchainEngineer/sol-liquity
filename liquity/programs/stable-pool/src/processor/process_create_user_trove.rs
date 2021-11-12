use anchor_lang::prelude::*;

use crate::{
    states::*,
    error::*,
    constant::*,
    instructions::*
};

pub fn process_create_user_trove(ctx: Context<CreateUserTrove>, nonce: u8) -> ProgramResult {
    ctx.accounts.user_trove.trove_owner = *ctx.accounts.trove_owner.key;
    ctx.accounts.user_trove.token_vault = ctx.accounts.token_vault.key();
    ctx.accounts.user_trove.locked_coll_balance = 0;
    ctx.accounts.user_trove.debt = 0;
    Ok(())
}
