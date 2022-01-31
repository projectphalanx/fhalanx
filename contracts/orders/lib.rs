#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]




#[brush::contract]
pub mod orders {
    use phalanx::impls::orders::*;
    use brush::contracts::ownable::*;
    use trade_tokens_contract::trade_tokens::TradeTokensContract;
    use ink_storage::Lazy;

    
    #[ink(storage)]
    #[derive(Default, OwnableStorage)]
    pub struct Contract {
        phalanx_tokens: PhalanxPSP22Tokens,
        order_queue: OrderStruct,
        #[OwnableStorageField]
        ownable: OwnableData,
        trade_tokens_contract: Lazy<TradeTokensContract>,
    }

    brush::impl_storage_trait!(OrdersStorage, Contract, order_queue, OrderStruct);
    brush::impl_storage_trait!(PhalanxPSP22TokensStorage, Contract, phalanx_tokens, PhalanxPSP22Tokens);

    impl Orders for Contract {}
    impl Ownable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            base_token_account: AccountId, 
            quoted_token_account: AccountId, 
            phalanx_token_account: AccountId,
            code_hash: Hash,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            let total_balance = Self::env().balance();
            
            // Define owner as caller
            instance._init_with_owner(caller);

            // Initialize data ///Do we need these?
            instance.phalanx_tokens.base_token_account = base_token_account;
            instance.phalanx_tokens.quoted_token_account = quoted_token_account;
            instance.phalanx_tokens.phalanx_token_account = phalanx_token_account;
            
            // let trade_tokens_contract = TradeTokensContract::new(base_token_account,quoted_token_account,phalanx_token_account)
            // .endowment(total_balance/4)
            // .code_hash(code_hash)
            // .instantiate();
            // .expect("failed at instantiating the `OtherContract` contract");

            instance
        }
    }
}
