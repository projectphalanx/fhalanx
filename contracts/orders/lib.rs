#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod orders {
    use phalanx::impls::orders::*;
    use brush::{
        contracts::ownable::*,
    };

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
    impl Ownable for Contract {}

    impl Contract {

        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_owner(caller);
            instance
        }
    }
}
