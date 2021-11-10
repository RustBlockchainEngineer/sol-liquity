use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct LiquityState {
    owner: Pubkey,
}
