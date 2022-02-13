pub use brush::contracts::traits::psp22::PSP22Error;
// use core::fmt::Error;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PhalanxError {
    PSP22Error(PSP22Error),
    // MyError(Error),

    /// Overflow error when adding fees to Base Amount To Transfer
    BaseAmountOverflow,
    /// Overflow error when adding fees to Quoted Amount To Transfer
    QuotedAmountOverflow,
    
    /// Base Token transfer failed because of insufficient allowance
    InsufficientAllowanceForBase,
    /// Quoted Token transfer failed because of insufficient allowance
    InsufficientAllowanceForQuoted,

    NouvelleError,

}

impl From<PSP22Error> for PhalanxError {
    fn from(error: PSP22Error) -> Self {
        PhalanxError::PSP22Error(error)
    }
}

