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
    pub sp_solusd_amount: u64,
    pub sp_sol_amount: u64,
    
}

#[account]
#[derive(Default)]
pub struct TokenVault {
    pub mint_coll: Pubkey,
    pub token_coll: Pubkey,

    pub total_coll: u64,
    pub total_debt: u64,

    pub active_total_coll: u64,
    pub default_total_coll: u64,
    pub active_total_debt: u64,
    pub default_total_debt: u64,

    pub oracle_program: Pubkey,
    pub pyth_product: Pubkey,
    pub pyth_price: Pubkey,
}
impl TokenVault {
    pub fn check_recovery_mode(&self, market_price: u64) -> bool {
        let tcr = compute_cr(self.total_coll, self.total_debt, market_price);
        return tcr < CCR ;
    }
}

#[account]
#[derive(Default)]
pub struct UserTrove {
    pub owner: Pubkey,
    pub state: u8,
    pub coll: u64,
    pub debt: u64
}
impl UserTrove {
    pub fn is_closed(&self)->bool{
        self.state == 0
    }
    pub fn close(&mut self) {
        self.state = 0;
        self.coll = 0;
        self.debt = 0;
    }
}
#[account]
#[derive(Default)]
pub struct SPUserInfo {
    pub owner: Pubkey,
    pub deposit_balance: u64,
}


pub struct LiquidationTotals {
    pub total_coll_in_sequence:u64,
    pub total_debt_in_sequence:u64,
    pub total_coll_gas_compensation:u64,
    pub total_solusd_gas_compensation:u64,
    pub total_debt_to_offset:u64,
    pub total_coll_to_send_to_sp:u64,
    pub total_debt_to_redistribute:u64,
    pub total_coll_to_redistribute:u64,
    pub total_coll_surplus:u64,
}
impl LiquidationTotals{
    pub fn new()->LiquidationTotals{
        LiquidationTotals{
            total_coll_in_sequence:0,
            total_debt_in_sequence:0,
            total_coll_gas_compensation:0,
            total_solusd_gas_compensation:0,
            total_debt_to_offset:0,
            total_coll_to_send_to_sp:0,
            total_debt_to_redistribute:0,
            total_coll_to_redistribute:0,
            total_coll_surplus:0,
        }
    }
}