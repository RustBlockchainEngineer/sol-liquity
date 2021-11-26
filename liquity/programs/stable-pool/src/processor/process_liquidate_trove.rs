use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Burn, ID};

use crate::{
    constant::*,
    instructions::*,
};

pub fn process_liquidate_trove(ctx: Context<LiquidateTrove>, global_state_nonce: u8, token_vault_nonce: u8, user_trove_nonce: u8) -> ProgramResult {


    Ok(())
}
