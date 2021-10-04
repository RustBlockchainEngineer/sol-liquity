//! Program state processor
//! In here, All instructions are processed by Processor

use {
    liquity_common::{
        error::LiquityError,
        state::{
            TroveManager, 
            Trove, 
            RewardSnapshot, 
            LocalVariablesOuterLiquidationFunction,
            LocalVariablesInnerSingleLiquidateFunction,
            LocalVariablesLiquidationSequence,
            LiquidationValues,
            LiquidationTotals,
            ContractsCache,
            RedemptionTotals,
            SingleRedemptionValues,
            ActivePool,
            DefaultPool,
            Status,
            StabilityPool,
            CommunityIssuance,
            CollSurplusPool,
            EpochToScale
        },
        constant::{
            DECIMAL_PRECISION,
            MCR,
            CCR,
            PERCENT_DIVISOR,
            SOLUSD_GAS_COMPENSATION,
            _100PCT,
            MINUTE_DECAY_FACTOR,
            REDEMPTION_FEE_FLOOR,
            SECONDS_IN_ONE_MINUTE,
            BETA
        },
        liquity_math::{
            compute_cr,
            dec_pow,
            min,
            max
        },
        pyth,
        math::{Decimal,  TryDiv, TryMul},
        utils::*,
    },
    crate::{
        instruction::{TroveManagerInstruction},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    num_traits::FromPrimitive,
    num_derive::FromPrimitive, 
    solana_program::{
        account_info::{
            next_account_info,
            AccountInfo,
        },
        borsh::try_from_slice_unchecked,
        decode_error::DecodeError,
        entrypoint::ProgramResult,
        msg,
        program::{ invoke_signed},
        program_error::PrintProgramError,
        program_error::ProgramError,
        pubkey::Pubkey,
        clock::Clock,
        sysvar::Sysvar,
        program_pack::Pack,
    },
    spl_token::state::Mint, 
};
use std::convert::TryInto;
use std::str::FromStr;

/// Program state handler.
/// Main logic of this program
pub struct Processor {}
impl Processor {  
    /// All instructions start from here and are processed by their type.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = TroveManagerInstruction::try_from_slice(input)?;

        // determine instruction type
        match instruction {
            TroveManagerInstruction::Initialize{
                nonce,
            } => {
                // Instruction: Initialize
                Self::process_initialize(program_id, accounts, nonce)
            }
            TroveManagerInstruction::ApplyPendingRewards => {
                Self::process_apply_pending_rewards(program_id, accounts)
            }
            TroveManagerInstruction::Liquidate => {
                // Instruction: Initialize
                Self::process_liquidate(program_id, accounts)
            }
            TroveManagerInstruction::RedeemCollateral{
                solusd_amount,
                partial_redemption_hint_nicr,
                max_iterations,
                max_fee_percentage,
                total_sol_drawn,
                total_solusd_to_redeem
            } => {
                // Instruction: Initialize
                Self::process_redeem_collateral(program_id, accounts, solusd_amount, partial_redemption_hint_nicr, max_iterations, max_fee_percentage, total_sol_drawn,total_solusd_to_redeem)
            }
            TroveManagerInstruction::LiquidateTroves(number) => {
                // Instruction: Initialize
                Self::process_liquidate_troves(program_id, accounts, number)
            }
        }
    }

    /// process `Initialize` instruction.
    pub fn process_initialize(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        nonce: u8,                  // nonce for authorizing
    ) -> ProgramResult {
        // start initializeing this SOLID staking pool ...

        // get all account informations from accounts array by using iterator
        let account_info_iter = &mut accounts.iter();
        
        let trove_manager_id_info = next_account_info(account_info_iter)?;
        let default_pool_id_info = next_account_info(account_info_iter)?;
        let active_pool_id_info = next_account_info(account_info_iter)?;
        let stability_pool_id_info = next_account_info(account_info_iter)?;
        let gas_pool_id_info = next_account_info(account_info_iter)?;
        let coll_surplus_pool_id_info = next_account_info(account_info_iter)?;
        let borrow_operations_id_info = next_account_info(account_info_iter)?;
        let oracle_program_id_info = next_account_info(account_info_iter)?;
        let pyth_product_id_info = next_account_info(account_info_iter)?;
        let pyth_price_id_info = next_account_info(account_info_iter)?;
        let solusd_token_id_info = next_account_info(account_info_iter)?;
        let solid_staking_id_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, trove_manager_id_info.key, nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&trove_manager_id_info.data.borrow())?;

        trove_manager_data.borrower_operations_id = *borrow_operations_id_info.key;
        trove_manager_data.default_pool_id = *default_pool_id_info.key;
        trove_manager_data.active_pool_id = *active_pool_id_info.key;
        trove_manager_data.stability_pool_id = *stability_pool_id_info.key;
        trove_manager_data.gas_pool_id = *gas_pool_id_info.key;
        trove_manager_data.coll_surplus_pool_id = *coll_surplus_pool_id_info.key;
        trove_manager_data.oracle_program_id = *oracle_program_id_info.key;
        trove_manager_data.pyth_product_id = *pyth_product_id_info.key;
        trove_manager_data.pyth_price_id = *pyth_price_id_info.key;
        trove_manager_data.solusd_token_pubkey = *solusd_token_id_info.key;
        trove_manager_data.solid_staking_pubkey = *solid_staking_id_info.key;
        trove_manager_data.token_program_id = *token_program_info.key;

        Ok(())
    } 
    /// process `ApplyPendingRewards` instruction.
    pub fn process_apply_pending_rewards(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let trove_manager_id_info = next_account_info(account_info_iter)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        let reward_snapshots_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let caller_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;

        let trove_manager_data = try_from_slice_unchecked::<TroveManager>(&mut trove_manager_id_info.data.borrow())?;
        let mut borrower_trove = try_from_slice_unchecked::<Trove>(&borrower_trove_info.data.borrow())?;
        let mut reward_snapshot = try_from_slice_unchecked::<RewardSnapshot>(&reward_snapshots_info.data.borrow())?;

        if *caller_info.key != trove_manager_data.borrower_operations_id {
            return Err(LiquityError::InvalidBorrwerOperations.into());
        }

        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow()).unwrap();
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow()).unwrap();

        apply_pending_rewards(
            &trove_manager_data, 
            &mut borrower_trove,
            &mut reward_snapshot, 
            &mut default_pool_data, 
            &mut active_pool_data
        );

        Ok(())
    } 

    /*
    * Attempt to liquidate a custom list of troves provided by the caller.
    */
    pub fn process_liquidate(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let trove_manager_id_info = next_account_info(account_info_iter)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let reward_snapshots_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let community_issuance_id_info = next_account_info(account_info_iter)?;
        let epoch_to_scale_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let cur_timestamp = clock.unix_timestamp as u64;

        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&mut trove_manager_id_info.data.borrow())?;
        let mut borrower_trove = try_from_slice_unchecked::<Trove>(&borrower_trove_info.data.borrow())?;
        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow())?;
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        let mut coll_surplus_pool_data = try_from_slice_unchecked::<CollSurplusPool>(&coll_surplus_pool_info.data.borrow())?;
        let mut reward_snapshots_data = try_from_slice_unchecked::<RewardSnapshot>(&reward_snapshots_info.data.borrow())?;
        let mut stability_pool_data = try_from_slice_unchecked::<StabilityPool>(&stability_pool_info.data.borrow())?;
        let mut community_issuance_data = try_from_slice_unchecked::<CommunityIssuance>(&community_issuance_id_info.data.borrow())?;
        let mut epoch_to_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_scale_info.data.borrow())?;

        if !borrower_trove.is_active() {
            return Err(LiquityError::TroveNotActive.into());
        }

        let market_price = get_market_price(
            stability_pool_data.oracle_program_id,
            stability_pool_data.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;

        let mut vars = LocalVariablesOuterLiquidationFunction{
            price:0,
            solusd_in_stab_pool:0,
            recovery_mode_at_start:0,
            liquidated_debt:0,
            liquidated_coll:0,
        };

        vars.price = market_price;
        vars.solusd_in_stab_pool = stability_pool_data.total_sol_usd_deposits as u128;
        vars.recovery_mode_at_start = trove_manager_data.check_recovery_mode(vars.price, &active_pool_data, &default_pool_data);
        let mut totals = LiquidationTotals::new();
        
        // Perform the appropriate liquidation sequence - tally values and obtain their totals.
        if vars.recovery_mode_at_start == 1 {
            
            totals = get_total_from_batch_liquidate_recovery_mode(
                &mut trove_manager_data, 
                &mut active_pool_data, 
                &mut default_pool_data, 
                vars.price, 
                vars.solusd_in_stab_pool, 
                borrower_info.key, 
                &mut borrower_trove, 
                &mut reward_snapshots_data);
        }
        else {//  if !vars.recoveryModeAtStart
            totals = get_total_from_batch_liquidate_normal_mode(
                &mut trove_manager_data, 
                &mut active_pool_data, 
                &mut default_pool_data, 
                vars.price, 
                vars.solusd_in_stab_pool, 
                borrower_info.key, 
                &mut borrower_trove, 
                &mut reward_snapshots_data);
        }

        if totals.total_debt_in_sequence <= 0 {
            return Err(LiquityError::NothingToLiquidate.into());
        }

        // Move liquidated SOL and SOLUSD to the appropriate pools
        //stabilityPoolCached.offset(totals.totalDebtToOffset, totals.totalCollToSendToSP); -- implemented
        stability_pool_data.offset(totals.total_debt_to_offset, totals.total_coll_to_send_to_sp, community_issuance_data.issue_solid(cur_timestamp as u128), &mut active_pool_data, &mut epoch_to_scale);
        redistribute_debt_and_coll(&mut trove_manager_data, &mut active_pool_data, &mut default_pool_data, totals.total_debt_to_redistribute, totals.total_coll_to_redistribute);

        if totals.total_coll_surplus > 0 {
            //activePoolCached.sendETH(address(collSurplusPool), totals.totalCollSurplus); -- implemented
            active_pool_data.sol -= totals.total_coll_surplus;
            coll_surplus_pool_data.sol += totals.total_coll_surplus;
        }

        // update system snapshots
        update_system_snapshots_exclude_coll_reminder(&mut trove_manager_data, &active_pool_data, &default_pool_data,totals.total_coll_gas_compensation);

        vars.liquidated_debt = totals.total_debt_in_sequence;
        vars.liquidated_coll = totals.total_coll_in_sequence - totals.total_coll_gas_compensation - totals.total_coll_surplus;

        // Send gas compensation to caller

        // _sendGasCompensation(activePoolCached, msg.sender, totals.totalLUSDGasCompensation, totals.totalCollGasCompensation);

        Ok(())
    }
    /* Send solusd_amount SOLUSD to the system and redeem the corresponding amount of collateral from as many Troves as are needed to fill the redemption
    * request.  Applies pending rewards to a Trove before reducing its debt and coll.
    *
    * Note that if _amount is very large, this function can run out of gas, specially if traversed troves are small. This can be easily avoided by
    * splitting the total _amount in appropriate chunks and calling the function multiple times.
    *
    * Param `_maxIterations` can also be provided, so the loop through Troves is capped (if it’s zero, it will be ignored).This makes it easier to
    * avoid OOG for the frontend, as only knowing approximately the average cost of an iteration is enough, without needing to know the “topology”
    * of the trove list. It also avoids the need to set the cap in stone in the contract, nor doing gas calculations, as both gas price and opcode
    * costs can vary.
    *
    * All Troves that are redeemed from -- with the likely exception of the last one -- will end up with no debt left, therefore they will be closed.
    * If the last Trove does have some remaining debt, it has a finite ICR, and the reinsertion could be anywhere in the list, therefore it requires a hint.
    * A frontend should use getRedemptionHints() to calculate what the ICR of this Trove will be after redemption, and pass a hint for its position
    * in the sortedTroves list along with the ICR value that the hint was found for.
    *
    * If another transaction modifies the list between calling getRedemptionHints() and passing the hints to redeemCollateral(), it
    * is very likely that the last (partially) redeemed Trove would end up with a different ICR than what the hint is for. In this case the
    * redemption will stop after the last completely redeemed Trove and the sender will keep the remaining SOLUSD amount, which they can attempt
    * to redeem later.
    */
    pub fn process_redeem_collateral(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        solusd_amount:u128,
        partial_redemption_hint_nicr:u128,
        max_iterations:u128,
        max_fee_percentage:u128,

        total_sol_drawn: u128,
        total_solusd_to_redeem:u128,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let trove_manager_id_info = next_account_info(account_info_iter)?;
        let solid_staking_id_info = next_account_info(account_info_iter)?;
        let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let cur_timestamp = clock.unix_timestamp as u128;

        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&mut trove_manager_id_info.data.borrow())?;
        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow())?;
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        let stability_pool_data = try_from_slice_unchecked::<StabilityPool>(&stability_pool_info.data.borrow())?;

        if max_fee_percentage < REDEMPTION_FEE_FLOOR || max_fee_percentage > DECIMAL_PRECISION {
            return Err(LiquityError::MaxFeePercentageError.into());
        }
        //_requireAfterBootstrapPeriod();

        let market_price = get_market_price(
            stability_pool_data.oracle_program_id,
            stability_pool_data.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;

        let mut totals = RedemptionTotals::new();
        
        totals.price = market_price;
        
        let tcr = get_tcr(totals.price, &active_pool_data, &default_pool_data, &trove_manager_data);
        if tcr < MCR {
            return Err(LiquityError::TCRError.into());
        }

        if solusd_amount <= 0 {
            return Err(LiquityError::ZeroAmount.into());
        }

        //_requireLUSDBalanceCoversRedemption(contractsCache.lusdToken, msg.sender, _LUSDamount);

        totals.total_solusd_supply_at_start = active_pool_data.solusd_debt + default_pool_data.solusd_debt;

        // Confirm redeemer's balance is less than total SOLUSD supply
        //assert(contractsCache.lusdToken.balanceOf(msg.sender) <= totals.totalLUSDSupplyAtStart);

        totals.remaining_solusd = solusd_amount;

        /*
        address currentBorrower;

        if (_isValidFirstRedemptionHint(contractsCache.sortedTroves, _firstRedemptionHint, totals.price)) {
            currentBorrower = _firstRedemptionHint;
        } else {
            currentBorrower = contractsCache.sortedTroves.getLast();
            // Find the first trove with ICR >= MCR
            while (currentBorrower != address(0) && getCurrentICR(currentBorrower, totals.price) < MCR) {
                currentBorrower = contractsCache.sortedTroves.getPrev(currentBorrower);
            }
        }

        // Loop through the Troves starting from the one with lowest collateral ratio until _amount of LUSD is exchanged for collateral
        if (_maxIterations == 0) { _maxIterations = uint(-1); }
        while (currentBorrower != address(0) && totals.remainingLUSD > 0 && _maxIterations > 0) {
            _maxIterations--;
            // Save the address of the Trove preceding the current one, before potentially modifying the list
            address nextUserToCheck = contractsCache.sortedTroves.getPrev(currentBorrower);

            _applyPendingRewards(contractsCache.activePool, contractsCache.defaultPool, currentBorrower);

            SingleRedemptionValues memory singleRedemption = _redeemCollateralFromTrove(
                contractsCache,
                currentBorrower,
                totals.remainingLUSD,
                totals.price,
                _upperPartialRedemptionHint,
                _lowerPartialRedemptionHint,
                _partialRedemptionHintNICR
            );

            if (singleRedemption.cancelledPartial) break; // Partial redemption was cancelled (out-of-date hint, or new net debt < minimum), therefore we could not redeem from the last Trove

            totals.totalLUSDToRedeem  = totals.totalLUSDToRedeem.add(singleRedemption.LUSDLot);
            totals.totalETHDrawn = totals.totalETHDrawn.add(singleRedemption.ETHLot);

            totals.remainingLUSD = totals.remainingLUSD.sub(singleRedemption.LUSDLot);
            currentBorrower = nextUserToCheck;
        } 
        -------------implemented below
        */
        totals.total_sol_drawn = total_sol_drawn;
        totals.total_solusd_to_redeem = total_solusd_to_redeem;
        totals.remaining_solusd = 0;

        if totals.total_sol_drawn <= 0 {
            return Err(LiquityError::ZeroAmount.into());
        }

        // Decay the baseRate due to time passed, and then increase it according to the size of this redemption.
        // Use the saved total SOLUSD supply value, from before it was reduced by the redemption.
        update_base_rate_from_redemption(&mut trove_manager_data, cur_timestamp, totals.total_sol_drawn, totals.price, totals.total_solusd_supply_at_start);

        // calculate the sol fee
        totals.sol_fee = get_redemption_fee(&trove_manager_data, totals.total_sol_drawn);

        if totals.sol_fee * DECIMAL_PRECISION / totals.total_sol_drawn > max_fee_percentage {
            return Err(LiquityError::FeeExceeded.into());
        }

        // send the sol fee to the SOLID staking contract
        //contractsCache.activePool.sendETH(address(contractsCache.lqtyStaking), totals.ETHFee);
        //contractsCache.lqtyStaking.increaseF_ETH(totals.ETHFee);

        totals.sol_to_send_to_redeemer = totals.total_sol_drawn - totals.sol_fee;

        // Burn the total LUSD that is cancelled with debt, and send the redeemed ETH to msg.sender
        //contractsCache.lusdToken.burn(msg.sender, totals.totalLUSDToRedeem);
        // Update Active Pool LUSD, and send ETH to account
        active_pool_data.decrease_solusd_debt(totals.total_solusd_to_redeem);
        //contractsCache.activePool.sendETH(msg.sender, totals.ETHToSendToRedeemer);

        Ok(())
    } 

    /*
    * Liquidate a sequence of troves. Closes a maximum number of n under-collateralized Troves,
    * starting from the one with the lowest collateral ratio in the system, and moving upwards
    */
    pub fn process_liquidate_troves(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        number: u128,                  // nonce for authorizing
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let trove_manager_id_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&mut trove_manager_id_info.data.borrow())?;
        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow())?;
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        let stability_pool_data = try_from_slice_unchecked::<StabilityPool>(&stability_pool_info.data.borrow())?;

        let market_price = get_market_price(
            stability_pool_data.oracle_program_id,
            stability_pool_data.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;

        let mut vars = LocalVariablesOuterLiquidationFunction::new();
        let mut totals = LiquidationTotals::new();

        vars.price = market_price;
        vars.solusd_in_stab_pool = stability_pool_data.total_sol_usd_deposits as u128;
        vars.recovery_mode_at_start = trove_manager_data.check_recovery_mode(vars.price, &active_pool_data, &default_pool_data);

        // Perform the appropriate liquidation sequence - tally the values, and obtain their totals
        if vars.recovery_mode_at_start == 1 {
            //totals = _getTotalsFromLiquidateTrovesSequence_RecoveryMode(contractsCache, vars.price, vars.LUSDInStabPool, _n);
        }
        else {// if !vars.recoveryModeAtStart
            //totals = _getTotalsFromLiquidateTrovesSequence_NormalMode(contractsCache.activePool, contractsCache.defaultPool, vars.price, vars.LUSDInStabPool, _n);
        }

        if totals.total_debt_in_sequence <= 0 {
            return Err(LiquityError::ZeroAmount.into());
        }
        // Move liquidated SOL and SOLUSD to the appropriate pools
        //stabilityPoolCached.offset(totals.totalDebtToOffset, totals.totalCollToSendToSP);
        redistribute_debt_and_coll(&mut trove_manager_data, &mut active_pool_data, &mut default_pool_data, totals.total_debt_to_redistribute, totals.total_coll_to_redistribute);

        if totals.total_coll_surplus > 0 {
            //contractsCache.activePool.sendETH(address(collSurplusPool), totals.totalCollSurplus);
        }
        update_system_snapshots_exclude_coll_reminder(&mut trove_manager_data, &active_pool_data, &default_pool_data, totals.total_coll_gas_compensation);

        vars.liquidated_debt = totals.total_debt_in_sequence;
        vars.liquidated_coll = totals.total_coll_in_sequence - totals.total_coll_gas_compensation - totals.total_coll_surplus;

        // Send gas compensation to caller
        //_sendGasCompensation(contractsCache.activePool, msg.sender, totals.totalLUSDGasCompensation, totals.totalCollGasCompensation);

        Ok(())
    }
    
    
}
