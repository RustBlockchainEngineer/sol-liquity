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
use crate::{
    constant::*,
    liquity_math::*
};
use std::str::FromStr;
use std::convert::TryFrom;

/// Stability Pool struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct StabilityPool {
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// solUSD token's mint address
    pub token_program_pubkey: Pubkey,

    /// solUSD token's mint address
    pub sol_usd_pool_token_pubkey: Pubkey,

    /// Borrower Operations pubkey
    pub borrower_operations_pubkey: Pubkey,

    /// Borrower Operations pubkey
    pub trove_manager_pubkey: Pubkey,

    pub community_issuance_pubkey: Pubkey,

    /// Tracker for solUSD held in the pool. Changes when users deposit/withdraw, and when Trove debt is offset.
    pub total_sol_usd_deposits: u128,

    // Error tracker for the error correction in the SOLID issuance calculation
    pub last_solid_error:u128,

    // Error trackers for the error correction in the offset calculation
    pub last_sol_error_offset:u128,

    pub last_solusd_loss_error_offset:u128,

    pub p:u128,

    pub current_scale:u128,

    pub current_epoch:u128,

    // deposited sol tracker
    pub sol: u128,

    /// Oracle (Pyth) program id
    pub oracle_program_id: Pubkey,
    
    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

}
impl StabilityPool{
    pub fn offset(&mut self, debt_to_offset: u128, coll_to_add: u128, solid_issuance: u128, active_pool:&mut ActivePool){
        let total_solusd = self.total_sol_usd_deposits;
        if total_solusd == 0 || debt_to_offset == 0 {
            return;
        }
        self.trigger_solid_issuance(solid_issuance);
        let (sol_gain_per_unit_staked, solusd_loss_per_unit_staked) = self.compute_rewards_per_unit_staked(coll_to_add, debt_to_offset, total_solusd);
        self.update_reward_sum_and_product(sol_gain_per_unit_staked, solusd_loss_per_unit_staked);

        self.move_offset_coll_and_debt(coll_to_add, debt_to_offset, active_pool);
    }
    pub fn move_offset_coll_and_debt(&mut self, coll_to_add: u128, debt_to_offset: u128, active_pool:&mut ActivePool){
        active_pool.decrease_solusd_debt(debt_to_offset);
        self.decrease_solusd(debt_to_offset);

        // Burn the debt that was successfully offset
        //lusdToken.burn(address(this), _debtToOffset);

        //activePoolCached.sendETH(address(this), _collToAdd);
    }
    pub fn decrease_solusd(&mut self, amount: u128){
        let new_total_solusd_deposits = self.total_sol_usd_deposits - amount;
        self.total_sol_usd_deposits = new_total_solusd_deposits;
    }
    pub fn update_reward_sum_and_product(&mut self, sol_gain_per_unit_staked: u128, solusd_loss_per_unit_staked: u128){
        let current_p = self.p;
        let mut new_p = 0;

        //assert(_LUSDLossPerUnitStaked <= DECIMAL_PRECISION);
        /*
        * The newProductFactor is the factor by which to change all deposits, due to the depletion of Stability Pool SOLUSD in the liquidation.
        * We make the product factor 0 if there was a pool-emptying. Otherwise, it is (1 - SOLUSDLossPerUnitStaked)
        */
        let new_product_factor = DECIMAL_PRECISION - solusd_loss_per_unit_staked;
        let current_scale_cached = self.current_scale;
        let current_epoch_cached = self.current_epoch;
        //let current_s = epochToScaleToSum[currentEpochCached][currentScaleCached];
        let current_s = 0;

        /*
        * Calculate the new S first, before we update P.
        * The SOL gain for any given depositor from a liquidation depends on the value of their deposit
        * (and the value of totalDeposits) prior to the Stability being depleted by the debt in the liquidation.
        *
        * Since S corresponds to SOL gain, and P to deposit loss, we update S first.
        */
        let marginal_solid_gain = sol_gain_per_unit_staked * current_p;
        let new_s = current_s + marginal_solid_gain;
        //epochToScaleToSum[currentEpochCached][currentScaleCached] = newS;

        // If the Stability Pool was emptied, increment the epoch, and reset the scale and product P
        if new_product_factor == 0 {
            self.current_epoch = current_epoch_cached + 1;
            self.current_scale = 0;
            new_p = DECIMAL_PRECISION;
        }
        else if current_p * new_product_factor / DECIMAL_PRECISION < SCALE_FACTOR {
            new_p = current_p * new_product_factor * SCALE_FACTOR / DECIMAL_PRECISION;
            self.current_scale = current_scale_cached;
        }
        else {
            new_p = current_p * new_product_factor / DECIMAL_PRECISION;
        }

        //assert(newP > 0);
        self.p = new_p;

    }
    pub fn trigger_solid_issuance(&mut self,solid_issuance:u128){
        self.update_g(solid_issuance);
    }
    pub fn update_g(&mut self,solid_issuance:u128){
        // cached to save an SLOAD
        let total_solusd = self.total_sol_usd_deposits;

        /*
        * When total deposits is 0, G is not updated. In this case, the SOLID issued can not be obtained by later
        * depositors - it is missed out on, and remains in the balanceof the CommunityIssuance contract.
        *
        */
        if total_solusd == 0 || solid_issuance == 0 {
            return;
        }
        let solid_per_unit_staked = self.compute_solid_per_unit_staked(solid_issuance, total_solusd);

        let marginal_solid_gain = solid_per_unit_staked * self.p;
        //epochToScaleToG[currentEpoch][currentScale] = epochToScaleToG[currentEpoch][currentScale].add(marginalLQTYGain);
    }
    pub fn compute_rewards_per_unit_staked(&mut self, coll_to_add: u128, debt_to_offset:u128, total_solusd_deposits: u128) ->(u128,u128){
        /*
        * Compute the SOLUSD and SOL rewards. Uses a "feedback" error correction, to keep
        * the cumulative error in the P and S state variables low:
        *
        * 1) Form numerators which compensate for the floor division errors that occurred the last time this 
        * function was called.  
        * 2) Calculate "per-unit-staked" ratios.
        * 3) Multiply each ratio back by its denominator, to reveal the current floor division error.
        * 4) Store these errors for use in the next correction when this function is called.
        * 5) Note: static analysis tools complain about this "division before multiplication", however, it is intended.
        */
        let mut sol_gain_per_unit_staked = 0;
        let mut solusd_loss_per_unit_staked = 0;

        let sol_numerator = coll_to_add * DECIMAL_PRECISION + self.last_sol_error_offset;
        //assert(_debtToOffset <= _totalLUSDDeposits);
        if debt_to_offset == total_solusd_deposits {
            solusd_loss_per_unit_staked = DECIMAL_PRECISION;// When the Pool depletes to 0, so does each deposit 
            self.last_sol_error_offset = 0;
        }
        else {
            let solusd_loss_numerator = debt_to_offset * DECIMAL_PRECISION - self.last_solusd_loss_error_offset;
            /*
            * Add 1 to make error in quotient positive. We want "slightly too much" SOLUSD loss,
            * which ensures the error in any given compoundedLUSDDeposit favors the Stability Pool.
            */
            solusd_loss_per_unit_staked = solusd_loss_numerator / total_solusd_deposits + 1;
            self.last_solusd_loss_error_offset = solusd_loss_per_unit_staked * total_solusd_deposits - solusd_loss_numerator;
        }

        sol_gain_per_unit_staked = sol_numerator / total_solusd_deposits;
        self.last_sol_error_offset = sol_numerator - sol_gain_per_unit_staked * total_solusd_deposits;

        (sol_gain_per_unit_staked, solusd_loss_per_unit_staked)
    }
    pub fn compute_solid_per_unit_staked(&mut self,solid_issuance:u128,total_solusd:u128)->u128{
        /*  
        * Calculate the SOLID-per-unit staked.  Division uses a "feedback" error correction, to keep the 
        * cumulative error low in the running total G:
        *
        * 1) Form a numerator which compensates for the floor division error that occurred the last time this 
        * function was called.  
        * 2) Calculate "per-unit-staked" ratio.
        * 3) Multiply the ratio back by its denominator, to reveal the current floor division error.
        * 4) Store this error for use in the next correction when this function is called.
        * 5) Note: static analysis tools complain about this "division before multiplication", however, it is intended.
        */
        let solid_numerator = solid_issuance * DECIMAL_PRECISION + self.last_solid_error;

        let solid_per_unit_staked = solid_numerator / total_solusd;

        self.last_solid_error = solid_numerator - (solid_per_unit_staked * total_solusd);

        return solid_per_unit_staked;
    }

    // --- Reward calculator functions for depositor and front end ---

    /* Calculates the SOL gain earned by the deposit since its last snapshots were taken.
    * Given by the formula:  E = d0 * (S - S(0))/P(0)
    * where S(0) and P(0) are the depositor's snapshots of the sum S and product P, respectively.
    * d0 is the last recorded deposit value.
    */
    pub fn get_depositor_sol_gain(&self, initial_deposit:u128, snapshots:&Snapshots) -> u128 {
        if initial_deposit == 0 {
            return 0;
        }
        let sol_gain = self.get_sol_gain_from_snapshots(initial_deposit, snapshots);
        return sol_gain;
    }

    /*
    * Return the SOLID gain earned by the front end. Given by the formula:  E = D0 * (G - G(0))/P(0)
    * where G(0) and P(0) are the depositor's snapshots of the sum G and product P, respectively.
    *
    * D0 is the last recorded value of the front end's total tagged deposits.
    */
    pub fn get_frontend_solid_gain(&self, snapshots:&Snapshots, frontend:&FrontEnd)->u128{
        let frontend_stake = frontend.frontend_stake;
        if frontend_stake == 0 {
            return 0;
        }

        let kickback_rate = frontend.kickback_rate;
        let frontend_share = DECIMAL_PRECISION - kickback_rate;

        let solid_gain = frontend_share * self.get_solid_gain_from_snapshots(frontend_stake, snapshots);
        return solid_gain;
    }

    /*
    * Calculate the SOLID gain earned by a deposit since its last snapshots were taken.
    * Given by the formula:  SOLID = d0 * (G - G(0))/P(0)
    * where G(0) and P(0) are the depositor's snapshots of the sum G and product P, respectively.
    * d0 is the last recorded deposit value.
    */
    pub fn get_depositor_solid_gain(&self, snapshots:&Snapshots, user_deposit:&Deposit, frontend:&FrontEnd)->u128{
        let initial_deposit = user_deposit.initial_value;
        if initial_deposit == 0 {
            return 0;
        }
        let frontend_tag = user_deposit.front_end_tag;

        /*
        * If not tagged with a front end, the depositor gets a 100% cut of what their deposit earned.
        * Otherwise, their cut of the deposit's earnings is equal to the kickbackRate, set by the front end through
        * which they made their deposit.
        */
        let kickback_rate = if frontend_tag == Pubkey::from_str(ZERO_ADDRESS).unwrap() {DECIMAL_PRECISION} else {frontend.kickback_rate};

        let solid_gain = kickback_rate * self.get_solid_gain_from_snapshots(initial_deposit, snapshots);
        return solid_gain;

    }
    pub fn get_sol_gain_from_snapshots(&self, initial_deposit:u128, snapshots:&Snapshots) ->u128{
        /*
        * Grab the sum 'S' from the epoch at which the stake was made. The SOL gain may span up to one scale change.
        * If it does, the second portion of the SOL gain is scaled by 1e9.
        * If the gain spans no scale change, the second portion will be 0.
        */
        let epoch_snapshot = snapshots.epoch;
        let scale_snapshot = snapshots.scale;

        let s_snapshot = snapshots.s;
        let p_snapshot = snapshots.p;

        let first_portion = 0;//epochToScaleToSum[epochSnapshot][scaleSnapshot].sub(S_Snapshot);
        let second_portion = 0;//epochToScaleToSum[epochSnapshot][scaleSnapshot.add(1)].div(SCALE_FACTOR);

        let sol_gain = initial_deposit * (first_portion + second_portion) / p_snapshot / DECIMAL_PRECISION;
        return sol_gain;
    }
    pub fn get_solid_gain_from_snapshots(&self, initial_deposit:u128, snapshots:&Snapshots) ->u128{
        /*
        * Grab the sum 'S' from the epoch at which the stake was made. The SOLID gain may span up to one scale change.
        * If it does, the second portion of the SOLID gain is scaled by 1e9.
        * If the gain spans no scale change, the second portion will be 0.
        */
        let epoch_snapshot = snapshots.epoch;
        let scale_snapshot = snapshots.scale;

        let g_snapshot = snapshots.g;
        let p_snapshot = snapshots.p;

        let first_portion = 0;//epochToScaleToSum[epochSnapshot][scaleSnapshot].sub(S_Snapshot);
        let second_portion = 0;//epochToScaleToSum[epochSnapshot][scaleSnapshot.add(1)].div(SCALE_FACTOR);

        let solid_gain = initial_deposit * (first_portion + second_portion) / p_snapshot / DECIMAL_PRECISION;
        return solid_gain;
    }
    pub fn get_compounded_solusd_deposit(&self, initial_deposit:u128, snapshots:&Snapshots) -> u128 {
        if initial_deposit == 0 {
            return 0;
        }
        let compounded_deposit = self.get_compounded_stake_from_snapshots(initial_deposit, snapshots);
        return compounded_deposit;
    }
    /*
    * Return the front end's compounded stake. Given by the formula:  D = D0 * P/P(0)
    * where P(0) is the depositor's snapshot of the product P, taken at the last time
    * when one of the front end's tagged deposits updated their deposit.
    *
    * The front end's compounded stake is equal to the sum of its depositors' compounded deposits.
    */
    pub fn get_compounded_frontend_stake(&self, frontend:&FrontEnd, snapshots:&Snapshots)->u128{
        let frontend_stake = frontend.frontend_stake;
        if frontend_stake == 0 {
            return 0;
        }
        let compounded_frontend_stake = self.get_compounded_stake_from_snapshots(frontend_stake, snapshots);
        return compounded_frontend_stake;

    }
    pub fn get_compounded_stake_from_snapshots(&self, initial_stake:u128, snapshots:&Snapshots) -> u128{
        let snapshot_p = snapshots.p;
        let scale_snapshot = snapshots.scale;
        let epoch_snapshot = snapshots.epoch;

        // If stake was made before a pool-emptying event, then it has been fully cancelled with debt -- so, return 0
        if epoch_snapshot < self.current_epoch {
            return 0;
        }

        let scale_diff = self.current_scale - scale_snapshot;

        /* Compute the compounded stake. If a scale change in P was made during the stake's lifetime,
        * account for it. If more than one scale change was made, then the stake has decreased by a factor of
        * at least 1e-9 -- so return 0.
        */
        let mut compounded_stake = 0;
        if scale_diff == 0 {
            compounded_stake = initial_stake * self.p / snapshot_p;
        }
        else if scale_diff == 1 {
            compounded_stake = initial_stake * self.p / snapshot_p / SCALE_FACTOR;
        } else { // if scale_diff >= 2
            compounded_stake = 0;
        }

        /*
        * If compounded deposit is less than a billionth of the initial deposit, return 0.
        *
        * NOTE: originally, this line was in place to stop rounding errors making the deposit too large. However, the error
        * corrections should ensure the error in P "favors the Pool", i.e. any given compounded deposit should slightly less
        * than it's theoretical value.
        *
        * Thus it's unclear whether this line is still really needed.
        */
        if compounded_stake < (initial_stake / SCALE_FACTOR) {
            return 0;
        }
        return compounded_stake;
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct FrontEnd {
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
    pub fn update_snapshots_with_frontendstake(&mut self,new_value:u128, pool_data:&StabilityPool){
        if new_value == 0{
            return;
        }

        let current_scale_cached = pool_data.current_scale;
        let current_epoch_cached = pool_data.current_epoch;
        let current_p = pool_data.p;

        // Get G for the current epoch and current scale
        let current_g = 0;//epochToScaleToG[currentEpochCached][currentScaleCached];

        // Record new snapshots of the latest running product p and sum g for the front end
        self.p = current_p;
        self.g = current_g;
        self.scale = current_scale_cached;
        self.epoch = current_epoch_cached;

    }
    pub fn update_snapshots_with_deposit(&mut self,new_value:u128, pool_data:&StabilityPool){
        if new_value == 0{
            return;
        }

        let current_scale_cached = pool_data.current_scale;
        let current_epoch_cached = pool_data.current_epoch;
        let current_p = pool_data.p;

        // Get S and G for the current epoch and current scale
        let current_s = 0;//epochToScaleToSum[currentEpochCached][currentScaleCached];
        let current_g = 0;//epochToScaleToG[currentEpochCached][currentScaleCached];

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
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub debt:u128,
    pub coll:u128,
    pub stake:u128,
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

    pub sol:u128,
    pub solusd_debt:u128,
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
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
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
    pub fn increase_solusd_debt(&mut self, amount:u128){
        self.solusd_debt += amount;
    }
    pub fn decrease_solusd_debt(&mut self, amount:u128){
        self.solusd_debt -= amount;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct DefaultPool {
    pub trove_manager_address: Pubkey,
    pub active_pool_address: Pubkey,
    pub sol: u128,
    pub solusd_debt: u128,
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
    pub fn increase_solusd_debt(&mut self, amount:u128){
        self.solusd_debt += amount;
    }
    pub fn decrease_solusd_debt(&mut self, amount:u128){
        self.solusd_debt -= amount;
    }
}


/// BorrowerOperations struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BorrowerOperations {
    pub nonce: u8,
    pub token_program_pubkey: Pubkey,
    pub trove_manager_id: Pubkey,
    pub active_pool_id: Pubkey,
    pub default_pool_id: Pubkey,
    pub stability_pool_id: Pubkey,
    pub gas_pool_id: Pubkey,
    pub coll_surplus_pool_id: Pubkey,
    pub solusd_token_id: Pubkey,
    pub solid_staking_id: Pubkey,

    pub oracle_program_id: Pubkey,
    pub pyth_product_id: Pubkey,
    pub pyth_price_id: Pubkey,
    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

    
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesAdjustTrove {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub price:u128,
    pub coll_change:u128,
    pub net_debt_change:u128,
    pub is_coll_increase:u8,
    pub debt:u128,
    pub coll:u128,
    pub old_icr:u128,
    pub new_icr:u128,
    pub new_tcr:u128,
    pub solusd_fee:u128,
    pub new_debt:u128,
    pub new_coll:u128,
    pub stake:u128,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct LocalVariablesOpenTrove {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    pub price:u128,
    pub solusd_fee:u128,
    pub new_debt:u128,
    pub composit_debt:u128,
    pub icr:u128,
    pub nicr:u128,
    pub stake:u128,
    pub array_index:u128,
}
impl LocalVariablesOpenTrove {
    pub fn new(pool_id_pubkey:Pubkey, owner_pubkey: Pubkey)->LocalVariablesOpenTrove{
        LocalVariablesOpenTrove{
            pool_id_pubkey,
            owner_pubkey,
            price:0,
            solusd_fee:0,
            new_debt:0,
            composit_debt:0,
            icr:0,
            nicr:0,
            stake:0,
            array_index:0,
        }
    }
}

/// Community Issuance struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct CommunityIssuance {
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// spl-token program pubkey
    pub token_program_pubkey: Pubkey,

    /// SOLID token account pubkey
    pub solid_token_pubkey: Pubkey,

    /// stability pool account pubkey
    pub stability_pool_pubkey: Pubkey,

    pub total_solid_issued: u128,

    pub deployment_time:u128,
}
impl CommunityIssuance {
    pub fn issue_solid(&mut self, cur_timestamp:u128) -> u128 {
        let cumulative_issuance_fraction = self.get_cumulative_issuance_fraction(cur_timestamp);
        let latest_total_solid_issued = u128::try_from(SOLID_SUPPLY_CAP * (cumulative_issuance_fraction as u128) / (DECIMAL_PRECISION as u128)).unwrap();
        let issuance = latest_total_solid_issued - self.total_solid_issued;
        self.total_solid_issued = latest_total_solid_issued;
        return issuance;
    }
    pub fn get_cumulative_issuance_fraction(&self, cur_timestamp:u128) -> u128 {
        // Get the time passed since deployment
        let time_passed_in_minutes:u32 = u32::try_from(( cur_timestamp - self.deployment_time ) / SECONDS_IN_ONE_MINUTE).unwrap();

        // f^t
        let power = u128::pow(ISSUANCE_FACTOR,time_passed_in_minutes);

        //  (1 - f^t)
        let cumulative_issuance_fraction = DECIMAL_PRECISION - power;

        if cumulative_issuance_fraction > DECIMAL_PRECISION {
            return 0;
        }

        return cumulative_issuance_fraction;
    }
}


/// SOLID Staking struct
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct SOLIDStaking {
    /// nonce is used to authorize this farm pool
    pub nonce: u8,

    /// spl-token program pubkey
    pub token_program_pubkey: Pubkey,

    /// SOLID pool token account
    pub solid_pool_token_pubkey: Pubkey,

    /// TroveManager account
    pub trove_manager_id: Pubkey,

    /// BorrwoerOperations account
    pub borrower_operations_id: Pubkey,

    /// ActivePool account
    pub active_pool_id: Pubkey,

    /// total staked SOLID amount
    pub total_staked_amount:u64,

    /// Running sum of SOL fees per-SOLID-staked
    pub f_sol:u64,

    /// Running sum of SOLID fees per-SOLID-staked
    pub f_solusd:u64,
}

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct UserDeposit {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// deposited amount
    pub deposit_amount:u64,
}


#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Snapshot {
    /// pool pubkey
    pub pool_id_pubkey:Pubkey,

    /// owner pubkey
    pub owner_pubkey:Pubkey,

    /// SOL snapshot
    pub f_sol_snapshot:u64,

    /// solUSD snapshot
    pub f_solusd_snapshot:u64,
}