//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
    num_traits::FromPrimitive,
    num_derive::FromPrimitive, 
};

/// SOLID Staking struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct TroveManager {
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// BorrwoerOperations account
    pub borrower_operations_id: Pubkey,

    /// StabilityPool publickey
    pub stability_pool_id: Pubkey,

    /// Gas Pool publickey
    pub gas_pool_id: Pubkey,

    pub solusd_token_pubkey: Pubkey,

    pub solid_token_pubkey: Pubkey,

    pub solid_staking_pubkey: Pubkey,

    pub token_program_id: Pubkey,

    pub default_pool_id: Pubkey,

    pub active_pool_id: Pubkey,

    pub base_rate:u64,

    pub last_fee_operation_time:u64,
    pub total_stakes:u64,
    pub total_stakes_snapshot:u64,
    pub total_collateral_snapshot:u64,
    pub l_sol:u64,
    pub l_solusd_debt:u64,
    pub last_sol_error_redistribution:u64,
    pub last_solusd_debt_error_redistribution:u64,

}

#[repr(C)]
#[derive(FromPrimitive, Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum Status {
    NonExistent,
    Active,
    ClosedByOwner,
    ClosedByLiquidation,
    ClosedByRedemption
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Trove {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub debt:u64,
    pub coll:u64,
    pub stake:u64,
    pub status:u8,
    pub array_index:u128,
}
impl Trove {
    pub fn is_active(&self)->bool {
        let status = Status::from_u8(self.status).unwrap();
        match status {
            Status::Active =>{
                return true
            }
            _ =>{
                return false;
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RewardSnapshot {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub sol:u64,
    pub solusd_debt:u64,
}
impl RewardSnapshot{
    pub fn update_trove_reward_snapshots(&mut self, trove_manager:&TroveManager){
        self.sol = trove_manager.l_sol;
        self.solusd_debt = trove_manager.l_solusd_debt;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesOuterLiquidationFunction {
    pub price:u64,
    pub solusd_in_stab_pool:u64,
    pub recovery_mode_at_start:u8,
    pub liquidated_debt:u64,
    pub liquidated_coll:u64,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesInnerSingleLiquidateFunction {
    pub coll_to_liquidate:u64,
    pub pending_debt_reward:u64,
    pub pending_coll_reward:u64,
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesLiquidationSequence {
    pub remaining_solusd_in_stab_pool:u64,
    pub i:u64,
    pub icr:u64,
    pub user:Pubkey,
    pub back_to_normal_mode:u8,
    pub entire_system_debt:u64,
    pub entire_system_coll:u64,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LiquidationValues {
    pub entire_trove_debt:u64,
    pub entire_trove_coll:u64,
    pub coll_gas_compensation:u64,
    pub solusd_gas_compensation:u64,
    pub debt_to_offset:u64,
    pub coll_to_send_to_sp:u64,
    pub debt_to_redistribute:u64,
    pub coll_to_redistribute:u64,
    pub coll_surplus:u64,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
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
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ContractsCache {
    pub active_pool_pubkey:Pubkey,
    pub default_pool_pubkey:Pubkey,
    pub solusd_token_pubkey:Pubkey,
    pub solid_staking_pubkey:Pubkey,
    pub sorted_troves_pubkey:Pubkey,
    pub coll_surplus_pool:Pubkey,
    pub gas_pool_pubkey:Pubkey,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RedemptionTotals {
    pub remaining_solusd:u64,
    pub total_solusd_to_redeem:u64,
    pub total_sol_drawn:u64,
    pub sol_fee:u64,
    pub sol_to_send_to_redeemer:u64,
    pub decayed_base_rate:u64,
    pub price:u64,
    pub total_solusd_supply_at_start:u64,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct SingleRedemptionValues {
    pub solusd_lot:u64,
    pub sol_lot:u64,
    pub cancelled_partial:u8,
}



#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ActivePool {
    pub borrower_operations_address: Pubkey,
    pub trove_manager_address: Pubkey,
    pub stability_pool_address: Pubkey,
    pub default_pool_address: Pubkey,
    pub sol: u64,
    pub solusd_debt: u64,
}
impl ActivePool{
    pub fn set_addresses(
        &mut self, 
        borrower_operatins_address: &Pubkey,
        trove_manager_address: &Pubkey,
        stability_pool_address: &Pubkey,
        default_pool_address: &Pubkey,
    ){
        self.borrower_operations_address = *borrower_operatins_address;
        self.trove_manager_address = *trove_manager_address;
        self.stability_pool_address = *stability_pool_address;
        self.default_pool_address = *default_pool_address;

    }
    pub fn increase_solusd_debt(&mut self, amount:u64){
        self.solusd_debt += amount;
    }
    pub fn decrease_solusd_debt(&mut self, amount:u64){
        self.solusd_debt -= amount;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct DefaultPool {
    pub trove_manager_address: Pubkey,
    pub active_pool_address: Pubkey,
    pub sol: u64,
    pub solusd_debt: u64,
}
impl DefaultPool{
    pub fn set_addresses(
        &mut self, 
        trove_manager_address: &Pubkey,
        active_pool_address: &Pubkey,
    ){
        self.trove_manager_address = *trove_manager_address;
        self.active_pool_address = *active_pool_address;

    }
    pub fn increase_solusd_debt(&mut self, amount:u64){
        self.solusd_debt += amount;
    }
    pub fn decrease_solusd_debt(&mut self, amount:u64){
        self.solusd_debt -= amount;
    }
}