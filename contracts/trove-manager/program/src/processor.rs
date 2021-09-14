//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::TroveManagerError,
        instruction::{TroveManagerInstruction},
        state::{TroveManager},
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
            return Err(TroveManagerError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(TroveManagerError::InvalidOwner.into());
        }
        Ok(())
    } 
    /// process `ApplyPendingRewards` instruction.
    pub fn process_apply_pending_rewards(
        program_id: &Pubkey,        // this program id
        accounts: &[AccountInfo],   // all account informations
    ) -> ProgramResult {
        
        Ok(())
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

/// implement all farm error messages
impl PrintProgramError for TroveManagerError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            TroveManagerError::AlreadyInUse => msg!("Error: The account cannot be initialized because it is already being used"),
            TroveManagerError::InvalidProgramAddress => msg!("Error: The program address provided doesn't match the value generated by the program"),
            TroveManagerError::InvalidState => msg!("Error: The stake pool state is invalid"),
            TroveManagerError::InvalidOwner => msg!("Error: Pool token account's owner is invalid"),
        }
    }
} 