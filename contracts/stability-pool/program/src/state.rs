//! State transition types
//! State stores account data and manage version upgrade

#![allow(clippy::too_many_arguments)]
use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        pubkey::{Pubkey},
    },
};
use crate::{
    constant::{
        DECIMAL_PRECISION,
        SCALE_FACTOR,
        ZERO_ADDRESS,
    }
};
use std::str::FromStr;


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
    pub total_sol_usd_deposits: u64,

    // Error tracker for the error correction in the SOLID issuance calculation
    pub last_solid_error:u64,

    // Error trackers for the error correction in the offset calculation
    pub last_sol_error_offset:u64,

    pub last_solusd_loss_error_offset:u64,

    pub p:u64,

    pub current_scale:u128,

    pub current_epoch:u128,

    // deposited sol tracker
    pub sol: u64,

    /// Oracle (Pyth) program id
    pub oracle_program_id: Pubkey,
    
    /// Currency market prices are quoted in
    /// e.g. "USD" null padded (`*b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"`) or a SPL token mint pubkey
    pub quote_currency: [u8; 32],

}
impl StabilityPool{
    pub fn trigger_solid_issuance(&mut self,solid_issuance:u64){
        self.update_g(solid_issuance);
    }
    pub fn update_g(&mut self,solid_issuance:u64){
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
    pub fn compute_solid_per_unit_staked(&mut self,solid_issuance:u64,total_solusd:u64)->u64{
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
    pub fn get_depositor_sol_gain(&self, initial_deposit:u64, snapshots:&Snapshots) -> u64 {
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
    pub fn get_frontend_solid_gain(&self, snapshots:&Snapshots, frontend:&FrontEnd)->u64{
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
    pub fn get_depositor_solid_gain(&self, snapshots:&Snapshots, user_deposit:&Deposit, frontend:&FrontEnd)->u64{
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
    pub fn get_sol_gain_from_snapshots(&self, initial_deposit:u64, snapshots:&Snapshots) ->u64{
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
    pub fn get_solid_gain_from_snapshots(&self, initial_deposit:u64, snapshots:&Snapshots) ->u64{
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
    pub fn get_compounded_solusd_deposit(&self, initial_deposit:u64, snapshots:&Snapshots) -> u64 {
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
    pub fn get_compounded_frontend_stake(&self, frontend:&FrontEnd, snapshots:&Snapshots)->u64{
        let frontend_stake = frontend.frontend_stake;
        if frontend_stake == 0 {
            return 0;
        }
        let compounded_frontend_stake = self.get_compounded_stake_from_snapshots(frontend_stake, snapshots);
        return compounded_frontend_stake;

    }
    pub fn get_compounded_stake_from_snapshots(&self, initial_stake:u64, snapshots:&Snapshots) -> u64{
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
    pub kickback_rate:u64,

    /// flag for registered frontend
    pub registered: u8,

    /// last recorded total deposits, tagged with that front end
    pub frontend_stake:u64,
}

impl FrontEnd {
    pub fn update_frontend_stake(&mut self,new_value:u64){
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
    pub initial_value:u64,

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
    pub s:u64,

    /// P
    pub p:u64,

    /// G
    pub g:u64,

    /// scale
    pub scale: u128,

    /// epoch
    pub epoch: u128,
}

impl Snapshots{
    pub fn update_snapshots_with_frontendstake(&mut self,new_value:u64, pool_data:&StabilityPool){
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
    pub fn update_snapshots_with_deposit(&mut self,new_value:u64, pool_data:&StabilityPool){
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