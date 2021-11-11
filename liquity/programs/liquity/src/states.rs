
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct FrontEnd {
    /// account type
    pub account_type: u8, 
    
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// kickback rate
    pub kickback_rate:u128,

    /// flag for registered frontend
    pub registered: u8,

    /// last recorded total deposits, tagged with that front end
    pub frontend_stake:u128,
}

impl FrontEnd {
    pub fn update_frontend_stake(&mut self,new_value:u128){
        self.frontend_stake = new_value;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Deposit {
    /// account type
    pub account_type: u8, 

    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// initial value
    pub initial_value:u128,

    /// tag public key of this frontend
    pub front_end_tag: Pubkey,
}

impl Deposit{
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Snapshots {
    /// account type
    pub account_type: u8, 

    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// S
    pub s:u128,

    /// P
    pub p:u128,

    /// G
    pub g:u128,

    /// scale
    pub scale: u128,

    /// epoch
    pub epoch: u128,
}

impl Snapshots{
    pub fn update_snapshots_with_frontendstake(&mut self,new_value:u128, pool_data:&StabilityPool, epoch_to_scale: &EpochToScale){
        if new_value == 0{
            return;
        }
        if  pool_data.current_scale != epoch_to_scale.scale ||
            pool_data.current_epoch != epoch_to_scale.epoch {
                return;
        }
        let current_scale_cached = pool_data.current_scale;
        let current_epoch_cached = pool_data.current_epoch;
        let current_p = pool_data.p;

        // Get G for the current epoch and current scale
        let current_g = epoch_to_scale.epoch_to_scale_to_g;

        // Record new snapshots of the latest running product p and sum g for the front end
        self.p = current_p;
        self.g = current_g;
        self.scale = current_scale_cached;
        self.epoch = current_epoch_cached;

    }
    pub fn update_snapshots_with_deposit(&mut self,new_value:u128, pool_data:&StabilityPool, epoch_to_scale: &EpochToScale){
        if new_value == 0{
            return;
        }
        if  pool_data.current_scale != epoch_to_scale.scale ||
            pool_data.current_epoch != epoch_to_scale.epoch {
                return;
        }
        let current_scale_cached = pool_data.current_scale;
        let current_epoch_cached = pool_data.current_epoch;
        let current_p = pool_data.p;

        // Get S and G for the current epoch and current scale
        let current_s = epoch_to_scale.epoch_to_scale_to_sum;
        let current_g = epoch_to_scale.epoch_to_scale_to_g;

        // Record new snapshots of the latest running product P, sum S, and sum G, for the depositor
        self.p = current_p;
        self.s = current_s;
        self.g = current_g;
        self.scale = current_scale_cached;
        self.epoch = current_epoch_cached;

    }
}



#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct TroveManager {
    /// account type
    pub account_type: u8, 

    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// BorrwoerOperations account
    pub borrower_operations_id: Pubkey,

    /// StabilityPool publickey
    pub stability_pool_id: Pubkey,

    /// Gas Pool publickey
    pub gas_pool_id: Pubkey,

    pub coll_surplus_pool_id: Pubkey,

    pub solusd_token_pubkey: Pubkey,

    pub solid_token_pubkey: Pubkey,

    pub solid_staking_pubkey: Pubkey,

    pub token_program_id: Pubkey,

    pub default_pool_id: Pubkey,

    pub active_pool_id: Pubkey,
    pub oracle_program_id: Pubkey,
    pub pyth_product_id: Pubkey,
    pub pyth_price_id: Pubkey,

    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

    pub base_rate:u128,

    pub last_fee_operation_time:u128,
    pub total_stakes:u128,
    pub total_stakes_snapshot:u128,
    pub total_collateral_snapshot:u128,
    pub l_sol:u128,
    pub l_solusd_debt:u128,
    pub last_sol_error_redistribution:u128,
    pub last_solusd_debt_error_redistribution:u128,

}
impl TroveManager{
    pub fn check_recovery_mode(&self, price: u128, active_pool_data: &ActivePool, default_pool_data: &DefaultPool)->u8{
        let entire_system_coll = active_pool_data.sol + default_pool_data.sol;
        let entire_system_debt = active_pool_data.solusd_debt + default_pool_data.solusd_debt;
        let tcr = compute_cr(entire_system_coll, entire_system_debt, price);
        return if tcr < CCR {1} else {0};
    }
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
    pub owner_pubkey:Pubkey,
    pub status:u8,
    pub debt:u128,
    pub coll:u128,
    pub stake:u128,
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
    pub fn close_trove(&mut self) {
        self.coll = 0;
        self.debt = 0;
        self.status = Status::ClosedByOwner as u8;
    }
    pub fn remove_stake(&mut self) {
        self.stake = 0;
    }

    pub fn increase_trove_coll(&mut self, coll_change:u128) ->u128  {
        self.coll += coll_change;
        return self.coll;
    }

    pub fn decrease_trove_coll(&mut self, coll_change:u128) ->u128  {
        self.coll -= coll_change;
        return self.coll;
    }

    pub fn increase_trove_debt(&mut self, debt_change:u128) ->u128  {
        self.debt += debt_change;
        return self.debt;
    }

    pub fn decrease_trove_debt(&mut self, debt_change:u128) ->u128 {
        self.debt -= debt_change;
        return self.debt;
    }

}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RewardSnapshot {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub sol:u128,
    pub solusd_debt:u128,
}
impl RewardSnapshot{
    pub fn update_trove_reward_snapshots(&mut self, trove_manager:&TroveManager){
        self.sol = trove_manager.l_sol;
        self.solusd_debt = trove_manager.l_solusd_debt;
    }
    pub fn reset(&mut self){
        self.sol = 0;
        self.solusd_debt = 0;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesOuterLiquidationFunction {
    pub price:u128,
    pub solusd_in_stab_pool:u128,
    pub recovery_mode_at_start:u8,
    pub liquidated_debt:u128,
    pub liquidated_coll:u128,
}
impl LocalVariablesOuterLiquidationFunction{
    pub fn new()->LocalVariablesOuterLiquidationFunction{
        LocalVariablesOuterLiquidationFunction{
            price: 0,
            solusd_in_stab_pool:0,
            recovery_mode_at_start:0,
            liquidated_debt:0,
            liquidated_coll:0
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesInnerSingleLiquidateFunction {
    pub coll_to_liquidate:u128,
    pub pending_debt_reward:u128,
    pub pending_coll_reward:u128,
}
impl LocalVariablesInnerSingleLiquidateFunction{
    pub fn new()->LocalVariablesInnerSingleLiquidateFunction{
        LocalVariablesInnerSingleLiquidateFunction{
            coll_to_liquidate:0,
            pending_debt_reward:0,
            pending_coll_reward:0
        }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesLiquidationSequence {
    pub remaining_solusd_in_stab_pool:u128,
    pub i:u128,
    pub icr:u128,
    pub user:Option<Pubkey>,
    pub back_to_normal_mode:u8,
    pub entire_system_debt:u128,
    pub entire_system_coll:u128,
}
impl LocalVariablesLiquidationSequence{
    pub fn new()->LocalVariablesLiquidationSequence{
        return LocalVariablesLiquidationSequence{
            remaining_solusd_in_stab_pool:0,
            i:0,
            icr:0,
            user:None,
            back_to_normal_mode:0,
            entire_system_debt:0,
            entire_system_coll:0
        };
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LiquidationValues {
    pub entire_trove_debt:u128,
    pub entire_trove_coll:u128,
    pub coll_gas_compensation:u128,
    pub solusd_gas_compensation:u128,
    pub debt_to_offset:u128,
    pub coll_to_send_to_sp:u128,
    pub debt_to_redistribute:u128,
    pub coll_to_redistribute:u128,
    pub coll_surplus:u128,
}
impl LiquidationValues{
    pub fn new()->LiquidationValues{
        return LiquidationValues{
            entire_trove_debt:0,
            entire_trove_coll:0,
            coll_gas_compensation:0,
            solusd_gas_compensation:0,
            debt_to_offset:0,
            coll_to_send_to_sp:0,
            debt_to_redistribute:0,
            coll_to_redistribute:0,
            coll_surplus:0
        };
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LiquidationTotals {
    pub total_coll_in_sequence:u128,
    pub total_debt_in_sequence:u128,
    pub total_coll_gas_compensation:u128,
    pub total_solusd_gas_compensation:u128,
    pub total_debt_to_offset:u128,
    pub total_coll_to_send_to_sp:u128,
    pub total_debt_to_redistribute:u128,
    pub total_coll_to_redistribute:u128,
    pub total_coll_surplus:u128,
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
    pub remaining_solusd:u128,
    pub total_solusd_to_redeem:u128,
    pub total_sol_drawn:u128,
    pub sol_fee:u128,
    pub sol_to_send_to_redeemer:u128,
    pub decayed_base_rate:u128,
    pub price:u128,
    pub total_solusd_supply_at_start:u128,
}
impl RedemptionTotals{
    pub fn new()->RedemptionTotals{
        RedemptionTotals{
            remaining_solusd:0,
            total_solusd_to_redeem:0,
            total_sol_drawn:0,
            sol_fee:0,
            sol_to_send_to_redeemer:0,
            decayed_base_rate:0,
            price:0,
            total_solusd_supply_at_start:0
        }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct SingleRedemptionValues {
    pub solusd_lot:u128,
    pub sol_lot:u128,
    pub cancelled_partial:u8,
}



#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ActivePool {
    pub borrower_operations_address: Pubkey,
    pub trove_manager_address: Pubkey,
    pub stability_pool_address: Pubkey,
    pub default_pool_address: Pubkey,
    pub sol: u128,
    pub solusd_debt: u128,
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
    
    pub fn increase_coll(&mut self, amount: u128){
        self.sol += amount;
    }

    pub fn increase_solusd_debt(&mut self, amount:u128){
        self.solusd_debt += amount;
    }
    pub fn decrease_solusd_debt(&mut self, amount:u128){
        self.solusd_debt -= amount;
    }
    pub fn send_sol(&mut self, _amount:u128){

    }

}

