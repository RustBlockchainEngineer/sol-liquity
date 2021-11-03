//! Program state processor
//! In here, All instructions are processed by Processor

use {
    liquity_common::{
        error::LiquityError,
        state::{SOLIDStaking,UserDeposit},
        utils::{
            authority_id,token_transfer,create_or_allocate_account_raw
        }
    },
    crate::{
        instruction::{SOLIDStakingInstruction},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{
            next_account_info,
            AccountInfo,
        },
        borsh::try_from_slice_unchecked,
        entrypoint::ProgramResult,
        program_error::ProgramError,
        msg,
        pubkey::Pubkey,
    },
};

const PREFIX:&str = "liquity-solid-staking";

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
        msg!("initializing solid staking ...");

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
        if *authority_info.key != authority_id(program_id, pool_id_info.key, nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        // borrow pool account data to initialize (mutable)
        let mut pool_data = try_from_slice_unchecked::<SOLIDStaking>(&pool_id_info.data.borrow())?;

        pool_data.token_program_pubkey = *token_program_info.key;
        pool_data.solid_pool_token_pubkey = *solid_pool_info.key;
        pool_data.nonce = nonce;
        
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
        msg!("staking ...");
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

        // user wallet
        let depositor_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;
        let rent_info = next_account_info(account_info_iter)?;
        let system_info = next_account_info(account_info_iter)?;

        // borrow pool account data
        let pool_data = try_from_slice_unchecked::<SOLIDStaking>(&pool_id_info.data.borrow())?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        let bump = Self::assert_pda(program_id, solid_pool_info.key, user_deposit_info.key, depositor_info.key, PREFIX)?;

        let size = std::mem::size_of::<UserDeposit>();

        let user_data_is_empty = user_deposit_info.data_is_empty();

        if user_data_is_empty {
            // Create account with enough space
            create_or_allocate_account_raw(
                *program_id,
                user_deposit_info,
                rent_info,
                system_info,
                depositor_info,
                size,
                &[
                    PREFIX.as_bytes(),
                    depositor_info.key.as_ref(),
                    program_id.as_ref(),
                    &[bump],
                ],
            )?;
        }

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<UserDeposit>(&user_deposit_info.data.borrow())?;

        if amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            token_transfer(
                pool_id_info.key,
                token_program_info.clone(), 
                solid_user_info.clone(), 
                solid_pool_info.clone(), 
                depositor_info.clone(), 
                pool_data.nonce, 
                amount
            )?;

            user_deposit.deposit_amount += amount;

            if user_data_is_empty {
                user_deposit.pool_id_pubkey = *pool_id_info.key;
                user_deposit.owner_pubkey = *depositor_info.key;
            }
            // serialize/store user info again
            user_deposit
                .serialize(&mut *user_deposit_info.data.borrow_mut())?;
        }

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
        msg!("unstaking ...");

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

        // user wallet
        let withdrawer_info = next_account_info(account_info_iter)?;

        // user deposit info
        let user_deposit_info = next_account_info(account_info_iter)?;

        // spl-token program address
        let token_program_info = next_account_info(account_info_iter)?;

        // borrow pool account data to initialize 
        let pool_data = try_from_slice_unchecked::<SOLIDStaking>(&pool_id_info.data.borrow())?;

        // check if this SOLID staking pool account was created by this program with authority and nonce
        // if fail, returns InvalidProgramAddress error
        if *authority_info.key != authority_id(program_id, pool_id_info.key, pool_data.nonce)? {
            return Err(LiquityError::InvalidProgramAddress.into());
        }

        Self::assert_pda(program_id,pool_id_info.key, user_deposit_info.key, withdrawer_info.key, PREFIX)?;

        // borrow user deposit data
        let mut user_deposit = try_from_slice_unchecked::<UserDeposit>(&user_deposit_info.data.borrow())?;

        // check if given amount is small than deposit amount
        let mut _amount = amount;
        if user_deposit.deposit_amount < amount {
            _amount = user_deposit.deposit_amount;
        }

        if _amount > 0 {
            // transfer solUSD token amount from user's solUSD token account to pool's solUSD token pool
            token_transfer(
                pool_id_info.key,
                token_program_info.clone(),
                solid_pool_info.clone(),
                solid_user_info.clone(),
                authority_info.clone(),
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
    
    /// check if pda address is correct
    pub fn assert_pda(program_id:&Pubkey, pool_key:&Pubkey, key: &Pubkey,authority: &Pubkey, tag: &str)->Result<u8, ProgramError>{
        let seeds = [
            tag.as_bytes(),
            authority.as_ref(),
            pool_key.as_ref(),
        ];
        
        let (pda_key, _bump) = Pubkey::find_program_address(&seeds, program_id);
        if pda_key != *key {
            return Err(LiquityError::InvalidPdaAddress.into());
        } 
        else {
            Ok(_bump)
        }
    }

}
