pub use brush::contracts::traits::psp22::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
    // Balance,
},
};

use ink_storage::traits::SpreadLayout;

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct PhalanxPSP22Tokens {
    pub base_token_account: AccountId,
    pub quoted_token_account: AccountId,
    pub phalanx_token_account: AccountId,
}

declare_storage_trait!(PhalanxPSP22TokensStorage, PhalanxPSP22Tokens);

pub trait PhalanxPSP22TokensBaseInternal {
    fn base(&self) -> &PSP22Ref;
}

impl<T: PhalanxPSP22TokensStorage> PhalanxPSP22TokensBaseInternal for T {
    fn base(&self) -> &PSP22Ref {
        &PhalanxPSP22TokensStorage::get(self).base_token_account
    }
}

pub trait PhalanxPSP22TokensQuotedInternal {
    fn quoted(&self) -> &PSP22Ref;
}

impl<T: PhalanxPSP22TokensStorage> PhalanxPSP22TokensQuotedInternal for T {
    fn quoted(&self) -> &PSP22Ref {
        &PhalanxPSP22TokensStorage::get(self).quoted_token_account
    }
}

pub trait PhalanxPSP22TokensPhalanxInternal {
    fn phalanx(&self) -> &PSP22Ref;
}

impl<T: PhalanxPSP22TokensStorage> PhalanxPSP22TokensPhalanxInternal for T {
    fn phalanx(&self) -> &PSP22Ref {
        &PhalanxPSP22TokensStorage::get(self).phalanx_token_account
    }
}