//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::StabilityPoolError,
        instruction::{StabilityPoolInstruction},
        state::{StabilityPool,FrontEnd,Deposit,Snapshots},
        constant::{
            DECIMAL_PRECISION,
        },
        pyth,
        math::{Decimal, Rate, TryAdd, TryDiv, TryMul, WAD},
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
    spl_token::state::Mint, 
};
use std::str::FromStr;
use std::convert::TryInto;
use std::io::Error;
use community_issuance::state::{
    CommunityIssuance,
    
};

/// Program state handler.
/// Main logic of this program
pub struct Processor {}
impl Processor {  
    /// All instructions start from here and are processed by their type.
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = StabilityPoolInstruction::try_from_slice(input)?;

        // determine instruction type
        match instruction {
            StabilityPoolInstruction::Initialize{
                nonce,
            } => {
                // Instruction: Initialize
                Self::process_initialize(program_id, accounts, nonce)
            }
            StabilityPoolInstruction::ProvideToSP(amount) => {
                // Instruction: ProvideToSP
                Self::process_provide_to_sp(program_id, accounts, amount)
            }
            StabilityPoolInstruction::WithdrawFromSP(amount) => {
                // Instruction: WithdrawFromSP
                Self::process_withdraw_from_sp(program_id, accounts, amount)
            }
            StabilityPoolInstruction::WithdrawSOLGainToTrove => {
                // Instruction: WithdrawSOLGainToTrove
                Self::process_withdraw_sol_gain_to_trove(program_id, accounts)
            }
            StabilityPoolInstruction::RegisterFrontEnd(kickback_rate) => {
                // Instruction: RegisterFrontEnd
                Self::process_register_frontend(program_id, accounts, kickback_rate)
            }
        }
    }

    /// process `Initialize` instruction.
    pub fn process_initialize(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
        nonce: u8,                  // nonce for authorizing
    ) -> ProgramResult {
        // start initializeing this stability pool ...

        // get all account informations from accounts array by using iterator
        let account_info_iter = &mut accounts.iter();
        
        // stability pool account info to create newly
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority of stability pool account
        let authority_info = next_account_info(account_info_iter)?;

        // pool solUsd token account
        let solusd_pool_info = next_account_info(account_info_iter)?;

        // pool solUsd token account
        let community_issuance_info = next_account_info(account_info_iter)?;

        // spl-token program account information
        let token_program_info = next_account_info(account_info_iter)?;

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solusd_pool_info.owner != *program_id {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // borrow pool account data to initialize (mutable)
        let mut pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        pool_data.token_program_pubkey = *token_program_info.key;
        pool_data.sol_usd_pool_token_pubkey = *solusd_pool_info.key;
        pool_data.community_issuance_pubkey = *community_issuance_info.key;
        
        // serialize/store this initialized stability pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
    } 

    /// process ProvideToSP instruction
    pub fn process_provide_to_sp(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // pool account information to provide
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        // pool solUsd token account
        let solusd_pool_info = next_account_info(account_info_iter)?;

        // user solUsd token account
        let solusd_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // front end account info
        let frontend_info = next_account_info(account_info_iter)?;

        // depositor's frontend account info
        let depositor_frontend_info = next_account_info(account_info_iter)?;

        // snapshotsaccount info
        let snapshots_info = next_account_info(account_info_iter)?;

        let community_issuance_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // clock account information to use timestamp
        let clock_sysvar_info = next_account_info(account_info_iter)?;

        // get clock from clock sysvar account information
        let clock = &Clock::from_account_info(clock_sysvar_info)?;

        // get current timestamp(second)
        let cur_timestamp: u64 = clock.unix_timestamp as u64;

        // borrow pool account data
        let mut pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        let mut community_issuance_data = try_from_slice_unchecked::<CommunityIssuance>(&community_issuance_info.data.borrow())?;

        let issue_solid = community_issuance_data.issue_solid(cur_timestamp);

        pool_data.trigger_solid_issuance(issue_solid);

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solusd_pool_info.owner != *program_id {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *solusd_pool_info.key != pool_data.sol_usd_pool_token_pubkey {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // borrow frontend account data
        let frontend = try_from_slice_unchecked::<FrontEnd>(&frontend_info.data.borrow());
        let mut frontend_data = frontend.unwrap();

        if frontend_data.registered == 0 {
            return Err(StabilityPoolError::NotRegistered.into());
        }

        if frontend_data.pool_id_pubkey == *pool_id_info.key {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // borrow depositor's frontend account data
        let depositor_frontend = try_from_slice_unchecked::<FrontEnd>(&depositor_frontend_info.data.borrow());
        let depositor_frontend_data = depositor_frontend.unwrap();

        if user_deposit.initial_value == 0 {
            user_deposit.front_end_tag = frontend_data.owner_pubkey;
        }
        let initial_deposit = user_deposit.initial_value;
        let mut snapshots_data = try_from_slice_unchecked::<Snapshots>(&snapshots_info.data.borrow())?;
        let depositor_sol_gain = pool_data.get_depositor_sol_gain(initial_deposit,&snapshots_data);

        let compounded_solusd_deposit = pool_data.get_compounded_solusd_deposit(initial_deposit,&snapshots_data);
        let solusd_loss = initial_deposit - compounded_solusd_deposit;

        // First pay out any SOLID gains
        Self::payout_solid_gains(&community_issuance_data, &pool_data, &frontend_data, &depositor_frontend_data, &snapshots_data, &user_deposit);

        // Update frontend stake
        let compounded_frontend_stake = pool_data.get_compounded_frontend_stake(&frontend_data,&snapshots_data);
        let new_frontend_stake = compounded_frontend_stake + amount;
        
        // update frontend stake and snaphots
        frontend_data.update_frontend_stake(new_frontend_stake);
        snapshots_data.update_snapshots_with_frontendstake(new_frontend_stake,&pool_data);


        if amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            Self::token_transfer(
                pool_id_info.key,
                token_program_info.clone(), 
                solusd_user_info.clone(), 
                solusd_pool_info.clone(), 
                user_transfer_authority_info.clone(), 
                pool_data.nonce, 
                amount
            )?;

            // update deposit and snapshots
            user_deposit.initial_value += amount;
            snapshots_data.update_snapshots_with_deposit(new_frontend_stake,&pool_data)
        }

        if depositor_sol_gain > 0 {
            pool_data.sol -=  depositor_sol_gain;
            //send depositor_sol_gain to user (_sendETHGainToDepositor(depositorETHGain);)

        }

        // serialize/store user info again
        user_deposit
            .serialize(&mut *user_deposit_info.data.borrow_mut())?;

        // serialize/store this initialized stability pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }

    /// process WithdrawFromSP instruction
    pub fn process_withdraw_from_sp(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // pool account information to withdraw
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this pool account
        let authority_info = next_account_info(account_info_iter)?;

        // pool solUsd token account
        let solusd_pool_info = next_account_info(account_info_iter)?;

        // user solUsd token account
        let solusd_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        // borrow pool account data to initialize 
        let pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solusd_pool_info.owner != *program_id {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *solusd_pool_info.key != pool_data.sol_usd_pool_token_pubkey {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        //require no under collateralized troves
        // get market price
        if &pool_data.oracle_program_id != pyth_product_info.owner {
            msg!("Pyth product account provided is not owned by the lending market oracle program");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
        if &pool_data.oracle_program_id != pyth_price_info.owner {
            msg!("Pyth price account provided is not owned by the lending market oracle program");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }
    
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
    
        let market_price = Self::get_pyth_price(pyth_price_info, clock)?;

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // check if given amount is small than deposit amount
        let mut _amount = amount;
        if user_deposit.initial_value < amount {
            _amount = user_deposit.initial_value;
        }

        if _amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            Self::token_transfer(
                pool_id_info.key,
                token_program_info.clone(),
                solusd_pool_info.clone(),
                solusd_user_info.clone(),
                user_transfer_authority_info.clone(),
                pool_data.nonce,
                _amount
            )?;
            user_deposit.initial_value -= _amount;
        }

        // serialize/store user info again
        user_deposit
            .serialize(&mut *user_deposit_info.data.borrow_mut())?;

        // serialize/store this initialized stability pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }
    /// process WithdrawSOLGainToTrove instruction
    pub fn process_withdraw_sol_gain_to_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
        
    }
    /// process RegisterFrontend instruction
    pub fn process_register_frontend(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        kickback_rate: u64,
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // pool account information to withdraw
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this pool account
        let authority_info = next_account_info(account_info_iter)?;

        // frontend account
        let frontend_info = next_account_info(account_info_iter)?;

        // user deposit account
        let user_deposit_info = next_account_info(account_info_iter)?;

        // borrow frontend account data
        let pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        // borrow frontend account data
        let mut frontend_data = try_from_slice_unchecked::<FrontEnd>(&frontend_info.data.borrow())?;

        if frontend_data.pool_id_pubkey != *pool_id_info.key {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        if frontend_data.registered > 0 {
            return Err(StabilityPoolError::AlreadyRegistered.into());
        }

        if kickback_rate > DECIMAL_PRECISION {
            return Err(StabilityPoolError::InvalidKickbackRate.into());
        }

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // borrow user deposit data
        let user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        if user_deposit.initial_value > 0 {
            return Err(StabilityPoolError::HasDeposit.into());
        }

        frontend_data.kickback_rate = kickback_rate;
        frontend_data.registered = 1;

        // serialize/store this initialized stability pool again
        frontend_data
            .serialize(&mut *frontend_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }

    pub fn payout_solid_gains(
        community_issuance: &CommunityIssuance,
        pool_data: &StabilityPool,
        frontend:&FrontEnd,
        depositor_frontend:&FrontEnd,
        snapshots:&Snapshots,
        user_deposit:&Deposit
    ){
        // Pay out front end's SOLID gain
        let frontend_solid_gain = pool_data.get_frontend_solid_gain(snapshots,frontend);
        if frontend_solid_gain > 0 {
            // transfer SOLID token
            //_communityIssuance.sendLQTY(_frontEnd, frontEndLQTYGain);
        }

        let depositor_solid_gain = pool_data.get_depositor_solid_gain(snapshots, user_deposit, depositor_frontend);
        if depositor_solid_gain > 0 {
            // transfer SOLID token
            //_communityIssuance.sendLQTY(_depositor, depositorLQTYGain);
        }

    }

    /// get authority by given program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, StabilityPoolError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(StabilityPoolError::InvalidProgramAddress))
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
    pub fn get_pyth_product_quote_currency(pyth_product: &pyth::Product) -> Result<[u8; 32], ProgramError> {
        const LEN: usize = 14;
        const KEY: &[u8; LEN] = b"quote_currency";

        let mut start = 0;
        while start < pyth::PROD_ATTR_SIZE {
            let mut length = pyth_product.attr[start] as usize;
            start += 1;

            if length == LEN {
                let mut end = start + length;
                if end > pyth::PROD_ATTR_SIZE {
                    msg!("Pyth product attribute key length too long");
                    return Err(StabilityPoolError::InvalidOracleConfig.into());
                }

                let key = &pyth_product.attr[start..end];
                if key == KEY {
                    start += length;
                    length = pyth_product.attr[start] as usize;
                    start += 1;

                    end = start + length;
                    if length > 32 || end > pyth::PROD_ATTR_SIZE {
                        msg!("Pyth product quote currency value too long");
                        return Err(StabilityPoolError::InvalidOracleConfig.into());
                    }

                    let mut value = [0u8; 32];
                    value[0..length].copy_from_slice(&pyth_product.attr[start..end]);
                    return Ok(value);
                }
            }

            start += length;
            start += 1 + pyth_product.attr[start] as usize;
        }

        msg!("Pyth product quote currency not found");
        Err(StabilityPoolError::InvalidOracleConfig.into())
    }

    pub fn get_pyth_price(pyth_price_info: &AccountInfo, clock: &Clock) -> Result<Decimal, ProgramError> {
        const STALE_AFTER_SLOTS_ELAPSED: u64 = 5;

        let pyth_price_data = pyth_price_info.try_borrow_data()?;
        let pyth_price = pyth::load::<pyth::Price>(&pyth_price_data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        if pyth_price.ptype != pyth::PriceType::Price {
            msg!("Oracle price type is invalid");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }

        let slots_elapsed = clock
            .slot
            .checked_sub(pyth_price.valid_slot)
            .ok_or(StabilityPoolError::MathOverflow)?;
        if slots_elapsed >= STALE_AFTER_SLOTS_ELAPSED {
            msg!("Oracle price is stale");
            return Err(StabilityPoolError::InvalidOracleConfig.into());
        }

        let price: u64 = pyth_price.agg.price.try_into().map_err(|_| {
            msg!("Oracle price cannot be negative");
            StabilityPoolError::InvalidOracleConfig
        })?;

        let market_price = if pyth_price.expo >= 0 {
            let exponent = pyth_price
                .expo
                .try_into()
                .map_err(|_| StabilityPoolError::MathOverflow)?;
            let zeros = 10u64
                .checked_pow(exponent)
                .ok_or(StabilityPoolError::MathOverflow)?;
            Decimal::from(price).try_mul(zeros)?
        } else {
            let exponent = pyth_price
                .expo
                .checked_abs()
                .ok_or(StabilityPoolError::MathOverflow)?
                .try_into()
                .map_err(|_| StabilityPoolError::MathOverflow)?;
            let decimals = 10u64
                .checked_pow(exponent)
                .ok_or(StabilityPoolError::MathOverflow)?;
            Decimal::from(price).try_div(decimals)?
        };

        Ok(market_price)
    }


}

/// implement all stability pool error messages
impl PrintProgramError for StabilityPoolError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string());
    }
}