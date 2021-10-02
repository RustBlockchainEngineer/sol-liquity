//! Program state processor
//! In here, All instructions are processed by Processor

use {
    liquity_common::{
        error::LiquityError,
        state::{
            StabilityPool,
            FrontEnd,
            Deposit,
            Snapshots,
            CommunityIssuance,
            Trove,
            TroveManager,
            RewardSnapshot,
            EpochToScale
        },
        constant::{
            DECIMAL_PRECISION,
            MCR,
        },
        pyth,
        math::{Decimal, TryDiv, TryMul},
        utils::*,
    },
    crate::{
        instruction::{StabilityPoolInstruction},
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
        if *authority_info.key != authority_id(program_id, pool_id_info.key, nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solusd_pool_info.owner != *program_id {
            return Err(LiquityError::InvalidOwner.into());
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

        // pool wsol gain token account
        let wsol_pool_gain_info = next_account_info(account_info_iter)?;

        // user wsol token account
        let wsol_user_info = next_account_info(account_info_iter)?;

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

        let epoch_to_scale_info = next_account_info(account_info_iter)?; 

        let epoch_to_plus_scale_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // clock account information to use timestamp
        let clock_sysvar_info = next_account_info(account_info_iter)?;

        // get clock from clock sysvar account information
        let clock = &Clock::from_account_info(clock_sysvar_info)?;

        // get current timestamp(second)
        let cur_timestamp: u128 = clock.unix_timestamp as u128;

        // borrow pool account data
        let mut pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        let mut community_issuance_data = try_from_slice_unchecked::<CommunityIssuance>(&community_issuance_info.data.borrow())?;

        let mut epoch_to_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_scale_info.data.borrow())?;
        let mut epoch_to_plus_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_plus_scale_info.data.borrow())?;

        let issue_solid = community_issuance_data.issue_solid(cur_timestamp);

        pool_data.trigger_solid_issuance(issue_solid, &mut epoch_to_scale);

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solusd_pool_info.owner != *program_id {
            return Err(LiquityError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *solusd_pool_info.key != pool_data.sol_usd_pool_token_pubkey {
            return Err(LiquityError::InvalidOwner.into());
        }

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // borrow frontend account data
        let frontend = try_from_slice_unchecked::<FrontEnd>(&frontend_info.data.borrow());
        let mut frontend_data = frontend.unwrap();

        if frontend_data.registered == 0 {
            return Err(LiquityError::NotRegistered.into());
        }

        if frontend_data.pool_id_pubkey == *pool_id_info.key {
            return Err(LiquityError::InvalidOwner.into());
        }
        if user_deposit.initial_value == 0 {
            user_deposit.front_end_tag = frontend_data.owner_pubkey;
        }

        // borrow depositor's frontend account data
        let depositor_frontend = try_from_slice_unchecked::<FrontEnd>(&depositor_frontend_info.data.borrow());
        let depositor_frontend_data = depositor_frontend.unwrap();

        let initial_deposit = user_deposit.initial_value;
        let mut snapshots_data = try_from_slice_unchecked::<Snapshots>(&snapshots_info.data.borrow())?;
        let depositor_sol_gain = pool_data.get_depositor_sol_gain(initial_deposit,&snapshots_data, &mut epoch_to_scale, &mut epoch_to_plus_scale);

        let compounded_solusd_deposit = pool_data.get_compounded_solusd_deposit(initial_deposit,&snapshots_data);
        let solusd_loss = initial_deposit - compounded_solusd_deposit;

        // First pay out any SOLID gains
        payout_solid_gains( &pool_data, &frontend_data, &depositor_frontend_data, &snapshots_data, &user_deposit, &mut epoch_to_scale, &mut epoch_to_plus_scale);

        // Update frontend stake
        let compounded_frontend_stake = pool_data.get_compounded_frontend_stake(&frontend_data,&snapshots_data);
        let new_frontend_stake = compounded_frontend_stake + amount as u128;
        
        // update frontend stake and snaphots
        frontend_data.update_frontend_stake(new_frontend_stake);
        snapshots_data.update_snapshots_with_frontendstake(new_frontend_stake,&pool_data, &mut epoch_to_scale);

        if amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            token_transfer(
                pool_id_info.key,
                token_program_info.clone(), 
                solusd_user_info.clone(), 
                solusd_pool_info.clone(), 
                user_transfer_authority_info.clone(), 
                pool_data.nonce, 
                amount
            )?;

            // update deposit and snapshots
            user_deposit.initial_value += amount as u128;
            snapshots_data.update_snapshots_with_deposit(new_frontend_stake,&pool_data, &mut epoch_to_scale)
        }
 
        if depositor_sol_gain > 0 {
            pool_data.sol -=  depositor_sol_gain;
            //send depositor_sol_gain to user (_sendETHGainToDepositor(depositorETHGain);) -- implemented below
            token_transfer(
                pool_id_info.key,
                token_program_info.clone(), 
                wsol_pool_gain_info.clone(), 
                wsol_user_info.clone(), 
                authority_info.clone(), 
                pool_data.nonce, 
                depositor_sol_gain.try_into().unwrap()
            )?;

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

        // pool sol token account
        let wsol_gain_pool_info = next_account_info(account_info_iter)?;

        // user sol token account
        let wsol_gain_user_info = next_account_info(account_info_iter)?;

        // trove manager
        let trove_manager_info = next_account_info(account_info_iter)?;

        // reward snapshots
        let reward_snapshots_info = next_account_info(account_info_iter)?;

        // lowest trove
        let lowest_trove_info = next_account_info(account_info_iter)?;

        // front end account info
        let frontend_info = next_account_info(account_info_iter)?;

        // depositor's frontend account info
        let depositor_frontend_info = next_account_info(account_info_iter)?;

        // snapshotsaccount info
        let snapshots_info = next_account_info(account_info_iter)?;

        // user community issuance account
        let community_issuance_info = next_account_info(account_info_iter)?;

        let epoch_to_scale_info = next_account_info(account_info_iter)?;
        let epoch_to_plus_scale_info = next_account_info(account_info_iter)?;
        

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        let pyth_product_info = next_account_info(account_info_iter)?;
        let pyth_price_info = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        // get current timestamp(second)
        let cur_timestamp: u128 = clock.unix_timestamp as u128;

        // borrow pool account data to initialize 
        let mut pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        let mut community_issuance_data = try_from_slice_unchecked::<CommunityIssuance>(&community_issuance_info.data.borrow())?;

        let mut epoch_to_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_scale_info.data.borrow())?;
        let mut epoch_to_plus_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_plus_scale_info.data.borrow())?;

        let issue_solid = community_issuance_data.issue_solid(cur_timestamp);

        pool_data.trigger_solid_issuance(issue_solid, &mut epoch_to_scale);

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solusd_pool_info.owner != *program_id {
            return Err(LiquityError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *solusd_pool_info.key != pool_data.sol_usd_pool_token_pubkey {
            return Err(LiquityError::InvalidOwner.into());
        }

        let market_price = get_market_price(
            pool_data.oracle_program_id,
            pool_data.quote_currency,
            pyth_product_info,
            pyth_price_info,
            clock
        )?;
        
        
        //address lowestTrove = sortedTroves.getLast(); ---implemented
        let trove_manager = try_from_slice_unchecked::<TroveManager>(&trove_manager_info.data.borrow())?;
        let mut lowest_trove = try_from_slice_unchecked::<Trove>(&lowest_trove_info.data.borrow())?;
        let mut reward_snapshots = try_from_slice_unchecked::<RewardSnapshot>(&reward_snapshots_info.data.borrow())?;

        //let icr = troveManager.getCurrentICR(lowestTrove, price); -- implemented
        let icr = get_current_icr(&trove_manager, &mut lowest_trove, &mut reward_snapshots, market_price);
        if icr < MCR {
            return Err(LiquityError::RequireNoUnderCollateralizedTroves.into());
        }
        
        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // check if given amount is small than deposit amount
        let mut _amount = amount;
        if user_deposit.initial_value < amount as u128 {
            _amount = user_deposit.initial_value.try_into().unwrap();
        }

        // borrow frontend account data
        let frontend = try_from_slice_unchecked::<FrontEnd>(&frontend_info.data.borrow());
        let mut frontend_data = frontend.unwrap();

        // borrow depositor's frontend account data
        let depositor_frontend = try_from_slice_unchecked::<FrontEnd>(&depositor_frontend_info.data.borrow());
        let depositor_frontend_data = depositor_frontend.unwrap();

        let initial_deposit = user_deposit.initial_value;
        let mut snapshots_data = try_from_slice_unchecked::<Snapshots>(&snapshots_info.data.borrow())?;
        let depositor_sol_gain = pool_data.get_depositor_sol_gain(initial_deposit,&snapshots_data, &mut epoch_to_scale, &mut epoch_to_plus_scale);

        let compounded_solusd_deposit = pool_data.get_compounded_solusd_deposit(initial_deposit,&snapshots_data);
        let solusd_to_withdraw = if _amount < compounded_solusd_deposit.try_into().unwrap() {_amount} else {compounded_solusd_deposit.try_into().unwrap()};
        let solusd_loss = initial_deposit - compounded_solusd_deposit;

        // First pay out any SOLID gains
        payout_solid_gains( &pool_data, &frontend_data, &depositor_frontend_data, &snapshots_data, &user_deposit, &mut epoch_to_scale, &mut epoch_to_plus_scale);

        // Update frontend stake
        let compounded_frontend_stake = pool_data.get_compounded_frontend_stake(&frontend_data,&snapshots_data);
        let new_frontend_stake = compounded_frontend_stake - solusd_to_withdraw as u128;
        
        // update frontend stake and snaphots
        frontend_data.update_frontend_stake(new_frontend_stake);
        snapshots_data.update_snapshots_with_frontendstake(new_frontend_stake,&pool_data, &mut epoch_to_scale);


        if solusd_to_withdraw > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            token_transfer(
                pool_id_info.key,
                token_program_info.clone(),
                solusd_pool_info.clone(),
                solusd_user_info.clone(),
                user_transfer_authority_info.clone(),
                pool_data.nonce,
                solusd_to_withdraw
            )?;
            user_deposit.initial_value -= solusd_to_withdraw as u128;
            snapshots_data.update_snapshots_with_deposit(new_frontend_stake,&pool_data, &mut epoch_to_scale)
        }

        if depositor_sol_gain > 0 {
            pool_data.sol -=  depositor_sol_gain;
            //send depositor_sol_gain to user (_sendETHGainToDepositor(depositorETHGain);) -- implemented
            token_transfer(
                pool_id_info.key,
                token_program_info.clone(),
                wsol_gain_pool_info.clone(),
                wsol_gain_user_info.clone(),
                user_transfer_authority_info.clone(),
                pool_data.nonce,
                depositor_sol_gain as u64
            )?;
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
        let account_info_iter = &mut accounts.iter();

        // pool account information to withdraw
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this pool account
        let authority_info = next_account_info(account_info_iter)?;

        // front end account info
        let frontend_info = next_account_info(account_info_iter)?;

        // depositor's frontend account info
        let depositor_frontend_info = next_account_info(account_info_iter)?;

        // snapshotsaccount info
        let snapshots_info = next_account_info(account_info_iter)?;

        // user community issuance account
        let community_issuance_info = next_account_info(account_info_iter)?;

        let epoch_to_scale_info = next_account_info(account_info_iter)?;
        let epoch_to_plus_scale_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;

        // get current timestamp(second)
        let cur_timestamp: u128 = clock.unix_timestamp as u128;

        // borrow pool account data to initialize 
        let mut pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        let mut community_issuance_data = try_from_slice_unchecked::<CommunityIssuance>(&community_issuance_info.data.borrow())?;

        let mut epoch_to_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_scale_info.data.borrow())?;
        let mut epoch_to_plus_scale = try_from_slice_unchecked::<EpochToScale>(&epoch_to_plus_scale_info.data.borrow())?;

        let issue_solid = community_issuance_data.issue_solid(cur_timestamp);

        pool_data.trigger_solid_issuance(issue_solid, &mut epoch_to_scale);

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // borrow user deposit data
        let user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // borrow frontend account data
        let frontend = try_from_slice_unchecked::<FrontEnd>(&frontend_info.data.borrow());
        let mut frontend_data = frontend.unwrap();

        // borrow depositor's frontend account data
        let depositor_frontend = try_from_slice_unchecked::<FrontEnd>(&depositor_frontend_info.data.borrow());
        let depositor_frontend_data = depositor_frontend.unwrap();

        let initial_deposit = user_deposit.initial_value;
        let mut snapshots_data = try_from_slice_unchecked::<Snapshots>(&snapshots_info.data.borrow())?;
        let depositor_sol_gain = pool_data.get_depositor_sol_gain(initial_deposit,&snapshots_data, &mut epoch_to_scale, &mut epoch_to_plus_scale);

        let compounded_solusd_deposit = pool_data.get_compounded_solusd_deposit(initial_deposit,&snapshots_data);
        let solusd_loss = initial_deposit - compounded_solusd_deposit;

        // First pay out any SOLID gains
        payout_solid_gains( &pool_data, &frontend_data, &depositor_frontend_data, &snapshots_data, &user_deposit, &mut epoch_to_scale, &mut epoch_to_plus_scale);

        // Update frontend stake
        let compounded_frontend_stake = pool_data.get_compounded_frontend_stake(&frontend_data,&snapshots_data);
        let new_frontend_stake = compounded_frontend_stake;
        
        // update frontend stake and snaphots
        frontend_data.update_frontend_stake(new_frontend_stake);
        snapshots_data.update_snapshots_with_frontendstake(new_frontend_stake,&pool_data, &mut epoch_to_scale);
        
        if depositor_sol_gain > 0 {
            pool_data.sol -=  depositor_sol_gain;
        }

        //borrowerOperations.moveETHGainToTrove{ value: depositorETHGain }(msg.sender, _upperHint, _lowerHint);

        // serialize/store this initialized stability pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
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
            return Err(LiquityError::InvalidOwner.into());
        }

        if frontend_data.registered > 0 {
            return Err(LiquityError::AlreadyRegistered.into());
        }

        if kickback_rate as u128 > DECIMAL_PRECISION {
            return Err(LiquityError::InvalidKickbackRate.into());
        }

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // borrow user deposit data
        let user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        if user_deposit.initial_value > 0 {
            return Err(LiquityError::HasDeposit.into());
        }

        frontend_data.kickback_rate = kickback_rate as u128;
        frontend_data.registered = 1;

        // serialize/store this initialized stability pool again
        frontend_data
            .serialize(&mut *frontend_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }
}
