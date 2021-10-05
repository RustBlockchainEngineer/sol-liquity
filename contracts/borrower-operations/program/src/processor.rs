//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        instruction::{BorrowerOperationsInstruction, OpenTroveInstruction, AdjustTroveInstruction},
    },
    liquity_common::{
        state::{
            BorrowerOperations,LocalVariablesAdjustTrove,LocalVariablesOpenTrove,ContractsCache,TroveManager, ActivePool, Trove
        },
        utils::*,
        error::{LiquityError},
        constant::{
            DECIMAL_PRECISION,
            MIN_NET_DEBT,
            LUSD_GAS_COMPENSATION,
            BORROWING_FEE_FLOOR,
            MCR,
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

/// Program state handler.
/// Main logic of this program
pub struct Processor {}
impl Processor {

    fn _requireValidMaxFeePercentage(_maxFeePercentage:u64, _isRecoveryMode:bool) ->ProgramResult{
        if _isRecoveryMode {
            if (_maxFeePercentage as u128) > DECIMAL_PRECISION 
            {
                Err(LiquityError::ExceedMaxFeePercentage.into());
            }
        } else {
            if (_maxFeePercentage as u128) < BORROWING_FEE_FLOOR || (_maxFeePercentage as u128)> DECIMAL_PRECISION
            {
                Err(LiquityError::InvalidMaxFeePercentage.into());
            }
        }
        Ok(())
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
            BorrowerOperationsInstruction::OpenTrove(OpenTroveInstruction{
                max_fee_percentage,
                solusd_amount,
                coll_increase,
            }) => {
                // Instruction: OpenTrove
                Self::process_open_trove(program_id, accounts, max_fee_percentage, solusd_amount, coll_increase)
            }
            BorrowerOperationsInstruction::AdjustTrove(AdjustTroveInstruction{
                coll_withdrawal,
                solusd_change,
                is_debt_increase,
                max_fee_percentage,
                sol_amount
            }) => {
                // Instruction: AdjustTrove
                Self::process_adjust_trove(program_id, accounts, coll_withdrawal, solusd_change, is_debt_increase, max_fee_percentage, sol_amount)
            }
            BorrowerOperationsInstruction::CloseTrove(amount) => {
                // Instruction: CloseTrove
                Self::process_close_trove(program_id, accounts, amount)
            }
        }
    }
    fn active_pool_add_coll(active_pool_info:&AccountInfo, amount:u128){
        let mut active_pool = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        active_pool.sol = amount
        active_pool.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])
    }
    fn withdraw_solusd(
        borrower_data_info: &AccountInfo,
        authority_info: &AccountInfo,
        active_pool_info: &AccountInfo,
        solusd_token_info: &AccountInfo,
        destination_info: & AccountInfo, 
        token_program_info: &AccountInfo,
        nonce:u128,
        solusd_amount: u128,
        netdebt_increase: u128
     )->{
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        active_pool_data.increase_solusd_debt(netdebt_increase)
        active_pool_data.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])

        token_mint_to(            
            borrower_data_info,
            token_program_info.clone(),
            solusd_token_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            nonce,
            to_u64(solusd_amount)?,
        );
    }
    fn repay_solusd(
        borrower_data_info: &AccountInfo,
        authority_info: &AccountInfo,
        active_pool_info: &AccountInfo,
        solusd_token_info: &AccountInfo,
        destination_info: & AccountInfo, 
        token_program_info: &AccountInfo,
        nonce:u128,
        solusd_amount: u128,
        netdebt_increase: u128
     ){
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;
        active_pool_data.decrease_solusd_debt(netdebt_increase)
        active_pool_data.serialize(&mut &mut active_pool_info.data.borrow_mut()[..])

        token_burn(            
            borrower_data_info,
            token_program_info.clone(),
            solusd_token_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            nonce,
            to_u64(solusd_amount)?,
        );
    }

    fn trigger_borrowing_fee(
        borrower_operation_info:&AccountInfo,
        trove_manager_info:&AccountInfo,
        solusd_token_info: &AccountInfo,
        solid_staking_info: &AccountInfo,
        token_program_info: &AccountInfo,
        solusd_change: u64,
        max_fee_percentage: u64
    ){
        let mut trove_manager = TroveManager:try_from_slice(&trove_manager_info.data.borrow_mut())?;
        trove_manager.decay_base_rate_from_borrowing();
        u64 solusd_fee = trove_manager.get_borrowing_fee(solusd_amount);
        if !(solusd_fee.mul(DECIMAL_PRECISION).div(solusd_amount) <= max_fee_percentage){
            Err(LiquityError::FeeExceeded.into());
        }
        let mut solid_staking = SOLIDStaking::try_from_slice(&solid_staking_info.data.borrow_mut())?;
        solid_staking.increasef_solusd(solusd_fee);

        token_mint_to(            
            borrower_operation_info.clone(),
            token_program_info.clone(),
            solusd_token_info.clone(),
            solid_staking_info.clone(),
            authority_info.clone(),
            nonce,
            to_u64(solusd_fee)?,
        );

        trove_manager.serialize(&mut &mut trove_manager_info.data.borrow_mut()[..])
        solid_staking.serialize(&mut &mut solid_staking_info.data.borrow_mut()[..])

        return solusd_fee;
    }
    fn get_new_trove_amounts(
        coll:u64,
        debt:u64,
        coll_change:u64,
        is_coll_increase:u64,
        debt_change: u64, 
        is_debt_increase: u64
    ) {
        let mut newColl = coll;
        let mut newDebt = debt;

        newColl = is_coll_increase ? coll.add(coll_change) : coll.sub(coll_change);
        newDebt = _is_debt_increase ? debt.add(debt_change) : debt.sub(debt_change);

        return (newColl, newDebt);
    }
    fn get_new_icr_from_trove_change
    (
        coll:u64,
        debt: u64,
        coll_change: u64,
        is_coll_increase: bool,
        debt_change: u64,
        is_debt_increase:bool,
        price: u64
    ) -> Option<u64>  {
        let (new_coll, new_debt) = get_new_trove_amounts(coll, debt, coll_change, is_coll_ncrease, debt_change, is_debt_increase);

        let new_iCR = LiquityMath::compute_cr(new_coll, new_debt, price);
        return new_iCR;
    }

    fn  get_new_normal_icr_from_trove_change
    (
        coll:u64,
        debt: u64,
        coll_change: u64,
        is_coll_increase: bool,
        debt_change: u64,
        is_debt_increase:bool,
    ) -> Option<u64>  {
        let (new_coll, new_debt) = get_new_trove_amounts(coll, debt, coll_change, is_coll_ncrease, debt_change, is_debt_increase);

        let new_iCR = LiquityMath::comput_normal_cr(new_coll, new_debt, price);
        return new_iCR;
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
        
        let borrower_operations_info = next_account_info(account_info_iter)?;
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

        let mut borrower_operations = try_from_slice_unchecked::<BorrowerOperations>(&borrower_operations_info.data.borrow())?;

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

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }
        
        borrower_operations.serialize(&mut &mut borrower_operations_info.data.borrow_mut()[..])?;
        Ok(())
    } 
    
    /// process OpenTrove instruction
    pub fn process_open_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        max_fee_percentage: u64,
        solusd_amount: u64,
        coll_increase:u64,
        sol_amount:u64
    ) -> ProgramResult {
        
        let account_info_iter = &mut accounts.iter();
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        //let default_pool_info = next_account_info(account_info_iter)?;
        //let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        //let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        let price_feed_info = next_account_info(account_info_iter)?;
        //let sorted_troves_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        //let solid_staking_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock_info = next_account_info(account_info_iter)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        
        let borrower_operations = BorrowerOperations::try_from_slice(&borrower_operation_info.data.borrow())?;
        let borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_operations.nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }

        let token_program_id = *token_program_info.key;
        let market_price = Self::get_pyth_price(pyth_product_info, pyth_price_info, clock_info)
        let is_recovery_mode = _checkRecoveryMode(market_price)
        Self::_requireValidMaxFeePercentage(max_fee_percentage, is_recovery_mode);

        if(borrower_trove.status == 1){
            return Err(BorrowerOperationError::TroveIsActive.into())
        }
        let mut solusd_fee = 0;
        let mut net_debt = solusd_amount;
        if(!is_recovery_mode){
            solusd_fee = 
            net_debt += solusd_fee;
        }
        if(net_debt < MIN_NET_DEBT){
            return Err(BorrowerOperationError::InvalidNetDebt.into())
        }
        let composite_debt =  net_debt + LUSD_GAS_COMPENSATION

        if(composite_debt < 0 ){
            return Err(BorrowerOperationError::InvalidCompositeDebt.into())
        }

        let icr = compute_cr(coll_increase, composit_debt, market_price)
        let nicr = compute_nominal_cr(coll_increase, composit_debt, market_price)

        let mut vars = LocalVariablesOpenTrove::new(*borrower_operations_info.key, *owner_id_info.key);
        vars.price = get_market_price(borrower_operations_data.oracle_program_id,);
        vars.solusd_fee = solusd_fee;
        vars.net_debt = net_debt;
        vars.composite_debt = composite_debt;
        vars.icr = icr;
        vars.nicr = nicr;
       
        trove_data.status = 1;
        trove_data.coll += coll_increase;
        trove_data.debt += composite_debt;

        let new_stake = trove_data.coll;
        let old_stake = trove_data.stake;
        trove_data.stake = new_stake;
        trove_manager_data.total_stakes = trove_manager_data.total_stakes.sub(old_stake).add(new_stake)

        vars.stake = trove_manager_data.total_stakes;

        trove_data.serialize(&mut &mut trove_info.data.borrow_mut()[..])?;
        
////////////////////////////////////////
        sortedTroves.insert(msg.sender, vars.NICR, _upperHint, _lowerHint);
        vars.arrayIndex = contractsCache.troveManager.addTroveOwnerToArray(msg.sender);



        reward_snapshot_data.l_sol = trove_manager_data.l_sol
        reward_snapshot_data.solusd_debt = trove_manager_data.solusd_debt
        reward_snapshot_data.serialize(&mut &mut reward_snapshot_info.data.borrow_mut()[..])

        Self::withdraw_solusd(
            borrower_operation_info.clonse(), 
            authority_info.clone(),
            active_pool_info.clone(),
            solusd_mint_info.clone(),
            borrower_info.clone(),
            token_program_info.clone(), 
            borrower_operations.nonce,
            borrower_operations.solusd_amount
            vars.net_debt
        )

        Self::withdraw_solusd(
            borrower_operation_info.clonse(), 
            authority_info.clone(),
            active_pool_info.clone(),
            solusd_mint_info.clone(),
            borrower_info.clone(),
            token_program_info.clone(), 
            borrower_operations.nonce,
            SOLUSD_GAS_COMPENSATION,
            SOLUSD_GAS_COMPENSATION
        )

        Ok(())
        
    }

    /// process WithdrawFromSP instruction
    pub fn process_adjust_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        coll_withdrawal: u64,
        solusd_change: u64,
        is_debt_increase:bool,
        max_fee_percentage: u64,
        sol_amount: u64
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        //let default_pool_info = next_account_info(account_info_iter)?;
        //let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        //let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        let price_feed_info = next_account_info(account_info_iter)?;
        //let sorted_troves_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock_info = next_account_info(account_info_iter)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        
        let borrower_operations = BorrowerOperations::try_from_slice(&borrower_operation_info.data.borrow())?;
        let borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_operations.nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }

        let mut vars = LocalVariablesAdjustTrove::new(*borrower_operations_info.key, *owner_id_info.key);
        vars.price = get_market_price(borrower_operations_data.oracle_program_id,);
        if !(sol_amount == 0 || coll_withdrawal == 0)
        {
            Err(LiquityError::ErrorSignularCollChange.into());
        }

        if !(sol_amount != 0 || coll_withdrawal != 0 || solusd_change != 0)
        {
            Err(LiquityError::ErrorNoneZeroAdjustment.into());
        }

        let trove_manager_data = TroveManager::try_from_slice(&trove_manager_info.data.borrow())?;
        if !(trove_manager_data.get_trove_status() == 1)
        {
            Err(LiquityError::ErrorTroveisNotActive.into());
        }

        let mut coll_change:u64 = 0;
        let mut is_coll_increase:bool = false;
        if sol_amount != 0{
            coll_change = sol_amount;
            is_coll_increase = true;
        }
        else{
            coll_change = coll_withdrawal;
        }
        vars.net_debt_change = solusd_change;
        if is_debt_increase && !is_recovery_mode{
            vars.solusd_fee = Self::trigger_borrowing_fee(
                borrower_operation_info.clone(),
                trove_manager_info.clone(),
                solusd_token_info.clone(),
                solid_staking_info.clone(),
                token_program_info.clone(),
                solusd_change,
                max_fee_percentage
            vars.net_debt_change = vars.net_debt_change.add(vars.solusd_fee);
        }

        let mut borrower_trove = Trove::try_from_slice(borrower_trove_info.data.borrow_mut())?;

        vars.debt = borrower_trove.get_trove_debt();
        vars.coll = borrower_trove.get_trove_coll();
        vars.old_icr = liquity_math::compute_cr(vars.coll, vars.debt, vars.price);
        vars.new_icr = Self::get_new_icr_from_trove_change(vars.coll, vars.debt, vars.coll_change, vars.is_coll_increase, vars.net_debt_change, is_debt_increase, vars.price);

        //_updateTroveFromAdjustment
        if is_coll_increase
        {
            vars.new_coll = borrower_trove.increaseTroveColl(coll_change)
        }
        else
        {
            vars.new_coll = borrower_trove.decreaseTroveColl(coll_change)
        }

        if is_debt_increase
        {
            vars.new_debt = borrower_trove.increaseTroveDebt(debt_change)
        }
        else
        {
            vars.new_debt = borrower_trove.decreaseTroveDebt(debt_change)
        }

        let mut new_nicr = get_new_normal_icr_from_trove_change()
        

        borrower_trove.increase_trove_coll(vars.coll, vars.debt, vars.coll_change, vars.is_coll_increase, vars.net_debt_change, is_debt_increase);

        //_moveTokensAndETHfromAdjustment
        _moveTokensAndETHfromAdjustment(
            contractsCache.activePool,
            contractsCache.lusdToken,
            msg.sender,
            vars.collChange,
            vars.isCollIncrease,
            _LUSDChange,
            _isDebtIncrease,
            vars.netDebtChange
        );
        if is_debt_increase{
            Self::withdraw_solusd(
                active_pool_info.clone(),
                solusd_token_info.clone(),.
                borrower_info.clone(),
                solusd_change,
                vars.net_debt_change
            );
        }
        else
        {
            Self::repay_solusd(
                active_pool_info.clone(),
                solusd_token_info.clone(),.
                borrower_info.clone(),
                solusd_change,
                vars.net_debt_change
            );
        }

        if(vars.is_coll_increase){
            active_pool_data.increase_coll(vars.coll_change);
        }
        else{
            active_pool.send_sol(vars.coll_change);

        }

        // check if given pool token account is same with pool token account
        Ok(())
    }

    /// process WithdrawFromSP instruction
    pub fn process_close_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let borrower_operation_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        //let default_pool_info = next_account_info(account_info_iter)?;
        //let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        //let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        let price_feed_info = next_account_info(account_info_iter)?;
        //let sorted_troves_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock_info = next_account_info(account_info_iter)?;
        let borrower_info = next_account_info(account_info_iter)?;
        let borrower_trove_info = next_account_info(account_info_iter)?;
        
        let borrower_operations = BorrowerOperations::try_from_slice(&borrower_operation_info.data.borrow())?;
        let borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_operations.nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }

        let price = get_market_price(borrower_operations_data.oracle_program_id,);
        if !(sol_amount == 0 || coll_withdrawal == 0)
        {
            Err(LiquityError::ErrorSignularCollChange.into());
        }

        if !(sol_amount != 0 || coll_withdrawal != 0 || solusd_change != 0)
        {
            Err(LiquityError::ErrorNoneZeroAdjustment.into());
        }

        let trove_manager_data = TroveManager::try_from_slice(&trove_manager_info.data.borrow())?;
        if !(trove_manager_data.get_trove_status() == 1)
        {
            Err(LiquityError::ErrorTroveisNotActive.into());
        }

        let mut coll_change:u64 = 0;
        let mut is_coll_increase:bool = false;
        if sol_amount != 0{
            coll_change = sol_amount;
            is_coll_increase = true;
        }
        else{
            coll_change = coll_withdrawal;
        }
        vars.net_debt_change = solusd_change;
        if is_debt_increase && !is_recovery_mode{
            vars.solusd_fee = Self::trigger_borrowing_fee(
                borrower_operation_info.clone(),
                trove_manager_info.clone(),
                solusd_token_info.clone(),
                solid_staking_info.clone(),
                token_program_info.clone(),
                solusd_change,
                max_fee_percentage
            vars.net_debt_change = vars.net_debt_change.add(vars.solusd_fee);
        }

        let mut borrower_trove = Trove::try_from_slice(borrower_trove_info.data.borrow_mut())?;
        borrower_trove.apply_pending_rewards();
        let debt = borrower_trove.get_trove_debt();
        let coll = borrower_trove.get_trove_coll();
        let new_icr = Self::get_new_icr_from_trove_change(coll, false, debt, false, price);
        borrower_trove.remove_stake();
        borrower_trove.close_trove();
        Self::repay_solusd(
            active_pool_info.clone(),
            solusd_token_info.clone(),.
            borrower_info.clone(),
            solusd_change,
            debt.sub(SOLUSD_GAS_COMPENSATION)
        );

        Self::repay_solusd(
            active_pool_info.clone(),
            solusd_token_info.clone(),.
            gas_pool_info.clone(),
            solusd_change,
            SOLUSD_GAS_COMPENSATION
        );
        active_pool.send_sol(vars.coll_change);
        // check if given pool token account is same with pool token account
        Ok(())
        
    }
}
