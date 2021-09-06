//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::SOLIDStakingError,
        instruction::{SOLIDStakingInstruction},
        state::{SOLIDStaking,Snapshot,Deposit},
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
        let instruction = SOLIDStakingInstruction::try_from_slice(input)?;

        // determine instruction type
        match instruction {
            SOLIDStakingInstruction::Initialize{
                nonce,
            } => {
                // Instruction: Initialize
                Self::process_initialize(program_id, accounts, nonce)
            }
            SOLIDStakingInstruction::Stake(amount) => {
                // Instruction: Stake
                Self::process_stake(program_id, accounts, amount)
            }
            SOLIDStakingInstruction::Unstake(amount) => {
                // Instruction: Unstake
                Self::process_unstake(program_id, accounts, amount)
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
        
        // SOLID staking pool account info to create newly
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority of SOLID staking pool account
        let authority_info = next_account_info(account_info_iter)?;

        // pool SOLID token account
        let solid_pool_info = next_account_info(account_info_iter)?;

        // spl-token program account information
        let token_program_info = next_account_info(account_info_iter)?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, nonce)? {
            return Err(SOLIDStakingError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(SOLIDStakingError::InvalidOwner.into());
        }

        // borrow pool account data to initialize (mutable)
        let mut pool_data = try_from_slice_unchecked::<SOLIDStaking>(&pool_id_info.data.borrow())?;

        pool_data.token_program_pubkey = *token_program_info.key;
        pool_data.solid_pool_token_pubkey = *solid_pool_info.key;
        
        // serialize/store this initialized SOLID staking pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
    } 

    /// process ProvideToSP instruction
    pub fn process_stake(
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

        // pool SOLID token account
        let solid_pool_info = next_account_info(account_info_iter)?;

        // user SOLID token account
        let solid_user_info = next_account_info(account_info_iter)?;

        // user transfer authority
        let user_transfer_authority_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // snapshot account info
        let snapshot_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow pool account data
        let pool_data = try_from_slice_unchecked::<SOLIDStaking>(&pool_id_info.data.borrow())?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(SOLIDStakingError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(SOLIDStakingError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *solid_pool_info.key != pool_data.solid_pool_token_pubkey {
            return Err(SOLIDStakingError::InvalidOwner.into());
        }

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // borrow frontend account data
        let snapshot = try_from_slice_unchecked::<Snapshot>(&snapshot_info.data.borrow())?;

        if snapshot.pool_id_pubkey == *pool_id_info.key {
            return Err(SOLIDStakingError::InvalidOwner.into());
        }

        if amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            Self::token_transfer(
                pool_id_info.key,
                token_program_info.clone(), 
                solid_user_info.clone(), 
                solid_pool_info.clone(), 
                user_transfer_authority_info.clone(), 
                pool_data.nonce, 
                amount
            )?;

            user_deposit.deposit_amount += amount;
        }

        // serialize/store user info again
        user_deposit
            .serialize(&mut *user_deposit_info.data.borrow_mut())?;

        // serialize/store this initialized SOLID staking pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }

    /// process WithdrawFromSP instruction
    pub fn process_unstake(
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
        let pool_data = try_from_slice_unchecked::<SOLIDStaking>(&pool_id_info.data.borrow())?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(SOLIDStakingError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(SOLIDStakingError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        if *solid_pool_info.key != pool_data.solid_pool_token_pubkey {
            return Err(SOLIDStakingError::InvalidOwner.into());
        }

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<Deposit>(&user_deposit_info.data.borrow())?;

        // check if given amount is small than deposit amount
        let mut _amount = amount;
        if user_deposit.deposit_amount < amount {
            _amount = user_deposit.deposit_amount;
        }

        if _amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            Self::token_transfer(
                pool_id_info.key,
                token_program_info.clone(),
                solid_pool_info.clone(),
                solid_user_info.clone(),
                user_transfer_authority_info.clone(),
                pool_data.nonce,
                _amount
            )?;
            user_deposit.deposit_amount -= _amount;
        }

        // serialize/store user info again
        user_deposit
            .serialize(&mut *user_deposit_info.data.borrow_mut())?;

        // serialize/store this initialized SOLID staking pool again
        pool_data
            .serialize(&mut *pool_id_info.data.borrow_mut())
            .map_err(|e| e.into())
        
    }

    /// get authority by given program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, SOLIDStakingError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(SOLIDStakingError::InvalidProgramAddress))
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
impl PrintProgramError for SOLIDStakingError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            SOLIDStakingError::AlreadyInUse => msg!("Error: The account cannot be initialized because it is already being used"),
            SOLIDStakingError::InvalidProgramAddress => msg!("Error: The program address provided doesn't match the value generated by the program"),
            SOLIDStakingError::InvalidState => msg!("Error: The stake pool state is invalid"),
            SOLIDStakingError::InvalidOwner => msg!("Error: Pool token account's owner is invalid"),
            SOLIDStakingError::InvalidPoolToken => msg!("Error: Given pool token account isn't same with pool token account"),
        }
    }
} 
