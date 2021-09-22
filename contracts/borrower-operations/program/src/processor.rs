//! Program state processor
//! In here, All instructions are processed by Processor

use {
    crate::{
        error::BorrowerOperationsError,
        instruction::{BorrowerOperationsInstruction},
        state::{BorrowerOperations,LocalVariablesAdjustTrove,LocalVariablesOpenTrove,ContractsCache},
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
                solusd_amount
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
        let borrower_info = next_account_info(account_info_iter)?;
        let authority_info = next_account_info(account_info_iter)?;
        let trove_manager_info = next_account_info(account_info_iter)?;
        let active_pool_info = next_account_info(account_info_iter)?;
        let default_pool_info = next_account_info(account_info_iter)?;
        let stability_pool_info = next_account_info(account_info_iter)?;
        let gas_pool_info = next_account_info(account_info_iter)?;
        let coll_surplus_pool_info = next_account_info(account_info_iter)?;
        let price_feed_info = next_account_info(account_info_iter)?;
        let sorted_troves_info = next_account_info(account_info_iter)?;
        let solusd_token_info = next_account_info(account_info_iter)?;
        let solid_staking_info = next_account_info(account_info_iter)?;
        let token_program_info = next_account_info(account_info_iter)?;

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }
        
        let token_program_id = *token_program_info.key;
        
        let borrower_obj = BorrowerOperations{
            is_initialized:true,
            nonce,
            trove_manager_id:*trove_manager_info.key,
            active_pool_id: *active_pool_info.key,
            default_pool_id: *default_pool_info.key,
            stability_pool_id: *stability_pool_info.key,
            gas_pool_id: *gas_pool_info.key,
            coll_surplus_pool_id: *coll_surplus_pool_info.key,
            price_feed_id: *price_feed_info.key,
            sorted_troves_id: *sorted_troves_info.key,
            solusd_token_id: *solusd_token_info.key,
            solid_staking_id:*solid_staking_info.key,
            token_program_id
        };

        borrower_obj.serialize(&mut &mut borrower_info.data.borrow_mut()[..])?;
        Ok(())
    } 

    /// process OpenTrove instruction
    pub fn process_open_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        max_fee_percentage: u64,
        solusd_amount: u64,
    ) -> ProgramResult {
        
        let account_info_iter = &mut accounts.iter();
        let borrower_info = next_account_info(account_info_iter)?;
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

        if *authority_info.key != Self::authority_id(program_id, borrower_info.key, nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }

        let token_program_id = *token_program_info.key;
        
        let vars = LocalVariablesOpenTrove{
            priceFeed
        }

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
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(BorrowerOperationsError::InvalidOwner.into());
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
        if *authority_info.key != Self::authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(BorrowerOperationsError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *solid_pool_info.owner != *program_id {
            return Err(BorrowerOperationsError::InvalidOwner.into());
        }

        // check if given pool token account is same with pool token account
        Ok(())
        
    }

    /// get authority by given program address.
    pub fn authority_id(
        program_id: &Pubkey,
        my_info: &Pubkey,
        nonce: u8,
    ) -> Result<Pubkey, BorrowerOperationsError> {
        Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
            .or(Err(BorrowerOperationsError::InvalidProgramAddress))
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
impl PrintProgramError for BorrowerOperationsError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            BorrowerOperationsError::AlreadyInUse => msg!("Error: The account cannot be initialized because it is already being used"),
            BorrowerOperationsError::InvalidProgramAddress => msg!("Error: The program address provided doesn't match the value generated by the program"),
            BorrowerOperationsError::InvalidState => msg!("Error: The stake pool state is invalid"),
            BorrowerOperationsError::InvalidOwner => msg!("Error: Pool token account's owner is invalid"),
        }
    }
} 
