//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::TroveManagerError,
        instruction::{TroveManagerInstruction},
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
            Status
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
            MAX_BORROWING_FEE,
            SECONDS_IN_ONE_MINUTE,
            BETA
        },
        liquity_math::{
            compute_cr,
            dec_pow,
            min,
            max
        }
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
use stability_pool::{
    state::{
        StabilityPool
    },
    pyth,
    math::{Decimal, Rate, TryAdd, TryDiv, TryMul, WAD},
};

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
            } => {
                // Instruction: Initialize
                Self::process_redeem_collateral(program_id, accounts, solusd_amount, partial_redemption_hint_nicr, max_iterations, max_fee_percentage)
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
        let borrow_operations_id_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, trove_manager_id_info.key, nonce)? {
            return Err(TroveManagerError::InvalidProgramAddress.into());
        }

        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&trove_manager_id_info.data.borrow())?;

        trove_manager_data.borrower_operations_id = *borrow_operations_id_info.key;
        trove_manager_data.default_pool_id = *default_pool_id_info.key;
        trove_manager_data.active_pool_id = *active_pool_id_info.key;
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
            return Err(TroveManagerError::InvalidBorrwerOperations.into());
        }

        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow()).unwrap();
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow()).unwrap();

        Self::apply_pending_rewards(
            &trove_manager_data, 
            &mut borrower_trove,
            &mut reward_snapshot, 
            &mut default_pool_data, 
            &active_pool_data
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
        let active_pool_info = next_account_info(account_info_iter)?;
        let reward_snapshots_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&mut trove_manager_id_info.data.borrow())?;
        let mut borrower_trove = try_from_slice_unchecked::<Trove>(&borrower_trove_info.data.borrow())?;
        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow())?;
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        let mut reward_snapshots_data = try_from_slice_unchecked::<RewardSnapshot>(&reward_snapshots_info.data.borrow())?;
        let stability_pool_data = try_from_slice_unchecked::<StabilityPool>(&stability_pool_info.data.borrow())?;

        if !borrower_trove.is_active() {
            return Err(TroveManagerError::TroveNotActive.into());
        }

        let pyth_product_data = pyth_product_info.try_borrow_data()?;
        let pyth_product = pyth::load::<pyth::Product>(&pyth_product_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        if pyth_product.magic != pyth::MAGIC {
            msg!("Pyth product account provided is not a valid Pyth account");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
        if pyth_product.ver != pyth::VERSION_2 {
            msg!("Pyth product account provided has a different version than expected");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
        if pyth_product.atype != pyth::AccountType::Product as u32 {
            msg!("Pyth product account provided is not a valid Pyth product account");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
    
        let pyth_price_pubkey_bytes: &[u8; 32] = pyth_price_info
            .key
            .as_ref()
            .try_into()
            .map_err(|_| TroveManagerError::InvalidAccountInput)?;
        if &pyth_product.px_acc.val != pyth_price_pubkey_bytes {
            msg!("Pyth product price account does not match the Pyth price provided");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
    
        let market_price = Self::get_pyth_price(pyth_price_info, clock)?;

        let mut vars = LocalVariablesOuterLiquidationFunction{
            price:0,
            solusd_in_stab_pool:0,
            recovery_mode_at_start:0,
            liquidated_debt:0,
            liquidated_coll:0,
        };

        vars.price = market_price.try_round_u64().unwrap() as u128;
        vars.solusd_in_stab_pool = stability_pool_data.total_sol_usd_deposits as u128;
        vars.recovery_mode_at_start = trove_manager_data.check_recovery_mode(vars.price, &active_pool_data, &default_pool_data);
        let mut totals = LiquidationTotals::new();
        
        // Perform the appropriate liquidation sequence - tally values and obtain their totals.
        if vars.recovery_mode_at_start == 1 {
            
            totals = Self::get_total_from_batch_liquidate_recovery_mode(
                &mut trove_manager_data, 
                &active_pool_data, 
                &mut default_pool_data, 
                vars.price, 
                vars.solusd_in_stab_pool, 
                borrower_info.key, 
                &mut borrower_trove, 
                &mut reward_snapshots_data);
        }
        else {//  if !vars.recoveryModeAtStart
            totals = Self::get_total_from_batch_liquidate_normal_mode(
                &mut trove_manager_data, 
                &active_pool_data, 
                &mut default_pool_data, 
                vars.price, 
                vars.solusd_in_stab_pool, 
                borrower_info.key, 
                &mut borrower_trove, 
                &mut reward_snapshots_data);
        }

        if totals.total_debt_in_sequence <= 0 {
            return Err(TroveManagerError::NothingToLiquidate.into());
        }

        // Move liquidated SOL and SOLUSD to the appropriate pools
        //stabilityPoolCached.offset(totals.totalDebtToOffset, totals.totalCollToSendToSP);
        Self::redistribute_debt_and_coll(&mut trove_manager_data, &mut active_pool_data, &mut default_pool_data, totals.total_debt_to_redistribute, totals.total_coll_to_redistribute);

        if totals.total_coll_surplus > 0 {
            //activePoolCached.sendETH(address(collSurplusPool), totals.totalCollSurplus);
        }

        // update system snapshots
        Self::update_system_snapshots_exclude_coll_reminder(&mut trove_manager_data, &active_pool_data, &default_pool_data,totals.total_coll_gas_compensation);

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
        max_fee_percentage:u128
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

        if max_fee_percentage < REDEMPTION_FEE_FLOOR || max_fee_percentage > DECIMAL_PRECISION {
            return Err(TroveManagerError::MaxFeePercentageError.into());
        }
        //_requireAfterBootstrapPeriod();

        let pyth_product_data = pyth_product_info.try_borrow_data()?;
        let pyth_product = pyth::load::<pyth::Product>(&pyth_product_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        if pyth_product.magic != pyth::MAGIC {
            msg!("Pyth product account provided is not a valid Pyth account");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
        if pyth_product.ver != pyth::VERSION_2 {
            msg!("Pyth product account provided has a different version than expected");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
        if pyth_product.atype != pyth::AccountType::Product as u32 {
            msg!("Pyth product account provided is not a valid Pyth product account");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
    
        let pyth_price_pubkey_bytes: &[u8; 32] = pyth_price_info
            .key
            .as_ref()
            .try_into()
            .map_err(|_| TroveManagerError::InvalidAccountInput)?;
        if &pyth_product.px_acc.val != pyth_price_pubkey_bytes {
            msg!("Pyth product price account does not match the Pyth price provided");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
    
        let market_price = Self::get_pyth_price(pyth_price_info, clock)?;

        let mut totals = RedemptionTotals::new();
        
        totals.price = market_price.try_round_u64().unwrap() as u128;
        
        let tcr = Self::get_tcr(totals.price, &active_pool_data, &default_pool_data, &trove_manager_data);
        if tcr < MCR {
            return Err(TroveManagerError::TCRError.into());
        }

        if solusd_amount <= 0 {
            return Err(TroveManagerError::ZeroAmount.into());
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
        */

        if totals.total_sol_drawn <= 0 {
            return Err(TroveManagerError::ZeroAmount.into());
        }

        // Decay the baseRate due to time passed, and then increase it according to the size of this redemption.
        // Use the saved total SOLUSD supply value, from before it was reduced by the redemption.
        Self::update_base_rate_from_redemption(&mut trove_manager_data, cur_timestamp, totals.total_sol_drawn, totals.price, totals.total_solusd_supply_at_start);

        // calculate the sol fee
        totals.sol_fee = Self::get_redemption_fee(&trove_manager_data, totals.total_sol_drawn);

        if totals.sol_fee * DECIMAL_PRECISION / totals.total_sol_drawn > max_fee_percentage {
            return Err(TroveManagerError::FeeExceeded.into());
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
        let mut stability_pool_data = try_from_slice_unchecked::<StabilityPool>(&stability_pool_info.data.borrow())?;

        let pyth_product_data = pyth_product_info.try_borrow_data()?;
        let pyth_product = pyth::load::<pyth::Product>(&pyth_product_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        if pyth_product.magic != pyth::MAGIC {
            msg!("Pyth product account provided is not a valid Pyth account");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
        if pyth_product.ver != pyth::VERSION_2 {
            msg!("Pyth product account provided has a different version than expected");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
        if pyth_product.atype != pyth::AccountType::Product as u32 {
            msg!("Pyth product account provided is not a valid Pyth product account");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
    
        let pyth_price_pubkey_bytes: &[u8; 32] = pyth_price_info
            .key
            .as_ref()
            .try_into()
            .map_err(|_| TroveManagerError::InvalidAccountInput)?;
        if &pyth_product.px_acc.val != pyth_price_pubkey_bytes {
            msg!("Pyth product price account does not match the Pyth price provided");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }
    
        let market_price = Self::get_pyth_price(pyth_price_info, clock)?;

        let mut vars = LocalVariablesOuterLiquidationFunction::new();
        let mut totals = LiquidationTotals::new();

        vars.price = market_price.try_round_u64().unwrap() as u128;
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
            return Err(TroveManagerError::ZeroAmount.into());
        }
        // Move liquidated SOL and SOLUSD to the appropriate pools
        //stabilityPoolCached.offset(totals.totalDebtToOffset, totals.totalCollToSendToSP);
        Self::redistribute_debt_and_coll(&mut trove_manager_data, &mut active_pool_data, &mut default_pool_data, totals.total_debt_to_redistribute, totals.total_coll_to_redistribute);

        if totals.total_coll_surplus > 0 {
            //contractsCache.activePool.sendETH(address(collSurplusPool), totals.totalCollSurplus);
        }
        Self::update_system_snapshots_exclude_coll_reminder(&mut trove_manager_data, &active_pool_data, &default_pool_data, totals.total_coll_gas_compensation);

        vars.liquidated_debt = totals.total_debt_in_sequence;
        vars.liquidated_coll = totals.total_coll_in_sequence - totals.total_coll_gas_compensation - totals.total_coll_surplus;

        // Send gas compensation to caller
        //_sendGasCompensation(contractsCache.activePool, msg.sender, totals.totalLUSDGasCompensation, totals.totalCollGasCompensation);

        Ok(())
    }
    pub fn get_redemption_fee(trove_manager: &TroveManager, sol_drawn:u128)->u128{
        Self::calc_redemption_fee(Self::get_redemption_rate(trove_manager), sol_drawn)
    }
    pub fn calc_redemption_fee(redemption_rate: u128,sol_drawn: u128)->u128{
        let redemption_fee = redemption_rate * sol_drawn / DECIMAL_PRECISION;
        //require(redemptionFee < _ETHDrawn, "TroveManager: Fee would eat up all returned collateral");
        return redemption_fee;
    }
    pub fn get_redemption_rate(trove_manager:&TroveManager)->u128{
        Self::calc_redemption_rate(trove_manager.base_rate)
    }
    pub fn calc_redemption_rate(base_rate: u128)->u128{
        min(REDEMPTION_FEE_FLOOR + base_rate, DECIMAL_PRECISION)
    }
    // --- Redemption fee function ---

    /*
    * This function has two impacts on the baseRate state variable:
    * 1) decays the baseRate based on time passed since last redemption or LUSD borrowing operation.
    * then,
    * 2) increases the baseRate based on the amount redeemed, as a proportion of total supply
    */
    pub fn update_base_rate_from_redemption(trove_manager: &mut TroveManager, current_timestamp: u128, sol_drawn: u128, price: u128, total_solusd_supply: u128)->u128{
        let decayed_base_rate = Self::calc_decayed_base_rate(trove_manager, current_timestamp);

        /* Convert the drawn SOL back to SOLUSD at face value rate (1 SOLUSD:1 USD), in order to get
        * the fraction of total supply that was redeemed at face value. */
        let redeemed_solusd_fraction = sol_drawn * price / total_solusd_supply;

        let mut new_base_rate = decayed_base_rate + redeemed_solusd_fraction / BETA;
        new_base_rate = min(new_base_rate, DECIMAL_PRECISION);// cap baseRate at a maximum of 100%
        //assert(newBaseRate <= DECIMAL_PRECISION); // This is already enforced in the line above
        //assert(newBaseRate > 0); // Base rate is always non-zero after redemption

        // update the base_rate state variable
        trove_manager.base_rate = new_base_rate;
        Self::update_last_fee_op_time(trove_manager, current_timestamp);

        return new_base_rate;

    }
    pub fn update_last_fee_op_time(trove_manager: &mut TroveManager, current_timestamp: u128){
        let time_passed = current_timestamp - trove_manager.last_fee_operation_time;
        if time_passed >= SECONDS_IN_ONE_MINUTE {
            trove_manager.last_fee_operation_time = current_timestamp;
        }
    }
    pub fn calc_decayed_base_rate(trove_manager: &TroveManager, current_timestamp: u128)->u128{
        let minutes_passed = Self::minutes_passed_since_last_fee_op(trove_manager, current_timestamp);
        let decay_factor = dec_pow(MINUTE_DECAY_FACTOR, minutes_passed);
        return trove_manager.base_rate * decay_factor / DECIMAL_PRECISION;
    }
    
    pub fn minutes_passed_since_last_fee_op(trove_manager:&TroveManager, current_timestamp: u128)->u128{
        return (current_timestamp - trove_manager.last_fee_operation_time) / SECONDS_IN_ONE_MINUTE;
    }
    pub fn get_tcr(price: u128, active_pool:&ActivePool, default_pool:&DefaultPool, trove_manager:&TroveManager)->u128{
        let entire_system_debt = active_pool.solusd_debt + default_pool.solusd_debt;
        let entire_system_coll = active_pool.sol + default_pool.sol;
        
        let tcr = compute_cr(entire_system_coll, entire_system_debt, price);
        return tcr;
    }
    pub fn update_system_snapshots_exclude_coll_reminder(
        trove_manager:&mut TroveManager,
        active_pool:&ActivePool,
        default_pool:&DefaultPool,
        coll_remainder:u128
    ){
        trove_manager.total_stakes_snapshot = trove_manager.total_stakes;

        let active_coll = active_pool.sol;
        let liquidate_coll = default_pool.sol;

        trove_manager.total_collateral_snapshot = active_coll - coll_remainder + liquidate_coll;
    }
    pub fn redistribute_debt_and_coll(
        trove_manager:&mut TroveManager,
        active_pool:&mut ActivePool,
        default_pool:&mut DefaultPool,
        debt:u128,
        coll:u128
    ){
        if debt == 0 {
            return;
        }

        /*
        * Add distributed coll and debt rewards-per-unit-staked to the running totals. Division uses a "feedback"
        * error correction, to keep the cumulative error low in the running totals l_sol and l_solusd_debt:
        *
        * 1) Form numerators which compensate for the floor division errors that occurred the last time this
        * function was called.
        * 2) Calculate "per-unit-staked" ratios.
        * 3) Multiply each ratio back by its denominator, to reveal the current floor division error.
        * 4) Store these errors for use in the next correction when this function is called.
        * 5) Note: static analysis tools complain about this "division before multiplication", however, it is intended.
        */
        let sol_numerator = coll * DECIMAL_PRECISION + trove_manager.last_sol_error_redistribution;
        let solusd_debt_numerator = debt * DECIMAL_PRECISION + trove_manager.last_solusd_debt_error_redistribution;

        // Get the per-unit-staked terms
        let sol_reward_per_unit_staked = sol_numerator / trove_manager.total_stakes;
        let solusd_debt_reward_per_unit_staked = solusd_debt_numerator / trove_manager.total_stakes;

        trove_manager.last_sol_error_redistribution = sol_numerator - sol_reward_per_unit_staked * trove_manager.total_stakes;
        trove_manager.last_solusd_debt_error_redistribution = solusd_debt_numerator - solusd_debt_reward_per_unit_staked * trove_manager.total_stakes;

        // Add per-unit-staked terms to the running totals
        trove_manager.l_sol += sol_reward_per_unit_staked;
        trove_manager.l_solusd_debt += solusd_debt_reward_per_unit_staked;

        active_pool.decrease_solusd_debt(debt);
        default_pool.increase_solusd_debt(debt);
        //_activePool.sendETH(address(_defaultPool), _coll);

    }
    pub fn get_total_from_batch_liquidate_normal_mode(
        trove_manager_data:&mut TroveManager,
        active_pool:&ActivePool,
        default_pool:&mut DefaultPool,
        price:u128,
        solusd_in_stab_pool: u128,
        borrower_address:&Pubkey,
        borrower_trove:&mut Trove,
        reward_snapshot:&mut RewardSnapshot
    )->LiquidationTotals{
        let mut vars = LocalVariablesLiquidationSequence::new();
        let mut single_liquidation = LiquidationValues::new();

        vars.remaining_solusd_in_stab_pool = solusd_in_stab_pool;
        
        vars.user = Option::from(*borrower_address);

        let mut totals = LiquidationTotals::new();

        if borrower_trove.is_active() {
            vars.icr = Self::get_current_icr(trove_manager_data, borrower_trove, reward_snapshot, price);
            
            if vars.icr < MCR {
                single_liquidation = Self::liquidate_normal_mode(
                    trove_manager_data, 
                    active_pool, 
                    default_pool, 
                    borrower_trove, 
                    reward_snapshot, 
                    vars.remaining_solusd_in_stab_pool, 
                    );
            
                vars.remaining_solusd_in_stab_pool -= single_liquidation.debt_to_offset;

                // Add liquidation values to their respective running totals
                Self::add_liquidation_values_to_totals(&mut totals, &single_liquidation);
            }
        }

        totals
    }
    pub fn get_total_from_batch_liquidate_recovery_mode(
        trove_manager_data:&mut TroveManager,
        active_pool:&ActivePool,
        default_pool:&mut DefaultPool,
        price:u128,
        solusd_in_stab_pool: u128,
        borrower_address:&Pubkey,
        borrower_trove:&mut Trove,
        reward_snapshot:&mut RewardSnapshot
    )->LiquidationTotals{
        let mut vars = LocalVariablesLiquidationSequence::new();
        let mut single_liquidation = LiquidationValues::new();

        vars.remaining_solusd_in_stab_pool = solusd_in_stab_pool;
        vars.back_to_normal_mode = 0;
        vars.entire_system_debt = active_pool.solusd_debt + default_pool.solusd_debt;
        vars.entire_system_coll = active_pool.sol + default_pool.sol;
        vars.user = Option::from(*borrower_address);

        let mut totals = LiquidationTotals::new();

        if borrower_trove.is_active() {
            vars.icr = Self::get_current_icr(trove_manager_data, borrower_trove, reward_snapshot, price);
            
            if vars.back_to_normal_mode == 0 {
                if vars.icr < MCR || vars.remaining_solusd_in_stab_pool > 0 {
                    let tcr = compute_cr(vars.entire_system_coll, vars.entire_system_debt, price);
                    single_liquidation = Self::liquidate_recovery_mode(
                        trove_manager_data, 
                        active_pool, 
                        default_pool, 
                        borrower_trove, 
                        reward_snapshot, 
                        vars.icr, 
                        vars.remaining_solusd_in_stab_pool, 
                        tcr, 
                        price);
                    
                        // update aggregate trackers
                        vars.remaining_solusd_in_stab_pool -= single_liquidation.debt_to_offset;
                        vars.entire_system_debt -= single_liquidation.debt_to_offset;
                        vars.entire_system_coll -= vars.entire_system_coll - single_liquidation.coll_to_send_to_sp - single_liquidation.coll_gas_compensation - single_liquidation.coll_surplus;

                        // Add liquidation values to their respective running totals
                        
                        Self::add_liquidation_values_to_totals(&mut totals, &single_liquidation);
                        vars.back_to_normal_mode = Self::check_potential_not_recovery_mode(trove_manager_data, vars.entire_system_coll, vars.entire_system_debt, price);
                        
                }
                else if vars.back_to_normal_mode == 1 && vars.icr < MCR {
                    single_liquidation = Self::liquidate_normal_mode(trove_manager_data, active_pool, default_pool, borrower_trove, reward_snapshot, vars.remaining_solusd_in_stab_pool);
                    vars.remaining_solusd_in_stab_pool -= single_liquidation.debt_to_offset;

                    // Add liquidation values to their respective running totals
                    Self::add_liquidation_values_to_totals(&mut totals, &single_liquidation);

                }
            }
        }

        totals
    }
    
    pub fn check_potential_not_recovery_mode(trove_manager:&TroveManager, entire_system_coll:u128, entire_system_debt:u128, price:u128)->u8{
        let tcr = compute_cr(entire_system_coll, entire_system_debt, price);
        return if tcr < CCR {0} else {1};
    }
    pub fn add_liquidation_values_to_totals(totals:&mut LiquidationTotals, single_liquidation:&LiquidationValues){
        totals.total_coll_gas_compensation += single_liquidation.coll_gas_compensation;
        totals.total_solusd_gas_compensation += single_liquidation.solusd_gas_compensation;
        totals.total_debt_in_sequence += single_liquidation.entire_trove_debt;
        totals.total_coll_in_sequence += single_liquidation.entire_trove_coll;
        totals.total_debt_to_offset += single_liquidation.debt_to_offset;
        totals.total_coll_to_send_to_sp += single_liquidation.coll_to_send_to_sp;
        totals.total_debt_to_redistribute += single_liquidation.debt_to_redistribute;
        totals.total_coll_to_redistribute += single_liquidation.coll_to_redistribute;
        totals.total_coll_surplus += single_liquidation.coll_surplus;
    }
    pub fn liquidate_normal_mode(
        trove_manager: &mut TroveManager,
        active_pool:&ActivePool,
        default_pool:&mut DefaultPool,
        borrower_trove:&mut Trove,
        reward_snapshots:&mut RewardSnapshot,
        _solusd_in_stab_pool:u128,
    )->LiquidationValues{
        let mut vars = LocalVariablesInnerSingleLiquidateFunction::new();
        let mut single_liquidation = LiquidationValues::new();

        //if (TroveOwners.length <= 1) {return singleLiquidation;} // don't liquidate if last trove
        let (entire_trove_debt, entire_trove_coll, pending_debt_reward, pending_coll_reward) = Self::get_entire_debt_and_coll(trove_manager, borrower_trove, reward_snapshots);
        single_liquidation.entire_trove_debt = entire_trove_debt;
        single_liquidation.entire_trove_coll = entire_trove_coll;
        vars.pending_debt_reward = pending_debt_reward;
        vars.pending_coll_reward = pending_coll_reward;

        Self::move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
        Self::remove_stake(trove_manager,borrower_trove);

        single_liquidation.coll_gas_compensation = Self::get_coll_gas_compensation(single_liquidation.entire_trove_coll);
        single_liquidation.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;
        let coll_to_liquidate = single_liquidation.entire_trove_coll - single_liquidation.coll_gas_compensation;

        let (_debt_to_offset, _coll_to_send_to_sp, _debt_to_redistribute, _coll_to_liquidate) = Self::get_offset_and_redistribution_vals(single_liquidation.entire_trove_debt, coll_to_liquidate, _solusd_in_stab_pool);
        single_liquidation.debt_to_offset = _debt_to_offset;
        single_liquidation.coll_to_send_to_sp = _coll_to_send_to_sp;
        single_liquidation.debt_to_redistribute = _debt_to_redistribute;
        single_liquidation.coll_to_redistribute = _coll_to_liquidate;

        //_closeTrove(_borrower, Status.closedByLiquidation);
        return single_liquidation;

    }
    pub fn liquidate_recovery_mode(
        trove_manager: &mut TroveManager,
        active_pool:&ActivePool,
        default_pool:&mut DefaultPool,
        borrower_trove:&mut Trove,
        reward_snapshots:&mut RewardSnapshot,
        _icr:u128,
        _solusd_in_stab_pool:u128,
        _tcr:u128,
        _price:u128

    )->LiquidationValues{
        let mut vars = LocalVariablesInnerSingleLiquidateFunction::new();
        let mut single_liquidation = LiquidationValues::new();

        //if (TroveOwners.length <= 1) {return singleLiquidation;} // don't liquidate if last trove
        let (entire_trove_debt, entire_trove_coll, pending_debt_reward, pending_coll_reward) = Self::get_entire_debt_and_coll(trove_manager, borrower_trove, reward_snapshots);
        single_liquidation.entire_trove_debt = entire_trove_debt;
        single_liquidation.entire_trove_coll = entire_trove_coll;
        vars.pending_debt_reward = pending_debt_reward;
        vars.pending_coll_reward = pending_coll_reward;

        single_liquidation.coll_gas_compensation = Self::get_coll_gas_compensation(single_liquidation.entire_trove_coll);
        single_liquidation.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;
        vars.coll_to_liquidate = single_liquidation.entire_trove_coll - single_liquidation.coll_gas_compensation;

        // If ICR <= 100%, purely redistribute the Trove across all active Troves
        if _icr <= _100PCT {
            Self::move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
            Self::remove_stake(trove_manager,borrower_trove);

            single_liquidation.debt_to_offset = 0;
            single_liquidation.coll_to_send_to_sp = 0;
            single_liquidation.debt_to_redistribute = single_liquidation.entire_trove_debt;
            single_liquidation.coll_to_redistribute = vars.coll_to_liquidate;

            Self::close_trove(borrower_trove, reward_snapshots);
        }
        else if (_icr > _100PCT) && (_icr < MCR) {
            Self::move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
            Self::remove_stake(trove_manager,borrower_trove);

            let (_debt_to_offset, _coll_to_send_to_sp, _debt_to_redistribute, _coll_to_liquidate) = Self::get_offset_and_redistribution_vals(single_liquidation.entire_trove_debt, vars.coll_to_liquidate, _solusd_in_stab_pool);
            //_closeTrove(_borrower, Status.closedByLiquidation);
        }
        /*
        * If 110% <= ICR < current TCR (accounting for the preceding liquidations in the current sequence)
        * and there is SOLUSD in the Stability Pool, only offset, with no redistribution,
        * but at a capped rate of 1.1 and only if the whole debt can be liquidated.
        * The remainder due to the capped rate will be claimable as collateral surplus.
        */
        else if (_icr >= MCR) && (_icr < _tcr) && (single_liquidation.entire_trove_debt <= _solusd_in_stab_pool) {
            Self::move_pending_trove_reward_to_active_pool(trove_manager, vars.pending_debt_reward, vars.pending_coll_reward, default_pool, active_pool);
            //assert(_LUSDInStabPool != 0);
            Self::remove_stake(trove_manager,borrower_trove);
            Self::get_capped_offset_vals(&mut single_liquidation, _price);

            //_closeTrove(_borrower, Status.closedByLiquidation);
            if single_liquidation.coll_surplus > 0 {
                //collSurplusPool.accountSurplus(_borrower, singleLiquidation.collSurplus);
            }
        }
        else{
            let zero_vals = LiquidationValues::new();
            return zero_vals;
        }
        return single_liquidation;

    }
    pub fn get_capped_offset_vals(single_liquidation:&mut LiquidationValues, price:u128){
        let capped_coll_portion = single_liquidation.entire_trove_debt * MCR / price;

        single_liquidation.coll_gas_compensation = Self::get_coll_gas_compensation(capped_coll_portion);
        single_liquidation.solusd_gas_compensation = SOLUSD_GAS_COMPENSATION;

        single_liquidation.debt_to_offset = single_liquidation.entire_trove_debt;
        single_liquidation.coll_to_send_to_sp = capped_coll_portion - single_liquidation.coll_gas_compensation;
        single_liquidation.coll_surplus = single_liquidation.entire_trove_coll - capped_coll_portion;
        single_liquidation.debt_to_redistribute = 0;
        single_liquidation.coll_to_redistribute = 0;
    }
    /* In a full liquidation, returns the values for a trove's coll and debt to be offset, and coll and debt to be
    * redistributed to active troves.
    */
    pub fn get_offset_and_redistribution_vals(debt:u128, coll:u128, solusd_in_stab_pool:u128)->(u128,u128,u128,u128){
        if solusd_in_stab_pool > 0 {
            /*
            * Offset as much debt & collateral as possible against the Stability Pool, and redistribute the remainder
            * between all active troves.
            *
            *  If the trove's debt is larger than the deposited SOLUSD in the Stability Pool:
            *
            *  - Offset an amount of the trove's debt equal to the SOLUSD in the Stability Pool
            *  - Send a fraction of the trove's collateral to the Stability Pool, equal to the fraction of its offset debt
            *
            */
            let debt_to_offset = if debt < solusd_in_stab_pool {debt} else {solusd_in_stab_pool};
            let coll_to_send_to_sp = coll * debt_to_offset / debt;
            let debt_to_redistribute = debt - debt_to_offset;
            let coll_to_redistribute = coll - coll_to_send_to_sp;
            (debt_to_offset, coll_to_send_to_sp, debt_to_redistribute, coll_to_redistribute)
        }
        else {
            let debt_to_offset = 0;
            let coll_to_send_to_sp = 0;
            let debt_to_redistribute = debt;
            let coll_to_redistribute = coll;
            (debt_to_offset, coll_to_send_to_sp, debt_to_redistribute, coll_to_redistribute)
        }
    }
    pub fn close_trove(borrower_trove:&mut Trove, reward_snapshots:&mut RewardSnapshot){
        //assert(closedStatus != Status.nonExistent && closedStatus != Status.active);

        //uint TroveOwnersArrayLength = TroveOwners.length;
        //_requireMoreThanOneTroveInSystem(TroveOwnersArrayLength);

        //borrower_trove.status = closedStatus;
        borrower_trove.coll = 0;
        borrower_trove.debt = 0;

        reward_snapshots.sol = 0;
        reward_snapshots.solusd_debt = 0;

        //_removeTroveOwner(_borrower, TroveOwnersArrayLength);
        //sortedTroves.remove(_borrower);
    }
    pub fn remove_stake(trove_manager:&mut TroveManager, borrower_trove:&mut Trove){
        let stake = borrower_trove.stake;
        trove_manager.total_stakes = trove_manager.total_stakes - stake;
        borrower_trove.stake = 0;
    }
    pub fn get_coll_gas_compensation(entire_coll:u128)->u128{
        entire_coll / PERCENT_DIVISOR
    }
    pub fn get_entire_debt_and_coll(trove_manager:&TroveManager, borrower_trove:&Trove, reward_snapshots:&RewardSnapshot)->(u128,u128,u128,u128){
        let mut debt = borrower_trove.debt;
        let mut coll = borrower_trove.coll;

        let pending_solusd_debt_reward = Self::get_pending_solusd_debt_reward(trove_manager, borrower_trove, reward_snapshots);
        let pending_sol_reward = Self::get_pending_sol_reward(trove_manager, borrower_trove, reward_snapshots);

        debt += pending_solusd_debt_reward;
        coll += pending_sol_reward;

        (debt, coll, pending_solusd_debt_reward, pending_sol_reward)

    }
    pub fn get_current_icr(
        trove_manager_data:&TroveManager, 
        borrower_trove:&mut Trove, 
        reward_snapshot:&mut RewardSnapshot, 
        price:u128
    )->u128{
        let (current_sol, current_solusd_debt) = Self::get_current_trove_amounts(trove_manager_data, borrower_trove, reward_snapshot);
        let icr = compute_cr(current_sol, current_solusd_debt, price);
        return icr;
    }
    pub fn get_current_trove_amounts(
        trove_manager_data:&TroveManager, 
        borrower_trove:&mut Trove, 
        reward_snapshot:&mut RewardSnapshot, 
    )->(u128,u128){
        let pending_sol_reward = Self::get_pending_sol_reward(trove_manager_data,borrower_trove, reward_snapshot);
        let pending_solusd_debt_reward = Self::get_pending_solusd_debt_reward(trove_manager_data, borrower_trove, reward_snapshot);

        let current_sol = borrower_trove.coll + pending_sol_reward;
        let current_solusd = borrower_trove.debt + pending_solusd_debt_reward;

        return (current_sol, current_solusd);
    }
    pub fn apply_pending_rewards(
        trove_manager_data:&TroveManager, 
        borrower_trove:&mut Trove, 
        reward_snapshot:&mut RewardSnapshot, 
        default_pool_data:&mut DefaultPool, 
        active_pool_data:&ActivePool)
    {
        if Self::has_pending_rewards(trove_manager_data, borrower_trove, reward_snapshot) {
            if borrower_trove.is_active() {
                // Compute pending rewards
                let pending_sol_reward = Self::get_pending_sol_reward(trove_manager_data,borrower_trove, reward_snapshot);
                let pending_solusd_debt_reward = Self::get_pending_solusd_debt_reward(trove_manager_data, borrower_trove, reward_snapshot);

                // Apply pending rewards to trove's state
                borrower_trove.coll = borrower_trove.coll + pending_sol_reward;
                borrower_trove.debt = borrower_trove.debt + pending_solusd_debt_reward;

                reward_snapshot.update_trove_reward_snapshots(trove_manager_data);

                

                // Transfer from DefaultPool to ActivePool
                Self::move_pending_trove_reward_to_active_pool(
                    trove_manager_data, 
                    pending_sol_reward, 
                    pending_solusd_debt_reward,
                    default_pool_data,
                    &active_pool_data
                );
            }
        }
    }
    pub fn move_pending_trove_reward_to_active_pool(
        trove_manager_data:&TroveManager,
        _solusd:u128, 
        _sol:u128,
        default_pool_data:&mut DefaultPool,
        active_pool_data:&ActivePool
    ){
        
        default_pool_data.decrease_solusd_debt(_solusd);
        default_pool_data.increase_solusd_debt(_sol);

        // _defaultPool.sendETHToActivePool(_ETH);
    }
    
    pub fn get_pending_sol_reward(trove_manager_data:&TroveManager, borrower_trove:&Trove, reward_snapshot:&RewardSnapshot)->u128{
        let snapshot_sol = reward_snapshot.sol;
        let reward_per_unit_staked = trove_manager_data.l_sol - snapshot_sol;

        if reward_per_unit_staked == 0 || !borrower_trove.is_active() {
            return 0;
        }
        let stake = borrower_trove.stake;
        let pending_sol_reward = stake * reward_per_unit_staked / DECIMAL_PRECISION;
        return pending_sol_reward;
    }
    pub fn get_pending_solusd_debt_reward(trove_manager_data:&TroveManager, borrower_trove:&Trove, reward_snapshot:&RewardSnapshot)->u128{
        let snapshot_solusd_debt = reward_snapshot.solusd_debt;
        let reward_per_unit_staked = trove_manager_data.l_solusd_debt - snapshot_solusd_debt;

        if reward_per_unit_staked == 0 || !borrower_trove.is_active() {
            return 0;
        }
        let stake = borrower_trove.stake;
        let pending_solusd_debt_reward = stake * reward_per_unit_staked / DECIMAL_PRECISION;
        return pending_solusd_debt_reward;
    }
    pub fn has_pending_rewards(trove_manager_data:&TroveManager, borrower_trove:&Trove, reward_snapshot:&RewardSnapshot)->bool{
        /*
        * A Trove has pending rewards if its snapshot is less than the current rewards per-unit-staked sum:
        * this indicates that rewards have occured since the snapshot was made, and the user therefore has
        * pending rewards
        */
        let status = Status::from_u8(borrower_trove.status).unwrap();
        match status {
            Status::Active =>{
            }
            _ =>{
                return false;
            }
        }
        return reward_snapshot.sol < trove_manager_data.l_sol;
    }
    pub fn get_pyth_price(pyth_price_info: &AccountInfo, clock: &Clock) -> Result<Decimal, ProgramError> {
        const STALE_AFTER_SLOTS_ELAPSED: u128 = 5;

        let pyth_price_data = pyth_price_info.try_borrow_data()?;
        let pyth_price = pyth::load::<pyth::Price>(&pyth_price_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        if pyth_price.ptype != pyth::PriceType::Price {
            msg!("Oracle price type is invalid");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }

        let slots_elapsed = clock
            .slot
            .checked_sub(pyth_price.valid_slot)
            .ok_or(TroveManagerError::MathOverflow).unwrap() as u128;
        if slots_elapsed >= STALE_AFTER_SLOTS_ELAPSED {
            msg!("Oracle price is stale");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }

        let price: u128 = pyth_price.agg.price.try_into().map_err(|_| {
            msg!("Oracle price cannot be negative");
            TroveManagerError::InvalidOracleConfig
        })?;

        let market_price = if pyth_price.expo >= 0 {
            let exponent = pyth_price
                .expo
                .try_into()
                .map_err(|_| TroveManagerError::MathOverflow)?;
            let zeros = 10u64
                .checked_pow(exponent)
                .ok_or(TroveManagerError::MathOverflow)?;
            Decimal::from(price).try_mul(zeros)?
        } else {
            let exponent = pyth_price
                .expo
                .checked_abs()
                .ok_or(TroveManagerError::MathOverflow)?
                .try_into()
                .map_err(|_| TroveManagerError::MathOverflow)?;
            let decimals = 10u64
                .checked_pow(exponent)
                .ok_or(TroveManagerError::MathOverflow)?;
            Decimal::from(price).try_div(decimals)?
        };

        Ok(market_price)
    }
    

    /// get authority by given program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, TroveManagerError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(TroveManagerError::InvalidProgramAddress))
    }

    /// issue a spl_token `Transfer` instruction.
    pub fn token_transfer<'a>(
        pool: &Pubkey,
        token_program: AccountInfo<'a>,
        source: AccountInfo<'a>,
        destination: AccountInfo<'a>,
        authority: AccountInfo<'a>,
        nonce: u8,
        amount: u64,
    ) -> Result<(), ProgramError> {
        let pool_bytes = pool.to_bytes();
        let authority_signature_seeds = [&pool_bytes[..32], &[nonce]];
        let signers = &[&authority_signature_seeds[..]];
        let ix = spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?;
        invoke_signed(
            &ix,
            &[source, destination, authority, token_program],
            signers,
        )
    } 
    
}


/// implement all stability pool error messages
impl PrintProgramError for TroveManagerError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string());
    }
}