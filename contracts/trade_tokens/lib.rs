#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod trade_tokens {
    use phalanx::impls::trade_tokens::*;
use brush::{
        contracts::ownable::*,
    };


    #[ink(event)]
    pub struct Transferred {
      #[ink(topic)]
      from: Option<AccountId>,
      #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(storage)]
    #[derive(Default, OwnableStorage)]
    pub struct Contract {
        phalanx_tokens: PhalanxPSP22Tokens,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    brush::impl_storage_trait!(PhalanxPSP22TokensStorage, Contract, phalanx_tokens, PhalanxPSP22Tokens);

    impl TradePSP22Tokens for Contract {}
    impl Ownable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(base_token_account: AccountId, quoted_token_account: AccountId, phalanx_token_account: AccountId) -> Self {
            let mut instance = Self::default();
            instance.phalanx_tokens.base_token_account = base_token_account;
            instance.phalanx_tokens.quoted_token_account = quoted_token_account;
            instance.phalanx_tokens.phalanx_token_account = phalanx_token_account;
           
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }
    }
}
