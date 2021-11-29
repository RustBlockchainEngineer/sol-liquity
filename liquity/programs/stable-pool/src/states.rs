use anchor_lang::prelude::*;
use crate::{
    constant::*,
    utils::*
};
#[account]
#[derive(Default)]
pub struct GlobalState {
    pub super_owner: Pubkey,
    pub mint_usd: Pubkey,
    pub stability_solusd_pool: Pubkey,
    
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
impl TokenVault {
    pub fn check_recovery_mode(&self, market_price: u128) -> bool {
        let tcr = compute_cr(self.total_coll, self.total_debt, market_price);
        return tcr < CCR ;
    }
    pub fn get_current_icr(&self, user_trove: &UserTrove)->u128 {
        let pending_sol_reward = self.get_pending_sol_reward(user_trove);
        let pending_solusd_debt_reward = get_pending_solusd_debt_reward(user_trove);

        let current_sol = user_trove.locked_coll_balance + pending_sol_reward;
        let current_solusd = user_trove.debt + pending_solusd_debt_reward;

        return (current_sol, current_solusd);
    }
    pub fn get_pending_sol_reward(&self, user_trove: &UserTrove)->u128 {
        0
    }
    pub fn get_pending_solusd_reward(&self, user_trove: &UserTrove)->u128 {
        0
    }
}

#[account]
#[derive(Default)]
pub struct UserTrove {
    pub owner: Pubkey,
    pub locked_coll_balance: u64,
    pub debt: u64
}
