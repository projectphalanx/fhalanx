pub use crate::traits::trade_tokens::*;
pub use crate::traits::phalanx_tokens::*;
pub use crate::traits::errors::*;



// pub use brush::modifiers;
pub use brush::contracts::psp22::*;
// pub use brush::contracts::ownable::*;
use brush::{
    traits::{
        AccountId,
        Balance,
    },
};
pub use ink_prelude::vec::Vec;



// Trade tokens as extrinsic is mainly for testing purpose.
// This function is to ba called internally when a match is done.
// As extrinsic it should only be called by the contract owner
// TO DO
// Both transfer should only take place if both amounts are allowed. Need to be checked first
// Authorize transfer
// Add Fee
// Add Phalanx reward to user

    impl<T: PhalanxPSP22TokensStorage + PhalanxPSP22TokensBaseInternal> TradePSP22Tokens for T {

    fn trade_tokens(
        &mut self,
        bid_account: AccountId,
        ask_account: AccountId,
        base_amount: Balance,
        price: Balance,
    ) -> Result<(), PhalanxError> {
        self._trade_tokens(bid_account, ask_account, base_amount, price)
    }

    /// Returns BaseAmountOverflow if the base amount plus fees causes overflow
    /// Returns QuotedAmountOverflow if the quoted amount plus fees causes overflow
    /// Returns InsufficientAllowanceForBase if the bidder does not have enough base token allowance
    /// Returns InsufficientAllowanceForQuoted if the asker does not have enough quoted token allowance
    fn _trade_tokens(
        &mut self,
        bid_account: AccountId,
        ask_account: AccountId,
        base_amount: Balance,
        price: Balance,
    ) -> Result<(), PhalanxError> {
        // If allowances are not available, return an error
        let base_transfer_fee = 0;
        let quoted_transfer_fee = 0;
        let base_phalanx_fee = 0;
        let quoted_phalanx_fee = 0;

        let base_amount_to_transfer = 
            base_amount
            .checked_add( base_transfer_fee + base_phalanx_fee)
            .ok_or(0).map_err(|_| {PhalanxError::BaseAmountOverflow})?;            

        let quoted_amount_to_transfer = 
            base_amount
            .checked_mul(price)
            .ok_or(0).map_err(|_| {PhalanxError::QuotedAmountOverflow})?
            .checked_add(quoted_transfer_fee + quoted_phalanx_fee)
            .ok_or(0).map_err(|_| {PhalanxError::QuotedAmountOverflow})?;

        let bid_base_allowance = self.base().allowance(bid_account,Self::env().caller());
        let ask_quoted_allowance = self.quoted().allowance(ask_account,Self::env().caller());
        
        if base_amount_to_transfer > bid_base_allowance { return Err(PhalanxError::InsufficientAllowanceForBase) };
        if quoted_amount_to_transfer > ask_quoted_allowance { return Err(PhalanxError::InsufficientAllowanceForBase) };

        // This is not safe. A token swap or escrow pattern should be used? 
        self.base()
            .transfer_from(bid_account, ask_account, base_amount, Vec::<u8>::new())?;
        self.quoted()
            .transfer_from(ask_account, bid_account, base_amount * price, Vec::<u8>::new())?;
        Ok(())    
    }

}

