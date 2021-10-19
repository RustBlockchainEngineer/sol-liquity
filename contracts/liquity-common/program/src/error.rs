//! All error types for this program

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError, program_error::PrintProgramError, msg};
use thiserror::Error;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum LiquityError {
    // 0.
    /// The account cannot be initialized because it is already being used.
    #[error("AlreadyInUse")]
    AlreadyInUse,
    /// The program address provided doesn't match the value generated by the program.
    #[error("InvalidProgramAddress")]
    InvalidProgramAddress,
    /// The stability pool state is invalid.
    #[error("InvalidState")]
    InvalidState,
    /// pool token account's owner is invalid.
    #[error("InvalidOwner")]
    InvalidOwner,
    /// given pool token account isn't same with pool token account
    #[error("InvalidPoolToken")]
    InvalidPoolToken,
    /// given frontend wasn't registered
    #[error("NotRegistered")]
    NotRegistered,
    /// given frontend was registered already
    #[error("AlreadyRegistered")]
    AlreadyRegistered,
    /// given user has deposit balance, but it requires no deposit
    #[error("HasDeposit")]
    HasDeposit,
    /// given kickback rate is invalid
    #[error("InvalidKickbackRate")]
    InvalidKickbackRate,
    /// require no under collateralized troves. Cannot withdraw while there are troves with ICR < MCR
    #[error("RequireNoUnderCollateralizedTroves")]
    RequireNoUnderCollateralizedTroves,
    /// Oracle config is invalid
    #[error("Input oracle config is invalid")]
    InvalidOracleConfig,
    /// Math operation overflow
    #[error("Math operation overflow")]
    MathOverflow,
    /// Invalid account input
    #[error("Invalid account input")]
    InvalidAccountInput,
    /// invalid borrower operations.
    #[error("InvalidBorrwerOperations")]
    InvalidBorrwerOperations,
    /// borrower trove is not active yet
    #[error("TroveNotActive")]
    TroveNotActive,
    /// Nothing to liquidate
    #[error("Nothing to liquidate")]
    NothingToLiquidate,
    
    ///Max fee percentage must be between 0.5% and 100%
    #[error("Max fee percentage must be between 0.5% and 100%")]
    InvalidMaxFeePercentage,
    
    /// Max fee percentage must less than or equal to 100%
    #[error("Max fee percentage must less than or equal to 100%")]
    ExceedMaxFeePercentage,

    ///Cannot withdraw and add coll
    #[error("BorrowerOperations: Cannot withdraw and add coll")]
    ErrorSignularCollChange,
    
     ///There must be either a collateral change or a debt change
    #[error("BorrowerOps: There must be either a collateral change or a debt change")]
    ErrorNoneZeroAdjustment,
    
    ///Trove does not exist or is closed
    #[error("BorrowerOps: Trove does not exist or is closed")]
    ErrorTroveisNotActive,

    /// BorrowerOps: Trove is active
    #[error("BorrowerOps: Trove is active")]
    ErrorTroveisActive,

    /// BorrowerOps: Trove's net debt must be greater than minimum
    #[error("BorrowerOps: Trove's net debt must be greater than minimum")]
    ErrorMinNetDebt,

    /// BorrowerOps: Invalid Composite debt
    #[error("BorrowerOps: Invalid Composite debt")]
    InvalidCompositeDebt,
    

    /// BorrowerOps: Caller is not the BorrowerOperations contract
    #[error("TroveManager: Caller is not the BorrowerOperations contract")]
    NotTroveManagerSigner,
    

    /// TroveManager: Cannot redeem when TCR < MCR
    #[error("TroveManager: Cannot redeem when TCR < MCR")]
    TCRError,
    /// Amount must be greater than zero
    #[error("Amount must be greater than zero")]
    ZeroAmount,
    /// Fee exceeded provided maximum
    #[error("Fee exceeded provided maximum")]
    FeeExceeded,
}
impl From<LiquityError> for ProgramError {
    fn from(e: LiquityError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for LiquityError {
    fn type_of() -> &'static str {
        "Liquity Error"
    }
} 
/// implement all stability pool error messages
impl PrintProgramError for LiquityError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string());
    }
}