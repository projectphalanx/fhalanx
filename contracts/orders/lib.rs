#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod orders {
    use phalanx::impls::orders::*;
    use phalanx::impls::trade_tokens::*;
    use brush::contracts::ownable::*;

    
    #[ink(storage)]
    #[derive(Default, OwnableStorage)]
    pub struct Contract {
        phalanx_tokens: PhalanxPSP22Tokens,
        order_queue: OrderStruct,
        #[OwnableStorageField]
        ownable: OwnableData,
    }

    brush::impl_storage_trait!(OrdersStorage, Contract, order_queue, OrderStruct);
    brush::impl_storage_trait!(PhalanxPSP22TokensStorage, Contract, phalanx_tokens, PhalanxPSP22Tokens);

    impl Orders for Contract {}
    impl TradePSP22Tokens for Contract {}
    impl Ownable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            base_token_account: AccountId, 
            quoted_token_account: AccountId, 
            phalanx_token_account: AccountId,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            
            // Define owner as caller
            instance._init_with_owner(caller);

            // Initialize data ///Do we need these?
            instance.phalanx_tokens.base_token_account = base_token_account;
            instance.phalanx_tokens.quoted_token_account = quoted_token_account;
            instance.phalanx_tokens.phalanx_token_account = phalanx_token_account;
      
            instance
        }
    }
}
