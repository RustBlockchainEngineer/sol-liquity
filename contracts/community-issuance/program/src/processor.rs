//! Program state processor
//! In here, All instructions are processed by Processor

use {
    liquity_common::{
        state::{CommunityIssuance},
        error::LiquityError,
        utils::{authority_id}
    },
    crate::{
        instruction::{CommunityIssuanceInstruction},
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
        let instruction = CommunityIssuanceInstruction::try_from_slice(input)?;

        // determine instruction type
        match instruction {
            CommunityIssuanceInstruction::Initialize{
                nonce,
            } => {
                // Instruction: Initialize
                Self::process_initialize(program_id, accounts, nonce)
            }
            CommunityIssuanceInstruction::IssueSOLID => {
                // Instruction: Stake
                Self::process_issue_solid(program_id, accounts)
            }
            CommunityIssuanceInstruction::SendSOLID(amount) => {
                // Instruction: Stake
                Self::process_send_solid(program_id, accounts, amount)
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
        
        // Community Issuance account info to create newly
        let community_issuance_info = next_account_info(account_info_iter)?;

        // authority of SOLID staking pool account
        let authority_info = next_account_info(account_info_iter)?;

        // SOLID token account
        let solid_token_info = next_account_info(account_info_iter)?;

        // Stability Pool account
        let stability_pool_id_info = next_account_info(account_info_iter)?;

        // spl-token program account information
        let token_program_info = next_account_info(account_info_iter)?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, community_issuance_info.key, nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // check if pool token account's owner is this program
        // if not, returns InvalidOwner error
        if *community_issuance_info.owner != *program_id {
            return Err(LiquityError::InvalidOwner.into());
        }

        // borrow pool account data to initialize (mutable)
        let mut community_issuance_data = try_from_slice_unchecked::<CommunityIssuance>(&community_issuance_info.data.borrow())?;

        // When SOLID Token deployed, it should have transferred CommunityIssuance's SOLID entitlement
        // ...

        community_issuance_data.token_program_pubkey = *token_program_info.key;
        community_issuance_data.solid_token_pubkey = *solid_token_info.key;
        community_issuance_data.stability_pool_pubkey = *stability_pool_id_info.key;
        
        // serialize/store this initialized Community Issuance again
        community_issuance_data
            .serialize(&mut *community_issuance_info.data.borrow_mut())
            .map_err(|e| e.into())
    } 

    /// process ProvideToSP instruction
    pub fn process_issue_solid(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

        // pool account information to provide
        let pool_id_info = next_account_info(account_info_iter)?;

        // authority information of this farm account
        let authority_info = next_account_info(account_info_iter)?;

        

        Ok(())
        
    }

    /// process ProvideToSP instruction
    pub fn process_send_solid(
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
        let pool_data = try_from_slice_unchecked::<CommunityIssuance>(&pool_id_info.data.borrow())?;

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

        Ok(())
        
    }

    
}
