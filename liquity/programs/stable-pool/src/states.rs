use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub super_owner: Pubkey,
}

#[account]
#[derive(Default)]
pub struct TokenVault {
    pub vault_owner: Pubkey,
    pub token_coll: Pubkey,
    pub mint_usd: Pubkey,
}

#[account]
#[derive(Default)]
pub struct UserTrove {
    pub trove_owner: Pubkey,
    pub token_vault: Pubkey,
    pub locked_coll_balance: u64,
    pub debt: u64
}
