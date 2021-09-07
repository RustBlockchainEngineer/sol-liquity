//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::StabilityPoolError,
        instruction::{StabilityPoolInstruction},
        state::{StabilityPool,FrontEnd,Deposit},
        liquitiy_math::{
            DECIMAL_PRECISION,
        }
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
        let sol_usd_pool_info = next_account_info(account_info_iter)?;

        // spl-token program account information
        let token_program_info = next_account_info(account_info_iter)?;

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *sol_usd_pool_info.owner != *program_id {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // borrow pool account data to initialize (mutable)
        let mut pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        pool_data.token_program_pubkey = *token_program_info.key;
        pool_data.sol_usd_pool_token_pubkey = *sol_usd_pool_info.key;
        
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
        let sol_usd_pool_info = next_account_info(account_info_iter)?;

        // user solUsd token account
        let sol_usd_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // front end account info
        let frontend_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow pool account data
        let pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *sol_usd_pool_info.owner != *program_id {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *sol_usd_pool_info.key != pool_data.sol_usd_pool_token_pubkey {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // borrow frontend account data
        let frontend_data = try_from_slice_unchecked::<FrontEnd>(&frontend_info.data.borrow())?;

        if frontend_data.registered == 0 {
            return Err(StabilityPoolError::NotRegistered.into());
        }

        if frontend_data.pool_id_pubkey == *pool_id_info.key {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        if user_deposit.initial_value == 0 {
            user_deposit.front_end_tag = frontend_data.owner_pubkey;
        }

        if amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            Self::token_transfer(
                pool_id_info.key,
                token_program_info.clone(), 
                sol_usd_user_info.clone(), 
                sol_usd_pool_info.clone(), 
                user_transfer_authority_info.clone(), 
                pool_data.nonce, 
                amount
            )?;

            user_deposit.initial_value += amount;
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
        let sol_usd_pool_info = next_account_info(account_info_iter)?;

        // user solUsd token account
        let sol_usd_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow pool account data to initialize 
        let pool_data = try_from_slice_unchecked::<StabilityPool>(&pool_id_info.data.borrow())?;

        // check if this stability pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(StabilityPoolError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *sol_usd_pool_info.owner != *program_id {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *sol_usd_pool_info.key != pool_data.sol_usd_pool_token_pubkey {
            return Err(StabilityPoolError::InvalidOwner.into());
        }

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
                sol_usd_pool_info.clone(),
                sol_usd_user_info.clone(),
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
    
}

/// implement all farm error messages
impl PrintProgramError for StabilityPoolError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            StabilityPoolError::AlreadyInUse => msg!("Error: The account cannot be initialized because it is already being used"),
            StabilityPoolError::InvalidProgramAddress => msg!("Error: The program address provided doesn't match the value generated by the program"),
            StabilityPoolError::InvalidState => msg!("Error: The stake pool state is invalid"),
            StabilityPoolError::InvalidOwner => msg!("Error: Pool token account's owner is invalid"),
            StabilityPoolError::InvalidPoolToken => msg!("Error: Given pool token account isn't same with pool token account"),
            StabilityPoolError::NotRegistered => msg!("Error: Given frontend was not registered"),
            StabilityPoolError::AlreadyRegistered => msg!("Error: Given frontend was registered already"),
            StabilityPoolError::HasDeposit => msg!("Error: Given user has deposit balance already, but it requires no deposit"),
            StabilityPoolError::InvalidKickbackRate => msg!("Error: Given kickback rate is invalid"),
        }
    }
} 
