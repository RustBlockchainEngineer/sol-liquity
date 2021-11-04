//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        instruction::{BorrowerOperationsInstruction},
    },
    liquity_common::{
        state::*,
        utils::*,
        error::{LiquityError},
        constant::{
            DECIMAL_PRECISION,
            MIN_NET_DEBT,
            SOLUSD_GAS_COMPENSATION,
            BORROWING_FEE_FLOOR,
            MCR,
            CCR
        },
        pyth,
        math::{Decimal, Rate, TryAdd, TryDiv, TryMul, WAD},
        liquity_math::*
    },
    borsh::{BorshDeserialize, BorshSerialize},
    num_traits::FromPrimitive,
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
};

#[derive(Clone, Debug, Default, PartialEq, BorshDeserialize, BorshSerialize)]
struct TroveAmount{
    pub coll:u128,
    pub debt: u128
}

/// Program state handler.
/// Main logic of this program
pub struct Processor {}
impl Processor {

    fn require_valid_max_fee_percentage(_maxFeePercentage:u128, _isRecoveryMode:bool) -> Result<(), ProgramError> {
        if _isRecoveryMode {
            if (_maxFeePercentage as u128) > DECIMAL_PRECISION 
            {
                return Err(LiquityError::ExceedMaxFeePercentage.into());
            }
        } else if (_maxFeePercentage as u128) < BORROWING_FEE_FLOOR || (_maxFeePercentage as u128)> DECIMAL_PRECISION
        {
            return Err(LiquityError::InvalidMaxFeePercentage.into());
        }
        Ok(())
    }

    fn get_new_tcr_from_trove_change
    (
        activePool: ActivePool,
        defaultPool: DefaultPool,
        _collChange: u128,
        _isCollIncrease: bool ,
        _debtChange: u128,
        _isDebtIncrease: bool ,
        _price: u128
    ) -> u128{
        let mut totalColl = activePool.sol + defaultPool.sol;
        let mut totalDebt = activePool.solusd_debt + defaultPool.solusd_debt;

        totalColl = if _isCollIncrease {totalColl + _collChange } else { totalColl - _collChange};
        totalDebt = if _isDebtIncrease {totalDebt + _debtChange} else {totalDebt - _debtChange};

        let newTCR = compute_cr(totalColl, totalDebt, _price);
        return newTCR;
    }

    /// All instructions start from here and are processed by their type.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = BorrowerOperationsInstruction::try_from_slice(input)?;

        // determine instruction type
        match instruction {
            BorrowerOperationsInstruction::Initialize{
                nonce,
            } => {
                // Instruction: Initialize
                Self::process_initialize(program_id, accounts, nonce)
            }
            BorrowerOperationsInstruction::OpenTrove{
                max_fee_percentage,
                solusd_amount,
                coll_increase,
                sol_amount
            } => {
                // Instruction: OpenTrove
                Self::process_open_trove(program_id, accounts, max_fee_percentage as u128, solusd_amount  as u128, coll_increase  as u128, sol_amount  as u128)
            }
            BorrowerOperationsInstruction::AdjustTrove{
                coll_withdrawal,
                solusd_change,
                is_debt_increase,
                max_fee_percentage,
                sol_amount
            } => {
                // let is_debt_increase_t =  match is_debt_increase
                // {
                //     [0] => false,
                //     [1] => true,
                //     _ => return Err(LiquityError::InvalidAccountInput),
                // };
                // Instruction: AdjustTrove
                Self::process_adjust_trove(program_id, accounts, coll_withdrawal  as u128, solusd_change  as u128, is_debt_increase != 0, max_fee_percentage  as u128, sol_amount  as u128)
            }
            BorrowerOperationsInstruction::CloseTrove(amount) => {
                // Instruction: CloseTrove
                Self::process_close_trove(program_id, accounts, amount  as u128)
            }
        }
    }
    fn active_pool_add_coll(active_pool_info:&AccountInfo, amount:u128) -> Result<(), ProgramError> {
        let mut active_pool = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        active_pool.sol = amount;
        active_pool.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])?;
        Ok(())
    }
    fn withdraw_solusd<'a>(
        borrower_data_info: &AccountInfo<'a>,
        authority_info: &AccountInfo<'a>,
        active_pool_info: &AccountInfo<'a>,
        solusd_token_info: &AccountInfo<'a>,
        destination_info: & AccountInfo<'a>, 
        token_program_info: &AccountInfo<'a>,
        nonce: u8,
        solusd_amount: u128,
        netdebt_increase: u128
     )->Result<(), ProgramError> {
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        active_pool_data.increase_solusd_debt(netdebt_increase);
        active_pool_data.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])?;

        token_mint_to(
            borrower_data_info.key,
            token_program_info.clone(),
            solusd_token_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            nonce,
            solusd_amount as u64,
        )?;
        Ok(())
    }
    fn repay_solusd<'a>(
        borrower_data_info: &AccountInfo<'a>,
        authority_info: &AccountInfo<'a>,
        active_pool_info: &AccountInfo<'a>,
        solusd_token_info: &AccountInfo<'a>,
        destination_info: &AccountInfo<'a>, 
        token_program_info: &AccountInfo<'a>,
        nonce: u8,
        netdebt_increase: u128
     )->Result<(), ProgramError> {
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        active_pool_data.decrease_solusd_debt(netdebt_increase);
        active_pool_data.serialize(&mut &mut active_pool_info.data.borrow_mut()[..]);


        token_burn(            
            borrower_data_info.key,
            token_program_info.clone(),
            solusd_token_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            nonce,
            netdebt_increase as u64,
        )?;
        Ok(())
    }

    fn trigger_borrowing_fee<'a>(
        borrower_operation_info:&AccountInfo<'a>,
        authority_info:&AccountInfo<'a>,
        trove_manager_info:&AccountInfo<'a>,
        solusd_token_info: &AccountInfo<'a>,
        token_program_info: &AccountInfo<'a>,
        solid_staking_info: &AccountInfo<'a>,
        nonce:u8,
        solusd_amount: u128,
        max_fee_percentage: u128
    )->Result<u128, ProgramError> {
        let trove_manager = TroveManager::try_from_slice(&trove_manager_info.data.borrow_mut())?;
        decay_base_rate_from_borrowing(&trove_manager, borrower_operation_info)?;
        let solusd_fee:u128 = get_borrowing_fee(&trove_manager, solusd_amount);
        if !(solusd_fee * DECIMAL_PRECISION / solusd_amount <= max_fee_percentage){
            return Err(LiquityError::FeeExceeded.into());
        }

        let mut solid_staking = SOLIDStaking::try_from_slice(&solid_staking_info.data.borrow_mut())?;
        solid_staking.increase_f_solusd(solusd_fee);

        token_mint_to(            
            borrower_operation_info.key,
            token_program_info.clone(),
            solusd_token_info.clone(),
            solid_staking_info.clone(),
            authority_info.clone(),
            nonce,
            solusd_fee as u64,
        )?;

        trove_manager.serialize(&mut &mut trove_manager_info.data.borrow_mut()[..])?;
        solid_staking.serialize(&mut &mut solid_staking_info.data.borrow_mut()[..])?;

        Ok(solusd_fee)
    }
    fn get_new_trove_amounts(
        coll:u128,
        debt:u128,
        coll_change:u128,
        is_coll_increase:bool,
        debt_change: u128, 
        is_debt_increase: bool
    ) ->Result<TroveAmount, ProgramError>  {
        let mut newColl = coll;
        let mut newDebt = debt;

        newColl = if is_coll_increase {coll + coll_change} else {coll - coll_change};
        newDebt = if is_debt_increase {debt + debt_change} else {debt - debt_change};

        Ok(TroveAmount{
            coll:newColl, 
            debt:newDebt
        })
    }
    /// Calculates the authority id by generating a program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, LiquityError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(LiquityError::InvalidProgramAddress))
    }

    fn get_new_icr_from_trove_change
    (
        coll:u128,
        debt: u128,
        coll_change: u128,
        is_coll_increase: bool,
        debt_change: u128,
        is_debt_increase:bool,
        price: u128
    ) -> Result<u128, ProgramError>  {
        let res = Self::get_new_trove_amounts(coll, debt, coll_change, is_coll_increase, debt_change, is_debt_increase)?;

        let new_iCR = compute_cr(res.coll, res.debt, price);
        Ok(new_iCR)
    }
    /*#[allow(clippy::too_many_arguments)]
    fn check_accounts(
        borrower_operations: &dyn BorrowerOperations,
        program_id: &Pubkey,
        borrower_operation_info: &AccountInfo,
        authority_info: &AccountInfo,
        token_a_info: &AccountInfo,
        token_b_info: &AccountInfo,
        pool_mint_info: &AccountInfo,
        token_program_info: &AccountInfo,
        user_token_a_info: Option<&AccountInfo>,
        user_token_b_info: Option<&AccountInfo>,
    ) -> ProgramResult {
        if swap_account_info.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
        if *authority_info.key
            != Self::authority_id(program_id, swap_account_info.key, token_swap.nonce())?
        {
            return Err(AmmError::InvalidProgramAddress.into());
        }

        let borrower_operations_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        let coll_surplus_pool = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let oracle_program_id_info = next_account_info(account_info_iter)?;
        let pyth_product_id_info = next_account_info(account_info_iter)?;
        let pyth_price_id_info = next_account_info(account_info_iter)?;

        if *token_a_info.key != *token_swap.token_a_account() {
            return Err(AmmError::IncorrectSwapAccount.into());
        }
        if *token_b_info.key != *token_swap.token_b_account() {
            return Err(AmmError::IncorrectSwapAccount.into());
        }
        if *pool_mint_info.key != *token_swap.pool_mint() {
            return Err(AmmError::IncorrectPoolMint.into());
        }
        if *token_program_info.key != *token_swap.token_program_id() {
            return Err(AmmError::IncorrectTokenProgramId.into());
        }
        if let Some(user_token_a_info) = user_token_a_info {
            if token_a_info.key == user_token_a_info.key {
                return Err(AmmError::InvalidInput.into());
            }
        }
        if let Some(user_token_b_info) = user_token_b_info {
            if token_b_info.key == user_token_b_info.key {
                return Err(AmmError::InvalidInput.into());
            }
        }
        Ok(())
    }*/
    fn  get_new_normal_icr_from_trove_change
    (
        coll:u128,
        debt: u128,
        coll_change: u128,
        is_coll_increase: bool,
        debt_change: u128,
        is_debt_increase:bool,
        price: u128,
    ) -> Result<u128, ProgramError>  {
        let res = Self::get_new_trove_amounts(coll, debt, coll_change, is_coll_increase, debt_change, is_debt_increase)?;
        let new_iCR = compute_nominal_cr(res.coll, res.debt);
        Ok(new_iCR)
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
        
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        let coll_surplus_pool = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let oracle_program_id_info = next_account_info(account_info_iter)?;
        let pyth_product_id_info = next_account_info(account_info_iter)?;
        let pyth_price_id_info = next_account_info(account_info_iter)?;

        let mut borrower_operations = try_from_slice_unchecked::<BorrowerOperations>(&borrower_operation_info.data.borrow())?;

        borrower_operations.trove_manager_id = *trove_manager_info.key;
        borrower_operations.active_pool_id = *active_pool_info.key;
        borrower_operations.default_pool_id = *default_pool_info.key;
        borrower_operations.stability_pool_id = *stability_pool_info.key;
        borrower_operations.gas_pool_id = *gas_pool_info.key;
        borrower_operations.coll_surplus_pool_id = *coll_surplus_pool.key;
        borrower_operations.solusd_token_id = *solusd_token_info.key;
        borrower_operations.solid_staking_id = *solid_staking_info.key;
        borrower_operations.oracle_program_id = *oracle_program_id_info.key;
        borrower_operations.pyth_product_id = *pyth_product_id_info.key;
        borrower_operations.pyth_price_id = *pyth_price_id_info.key;

        if *authority_info.key != Self::authority_id(program_id, borrower_operation_info.key, nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }
        
        borrower_operations.serialize(&mut &mut borrower_operation_info.data.borrow_mut()[..])?;
        Ok(())
    } 
    
    /// process OpenTrove instruction
    pub fn process_open_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        max_fee_percentage: u128,
        solusd_amount: u128,
        coll_increase:u128,
        sol_amount:u128
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        let owner_id_info = next_account_info(account_info_iter)?;

        let borrower_operations = BorrowerOperations::try_from_slice(&borrower_operation_info.data.borrow())?;
        let mut borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;
        let mut trove_manager = TroveManager::try_from_slice(&trove_manager_info.data.borrow())?;

        let active_pool = ActivePool::try_from_slice(&active_pool_info.data.borrow())?;
        let default_pool = DefaultPool::try_from_slice(&default_pool_info.data.borrow())?;

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_operations.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        let market_price = get_market_price(
            borrower_operations.oracle_program_id,
            borrower_operations.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;
        
        let is_recovery_mode = trove_manager.check_recovery_mode(market_price, &active_pool, &default_pool) == 1;

        Self::require_valid_max_fee_percentage(max_fee_percentage, is_recovery_mode)?;

        if borrower_trove.status == 1{
            return Err(LiquityError::ErrorTroveisActive.into())
        }

        let mut solusd_fee = 0;
        let mut net_debt = solusd_amount;
        if !is_recovery_mode{
            solusd_fee = Self::trigger_borrowing_fee(
                &borrower_operation_info,
                &authority_info,
                &trove_manager_info,
                &solusd_token_info,
                &token_program_info,
                &solid_staking_info,
                borrower_operations.nonce,
                solusd_amount,
                max_fee_percentage)?;
            net_debt += solusd_fee;
        }
        if net_debt < MIN_NET_DEBT {
            return Err(LiquityError::ErrorMinNetDebt.into());
        }
        let composite_debt =  net_debt + SOLUSD_GAS_COMPENSATION;

        if composite_debt == 0 {
            return Err(LiquityError::InvalidCompositeDebt.into());
        }

        let icr = compute_cr(coll_increase, composite_debt, market_price);
        let nicr = compute_nominal_cr(coll_increase, composite_debt);

        let mut vars = LocalVariablesOpenTrove::new(*borrower_operation_info.key, *owner_id_info.key);
        vars.price = market_price;
        vars.solusd_fee = solusd_fee;
        vars.net_debt = net_debt;
        vars.composite_debt = composite_debt;
        vars.icr = icr;
        vars.nicr = nicr;

        if is_recovery_mode {
            if vars.icr < CCR {
                return Err(LiquityError::CCRError.into());
            }
        }
        else {
            if vars.icr < MCR {
                return Err(LiquityError::MCRError.into());
            }
            let new_tcr = Self::get_new_tcr_from_trove_change(
                active_pool, 
                default_pool,
                coll_increase, 
                true, 
                vars.composite_debt, 
                true, 
                vars.price);

            if new_tcr < CCR {
                return Err(LiquityError::CCRError.into());
            }
        }
       
        borrower_trove.status = 1;
        borrower_trove.coll += coll_increase;
        borrower_trove.debt += composite_debt;

        let new_stake = borrower_trove.coll;
        let old_stake = borrower_trove.stake;
        borrower_trove.stake = new_stake;
        trove_manager.total_stakes = trove_manager.total_stakes - old_stake + new_stake;

        vars.stake = trove_manager.total_stakes;

        Self::withdraw_solusd(
            &borrower_operation_info, 
            &authority_info,
            &active_pool_info,
            &solusd_token_info,
            &borrower_info,
            &token_program_info, 
            borrower_operations.nonce,
            solusd_amount,
            vars.net_debt
        )?;

        Self::withdraw_solusd(
            &borrower_operation_info, 
            &authority_info,
            &active_pool_info,
            &solusd_token_info,
            &borrower_info, 
            &token_program_info, 
            borrower_operations.nonce,
            SOLUSD_GAS_COMPENSATION,
            SOLUSD_GAS_COMPENSATION
        )?;

        borrower_operations
            .serialize(&mut *borrower_operation_info.data.borrow_mut())?;
        borrower_trove
            .serialize(&mut *borrower_operation_info.data.borrow_mut())?;
        trove_manager
            .serialize(&mut *borrower_operation_info.data.borrow_mut())?;
        
        Ok(())
    }

    /// process WithdrawFromSP instruction
    pub fn process_adjust_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        coll_withdrawal: u128,
        solusd_change: u128,
        is_debt_increase:bool,
        max_fee_percentage: u128,
        sol_amount: u128
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let owner_id_info = next_account_info(account_info_iter)?;
        //let default_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        //let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        // let price_feed_info = next_account_info(account_info_iter)?;
        //let sorted_troves_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;

        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock_info = next_account_info(account_info_iter)?;
        
        let borrower_operations = BorrowerOperations::try_from_slice(&borrower_operation_info.data.borrow())?;
        let borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;
        let stability_pool = StabilityPool::try_from_slice(&stability_pool_info.data.borrow())?;
        let mut active_pool = ActivePool::try_from_slice(&active_pool_info.data.borrow())?;
        let clock = &Clock::from_account_info(clock_info)?;

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_operations.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        let mut vars = LocalVariablesAdjustTrove::new(*borrower_operation_info.key, *owner_id_info.key);
        vars.price = get_market_price(
            stability_pool.oracle_program_id,
            stability_pool.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;
        let is_recovery_mode = true; // check_recovery_mode
        if !(sol_amount == 0 || coll_withdrawal == 0)
        {
            return Err(LiquityError::ErrorSignularCollChange.into());
        }

        if !(sol_amount != 0 || coll_withdrawal != 0 || solusd_change != 0)
        {
            return Err(LiquityError::ErrorNoneZeroAdjustment.into());
        }

        if !borrower_trove.is_active()
        {
            return Err(LiquityError::ErrorTroveisNotActive.into());
        }

        vars.coll_change = 0;
        if sol_amount != 0{
            vars.coll_change = sol_amount;
            vars.is_coll_increase = true;
        }
        else{
            vars.coll_change = coll_withdrawal;
        }
        vars.net_debt_change = solusd_change;
        if is_debt_increase && !is_recovery_mode
        {
            vars.solusd_fee = Self::trigger_borrowing_fee(
                &borrower_operation_info,
                &authority_info,
                &trove_manager_info,
                &solusd_token_info,
                &token_program_info,
                &solid_staking_info,
                borrower_operations.nonce,
                solusd_change,
                max_fee_percentage)?;

            vars.net_debt_change += vars.solusd_fee;
        }

        let mut borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow_mut())?;

        vars.debt = borrower_trove.debt;
        vars.coll = borrower_trove.coll;
        vars.old_icr = compute_cr(vars.coll, vars.debt, vars.price);
        vars.new_icr = Self::get_new_icr_from_trove_change(
                vars.coll, 
                vars.debt, 
                vars.coll_change, 
                vars.is_coll_increase, 
                vars.net_debt_change, 
                is_debt_increase, 
                vars.price)?;

        //_updateTroveFromAdjustment
        if vars.is_coll_increase
        {
            vars.new_coll = borrower_trove.increase_trove_coll(vars.coll_change)
        }
        else
        {
            vars.new_coll = borrower_trove.decrease_trove_coll(vars.coll_change)
        }

        if is_debt_increase
        {
            vars.new_debt = borrower_trove.increase_trove_debt(vars.net_debt_change)
        }
        else
        {
            vars.new_debt = borrower_trove.decrease_trove_debt(vars.net_debt_change)
        }

        let new_nicr = Self::get_new_normal_icr_from_trove_change(
            vars.coll, 
            vars.debt, 
            vars.coll_change, 
            vars.is_coll_increase, 
            vars.net_debt_change, 
            is_debt_increase,
            vars.price
        );
        
        if is_debt_increase{
            Self::withdraw_solusd(
                &borrower_info,
                &authority_info,
                &active_pool_info,
                &solusd_token_info,
                &borrower_info,
                &token_program_info,
                borrower_operations.nonce,
                solusd_change,
                vars.net_debt_change
            );
        }
        else
        {
            Self::repay_solusd(
                &borrower_operation_info,
                &authority_info,
                &active_pool_info,
                &solusd_token_info,
                &borrower_info,
                &token_program_info,
                borrower_operations.nonce,
                vars.net_debt_change

            );
        }

        if vars.is_coll_increase {
            active_pool.increase_coll(vars.coll_change);
        }
        else{
            active_pool.send_sol(vars.coll_change);
        }
        borrower_operations.serialize(&mut &mut borrower_operation_info.data.borrow_mut()[..])?;
        active_pool.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])?;
        borrower_trove.serialize(&mut &mut borrower_trove_info.data.borrow_mut()[..])?;
        // trove_manager.serialize(&mut &mut trove_manager_info.data.borrow_mut()[..])?;
        stability_pool.serialize(&mut &mut stability_pool_info.data.borrow_mut()[..])?;
        
        // check if given pool token account is same with pool token account
        Ok(())
    }

    /// process WithdrawFromSP instruction
    pub fn process_close_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u128,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        //let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        // let price_feed_info = next_account_info(account_info_iter)?;
        //let sorted_troves_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let rewardsnapshot_info = next_account_info(account_info_iter)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock_info = next_account_info(account_info_iter)?;

        
        let clock = &Clock::from_account_info(clock_info)?;
        
        let borrower_operations = BorrowerOperations::try_from_slice(&borrower_operation_info.data.borrow())?;
        let mut borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;
        let mut active_pool = ActivePool::try_from_slice(&active_pool_info.data.borrow())?;
        let mut trove_manager = TroveManager::try_from_slice(&trove_manager_info.data.borrow())?;
        let mut default_pool = DefaultPool::try_from_slice(&default_pool_info.data.borrow())?;
        let mut reward_snapshot = RewardSnapshot::try_from_slice(&borrower_trove_info.data.borrow())?;
        let stability_pool = StabilityPool::try_from_slice(&stability_pool_info.data.borrow())?;
        
        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_operations.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        let price = get_market_price(
            stability_pool.oracle_program_id,
            stability_pool.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;

        apply_pending_rewards(
            &trove_manager, 
            &mut borrower_trove, 
            &mut reward_snapshot, 
            &mut default_pool, 
            &mut active_pool);
        let debt = borrower_trove.debt;
        let coll = borrower_trove.coll;

        let new_tcr = Self::get_new_tcr_from_trove_change(
            active_pool, 
            default_pool,
            coll, 
            false, 
            debt, 
            false, 
            price);
        remove_stake(&mut trove_manager, &mut borrower_trove);
        borrower_trove.close_trove();
        reward_snapshot.reset();

        let debt = debt.checked_sub(SOLUSD_GAS_COMPENSATION).ok_or(LiquityError::ErrorMinNetDebt)?;

        Self::repay_solusd(
            &borrower_operation_info,
            &authority_info,
            &active_pool_info,
            &solusd_token_info,
            &borrower_info,
            &token_program_info,
            borrower_operations.nonce,
            debt
        )?;

        Self::repay_solusd(
            &borrower_operation_info,
            &authority_info,
            &active_pool_info,
            &solusd_token_info,
            &gas_pool_info,
            &token_program_info,
            borrower_operations.nonce,
            SOLUSD_GAS_COMPENSATION
        )?;
        active_pool.send_sol(coll);

        borrower_operations.serialize(&mut &mut borrower_operation_info.data.borrow_mut()[..])?;
        active_pool.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])?;
        borrower_trove.serialize(&mut &mut borrower_trove_info.data.borrow_mut()[..])?;
        trove_manager.serialize(&mut &mut trove_manager_info.data.borrow_mut()[..])?;
        stability_pool.serialize(&mut &mut stability_pool_info.data.borrow_mut()[..])?;

        // check if given pool token account is same with pool token account
        Ok(())
        
    }
}
