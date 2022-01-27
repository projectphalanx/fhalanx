pub use brush::contracts::traits::psp22::*;
use brush::traits::{
    AccountId,
    Balance,
};
use ink_storage::traits::SpreadLayout;

#[derive(
    Copy,
    PartialEq,
    // Eq,
    Debug,
    Clone,
    scale::Encode,
    scale::Decode,
    // PackedLayout,
    SpreadLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ::ink_storage::traits::StorageLayout))]
pub enum Side {
    Bid,
    Ask,
}

pub struct QueuePointer {
    pub side: Side,
    pub index: usize,
}
impl QueuePointer {
    pub fn new(side: Side, index: usize) -> Self {
        QueuePointer { side, index }
    }
}

#[brush::wrapper]
pub type OrdersRef = dyn Orders;

#[brush::trait_definition]
pub trait Orders {
    // The "order" function adds a bid or a ask order for the signer acct
    // acct can only have a single order in the queues. With the call of "order", any previous order of acct
    // is deleted

    // Initial order prototype. No market pair specified. Only one queue exists.
    #[ink(message)]
    fn order(&mut self, side: Side, amount: Balance) -> Result<(), PSP22Error>;

    // Internal function to find side and queue position of an account
    fn _queue_account_get(&self, acct: AccountId) -> Option<QueuePointer>;
}
