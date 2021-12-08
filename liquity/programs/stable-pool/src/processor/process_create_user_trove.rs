use anchor_lang::prelude::*;

use crate::{
    instructions::*
};

pub fn process_create_user_trove(ctx: Context<CreateUserTrove>, _user_trove_nonce:u8, _token_vault_nonce:u8) -> ProgramResult {
    ctx.accounts.user_trove.owner = ctx.accounts.trove_owner.key();
    ctx.accounts.user_trove.coll = 0;
    ctx.accounts.user_trove.debt = 0;
    Ok(())
}
