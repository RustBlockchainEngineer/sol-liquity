/// Main Entrypoint and declaration file

use solana_program::{
    account_info::{ AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::PrintProgramError,
    pubkey::Pubkey,
};
use liquity_common::error::*;
/// module declaration
/// 
/// instruction module
pub mod instruction;
/// processor module
pub mod processor;

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the Yield Farming program was loaded into
    accounts: &[AccountInfo], // account informations
    _instruction_data: &[u8], // Instruction data
) -> ProgramResult {
    // process a passed instruction
    if let Err(error) = processor::Processor::process(program_id, accounts, _instruction_data) {
        
        // catch the error so we can print it
        error.print::<LiquityError>();
        Err(error)
    } else {
        // processed successfully
        Ok(())
    }
    
}
