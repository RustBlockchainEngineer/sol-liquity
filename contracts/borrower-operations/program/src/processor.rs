//! Program state processor
//! In here, All instructions are processed by Processor

use {
    liquity_common::{
        state::{
            BorrowerOperations,LocalVariablesAdjustTrove,LocalVariablesOpenTrove,ContractsCache,TroveManager, ActivePool
        },
        utils::*,
        error::{LiquityError},
        utils::{
            authority_id
        },
        constant::{
            DECIMAL_PRECISION,
            MIN_NET_DEBT,
            LUSD_GAS_COMPENSATION
            MCR,
        },
        pyth,
        math::{Decimal, Rate, TryAdd, TryDiv, TryMul, WAD},
        liquity_math::*
    },
    crate::{
        instruction::{BorrowerOperationsInstruction},
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
    //get current Sol price
    fn get_pyth_price(pyth_product_info:&AccountInfo, pyth_price_info: &AccountInfo, clock_info: &AccountInfo) ->u64{
        let clock = &Clock::from_account_info(clock_info)?;
        let pyth_product_data = pyth_product_info.try_borrow_data()?;
        let pyth_product = pyth::load::<pyth::Product>(&pyth_product_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;
        if pyth_product.magic != pyth::MAGIC {
            msg!("Pyth product account provided is not a valid Pyth account");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
        if pyth_product.ver != pyth::VERSION_2 {
            msg!("Pyth product account provided has a different version than expected");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
        if pyth_product.atype != pyth::AccountType::Product as u32 {
            msg!("Pyth product account provided is not a valid Pyth product account");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
    
        let pyth_price_pubkey_bytes: &[u8; 32] = pyth_price_info
            .key
            .as_ref()
            .try_into()
            .map_err(|_| StabilityPoolError::InvalidAccountInput)?;
        if &pyth_product.px_acc.val != pyth_price_pubkey_bytes {
            msg!("Pyth product price account does not match the Pyth price provided");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
    
        let quote_currency = Self::get_pyth_product_quote_currency(pyth_product)?;
        if pool_data.quote_currency != quote_currency {
            msg!("Lending market quote currency does not match the oracle quote currency");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
        let market_price = stability_pool::processor::Processor::get_pyth_price(pyth_price_info, clock)
        return market_price;
    } 
    fn _requireValidMaxFeePercentage(_maxFeePercentage:u64, _isRecoveryMode:bool){
        if (_isRecoveryMode) {
            if(_maxFeePercentage > DECIMAL_PRECISION)
                return Err(BorrowerOperationsError::ExceedMaxFeePercentage.into());
        } else {
            if(_maxFeePercentage < BORROWING_FEE_FLOOR || _maxFeePercentage > DECIMAL_PRECISION)
                return Err(BorrowerOperationsError::InvalidMaxFeePercentage.into());
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
                coll_increase
            }) => {
                // Instruction: OpenTrove
                Self::process_open_trove(program_id, accounts, max_fee_percentage, solusd_amount)
            }
            BorrowerOperationsInstruction::AdjustTrove(amount) => {
                // Instruction: AdjustTrove
                Self::process_adjust_trove(program_id, accounts, amount)
            }
            BorrowerOperationsInstruction::CloseTrove(amount) => {
                // Instruction: CloseTrove
                Self::process_close_trove(program_id, accounts, amount)
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
        
        let borrower_operations_id_info = next_account_info(account_info_iter)?;
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

        let mut borrower_operations = try_from_slice_unchecked::<BorrowerOperations>(&borrower_operations_id_info.data.borrow())?;

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
        
        borrower_operations.serialize(&mut &mut borrower_operations_id_info.data.borrow_mut()[..])?;
        Ok(())
    } 
    
    /// process OpenTrove instruction
    pub fn process_open_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        max_fee_percentage: u64,
        solusd_amount: u64,
        coll_increase:u64
    ) -> ProgramResult {
        
        let account_info_iter = &mut accounts.iter();
<<<<<<< HEAD
        let borrower_opr_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        //let default_pool_info = next_account_info(account_info_iter)?;
        //let stability_pool_info = next_account_info(account_info_iter)?;
        //let gas_pool_info = next_account_info(account_info_iter)?;
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
        let borrower_opr = BorrowerOperations::try_from_slice(&borrower_opr_info.data.borrow())?;
        let borrower_trove = Trove::try_from_slice(&borrower_trove_info.data.borrow())?;
        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, borrower_opr.nonce)? {
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

<<<<<<< HEAD
        let vars = LocalVariablesOpenTrove{
            price:market_price,
            solusd_fee,
            net_debt,
            composite_debt,
            icr,
            nicr
=======
        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
>>>>>>> f74ee6f80ef9f0f67f4fee5046ad0da9af1b1c24
        }

=======

        let borrower_operations_id_info = next_account_info(account_info_iter)?;
        let owner_id_info = next_account_info(account_info_iter)?;
        let trove_manager_id_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let solusd_pool_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        let mut borrower_operations_data = try_from_slice_unchecked::<BorrowerOperations>(&borrower_operations_id_info.data.borrow())?;
        let mut trove_manager_data = try_from_slice_unchecked::<TroveManager>(&trove_manager_id_info.data.borrow())?;
        let mut active_pool_data = try_from_slice_unchecked::<ActivePool>(&active_pool_info.data.borrow())?;

        let mut vars = LocalVariablesOpenTrove::new(*borrower_operations_id_info.key, *owner_id_info.key);
        vars.price = get_market_price(borrower_operations_data.oracle_program_id,)
        
>>>>>>> de572fd8331e8381f1a343526f799a5bd0da3061
        Ok(())
        
    }

    /// process WithdrawFromSP instruction
    pub fn process_adjust_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // pool account information to withdraw
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // pool SOLID token account
        let solid_pool_info = next_account_info(account_info_iter)?;

        // user SOLID token account
        let solid_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow pool account data to initialize 
        let pool_data = try_from_slice_unchecked::<BorrowerOperations>(&pool_id_info.data.borrow())?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(LiquityError::InvalidOwner.into());
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
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // pool account information to withdraw
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // pool SOLID token account
        let solid_pool_info = next_account_info(account_info_iter)?;

        // user SOLID token account
        let solid_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow pool account data to initialize 
        let pool_data = try_from_slice_unchecked::<BorrowerOperations>(&pool_id_info.data.borrow())?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(LiquityError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        Ok(())
        
    }
}
