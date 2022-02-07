pub use brush::contracts::traits::psp22::*;
use brush::traits::{
    AccountId,
    Balance,
};



#[brush::wrapper]
pub type TradePSP22TokensRef = dyn TradePSP22Tokens;

#[brush::trait_definition]
pub trait TradePSP22Tokens {

    
   /// Transfer base_amount of base token from bid_user to ask_user
    /// and simultaneously
    /// Transfer base_amount * price of quoted token from ask_user to bid_user
    #[ink(message)]
    fn trade_tokens(
        &mut self,
        bid_account: AccountId,
        ask_account: AccountId,
        base_amount: Balance,
        price: Balance,
    ) -> Result<(), PSP22Error>;
}
