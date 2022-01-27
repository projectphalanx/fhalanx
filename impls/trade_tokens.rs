pub use crate::traits::trade_tokens::*;
pub use brush::contracts::psp22::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
    },
};
pub use ink_prelude::vec::Vec;

use ink_storage::traits::SpreadLayout;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct TradedPSP22Tokens {
    pub base_token_account: AccountId,
    pub quoted_token_account: AccountId,
    pub phalanx_token_account: AccountId,
}

declare_storage_trait!(TradedPSP22TokensStorage, TradedPSP22Tokens);

// TO DO
// Authorize transfer
// Add Fee
// Add Phalanx reward to user

impl<T: TradedPSP22TokensStorage + TradePSP22TokensBaseInternal> TradePSP22Tokens for T {
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

pub trait TradePSP22TokensBaseInternal {
    fn base(&self) -> &PSP22Ref;
}

impl<T: TradedPSP22TokensStorage> TradePSP22TokensBaseInternal for T {
    fn base(&self) -> &PSP22Ref {
        &TradedPSP22TokensStorage::get(self).base_token_account
    }
}

pub trait TradePSP22TokensQuotedInternal {
    fn quoted(&self) -> &PSP22Ref;
}

impl<T: TradedPSP22TokensStorage> TradePSP22TokensQuotedInternal for T {
    fn quoted(&self) -> &PSP22Ref {
        &TradedPSP22TokensStorage::get(self).quoted_token_account
    }
}

pub trait TradePSP22TokensPhalanxInternal {
    fn phalanx(&self) -> &PSP22Ref;
}

impl<T: TradedPSP22TokensStorage> TradePSP22TokensPhalanxInternal for T {
    fn phalanx(&self) -> &PSP22Ref {
        &TradedPSP22TokensStorage::get(self).phalanx_token_account
    }
}

/////////////////////////////////////////////////:::
