#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod base_token {
    use phalanx::impls::psp22_token::*;

    #[ink(storage)]
    #[derive(Default, PSP22Storage)]
    pub struct Contract {
        #[PSP22StorageField]
        psp22: PSP22Data,
    }

    impl PSP22 for Contract {}

    // It is need only to verify during compilation that `Contract` implements `Pausable` and `PSP22`
    impl Psp22Token for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();
            assert!(instance._mint(Self::env().caller(), total_supply).is_ok());
            instance
        }
    }
}
