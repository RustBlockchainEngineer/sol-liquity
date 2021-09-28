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
            }
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
            BorrowerOperationsInstruction::OpenTrove(amount) => {
                // Instruction: OpenTrove
                Self::process_open_trove(program_id, accounts, amount)
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

        Ok(())
    } 

    /// process OpenTrove instruction
    pub fn process_open_trove(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        // get account informations
        let account_info_iter = &mut accounts.iter();

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
