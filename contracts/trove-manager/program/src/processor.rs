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
            DECIMAL_PRECISION
        },
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

        Self::apply_pending_rewards(
            &trove_manager_data, 
            &mut borrower_trove,
            &mut reward_snapshot, 
            default_pool_info, 
            active_pool_info
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
        let borrower_trove_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        let trove_manager_data = try_from_slice_unchecked::<TroveManager>(&mut trove_manager_id_info.data.borrow())?;
        let mut borrower_trove = try_from_slice_unchecked::<Trove>(&borrower_trove_info.data.borrow())?;
        let default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_info.data.borrow())?;
        let active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
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

        vars.price = market_price.try_round_u64().unwrap();
        vars.solusd_in_stab_pool = stability_pool_data.total_sol_usd_deposits;
        vars.recovery_mode_at_start = trove_manager_data.check_recovery_mode(vars.price, &active_pool_data, &default_pool_data);

        // Perform the appropriate liquidation sequence - tally values and obtain their totals.


        Ok(())
    }
    pub fn process_redeem_collateral(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        solusd_amount:u64,
        partial_redemption_hint_nicr:u64,
        max_iterations:u64,
        max_fee_percentage:u64
    ) -> ProgramResult {
        
        Ok(())
    } 
    pub fn process_liquidate_troves(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        number: u64,                  // nonce for authorizing
    ) -> ProgramResult {
        Ok(())
    }

    pub fn apply_pending_rewards(
        trove_manager_data:&TroveManager, 
        borrower_trove:&mut Trove, 
        reward_snapshot:&mut RewardSnapshot, 
        default_pool_id_info:&AccountInfo, 
        active_pool_id_info:&AccountInfo)
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
                    default_pool_id_info,
                    active_pool_id_info
                );
            }
        }
    }
    pub fn move_pending_trove_reward_to_active_pool(
        trove_manager_data:&TroveManager,
        _solusd:u64, 
        _sol:u64,
        default_pool_id_info:&AccountInfo,
        active_pool_id_info:&AccountInfo
    ){
        let mut default_pool_data = try_from_slice_unchecked::<DefaultPool>(&default_pool_id_info.data.borrow()).unwrap();
        let mut active_pool_data = try_from_slice_unchecked::<DefaultPool>(&active_pool_id_info.data.borrow()).unwrap();
        default_pool_data.decrease_solusd_debt(_solusd);
        default_pool_data.increase_solusd_debt(_sol);

        // _defaultPool.sendETHToActivePool(_ETH);
    }
    
    pub fn get_pending_sol_reward(trove_manager_data:&TroveManager, borrower_trove:&Trove, reward_snapshot:&RewardSnapshot)->u64{
        let snapshot_sol = reward_snapshot.sol;
        let reward_per_unit_staked = trove_manager_data.l_sol - snapshot_sol;

        if reward_per_unit_staked == 0 || !borrower_trove.is_active() {
            return 0;
        }
        let stake = borrower_trove.stake;
        let pending_sol_reward = stake * reward_per_unit_staked / DECIMAL_PRECISION;
        return pending_sol_reward;
    }
    pub fn get_pending_solusd_debt_reward(trove_manager_data:&TroveManager, borrower_trove:&Trove, reward_snapshot:&RewardSnapshot)->u64{
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
        const STALE_AFTER_SLOTS_ELAPSED: u64 = 5;

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
            .ok_or(TroveManagerError::MathOverflow)?;
        if slots_elapsed >= STALE_AFTER_SLOTS_ELAPSED {
            msg!("Oracle price is stale");
            return Err(TroveManagerError::InvalidOracleConfig.into());
        }

        let price: u64 = pyth_price.agg.price.try_into().map_err(|_| {
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