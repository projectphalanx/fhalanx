pub use crate::traits::trade_tokens::*;
pub use crate::traits::phalanx_tokens::*;

pub use brush::contracts::psp22::*;
use brush::{
    traits::{
        AccountId,
        Balance,
    },
};
pub use ink_prelude::vec::Vec;


// TO DO
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
    ) -> Result<(), PSP22Error> {
        self.base()
            .transfer_from(bid_account, ask_account, base_amount, Vec::<u8>::new())?;
        self.quoted()
            .transfer_from(ask_account, bid_account, base_amount * price, Vec::<u8>::new())
    }
}

