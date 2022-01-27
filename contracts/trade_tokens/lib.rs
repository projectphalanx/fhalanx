#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod trade_tokens {
    use phalanx::impls::trade_tokens::*;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Contract {
        traded_tokens: TradedPSP22Tokens,
    }

    brush::impl_storage_trait!(TradedPSP22TokensStorage, Contract, traded_tokens, TradedPSP22Tokens);

    impl TradePSP22Tokens for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(base_token_account: AccountId, quoted_token_account: AccountId, phalanx_token_account: AccountId) -> Self {
            Self {
                // bids: Vec::new(),
                // asks: Vec::new(),
                traded_tokens: TradedPSP22Tokens {
                    base_token_account: base_token_account,
                    quoted_token_account: quoted_token_account,
                    phalanx_token_account: phalanx_token_account,
                },
            }
        }
    }
}
