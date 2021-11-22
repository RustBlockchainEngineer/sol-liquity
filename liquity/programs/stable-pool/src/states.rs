use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub super_owner: Pubkey,
    pub mint_usd: Pubkey,
    
}

#[account]
#[derive(Default)]
pub struct TokenVault {
    pub mint_coll: Pubkey,
    pub token_coll: Pubkey,
    pub total_coll: u64,
    pub total_debt: u64,
    
    pub oracle_program: Pubkey,
    pub pyth_product: Pubkey,
    pub pyth_price: Pubkey,
}


#[account]
#[derive(Default)]
pub struct UserTrove {
    pub locked_coll_balance: u64,
    pub debt: u64
}
