//! All error types for this program

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the StabilityPool program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MathError {
    /// Math operation overflow
    #[error("Math operation overflow")]
    MathOverflow,
}
impl From<MathError> for ProgramError {
    fn from(e: MathError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for MathError {
    fn type_of() -> &'static str {
        "Math Error"
    }
} 