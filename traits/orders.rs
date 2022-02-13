pub use brush::contracts::traits::psp22::*;
pub use crate::traits::errors::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
    },
};
use ink_prelude::vec::Vec;
use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};


#[derive(Copy, PartialEq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout)]
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

#[derive(Copy, PartialEq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Order {
    pub acct: AccountId,
    pub amount: Balance,
    pub side: Side,
    pub position: u32,
}

impl Order {
    pub fn new(acct: AccountId, amount: Balance, side: Side, position: u32) -> Self {
        Order {
            acct,
            amount,
            side,
            position,
        }
    }
}

#[derive(Copy, PartialEq, Debug, Clone, scale::Encode, scale::Decode, PackedLayout, SpreadLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct OrderInternal {
    pub acct: AccountId,
    pub amount: Balance,
}

impl OrderInternal {
    pub fn new(acct: AccountId, amount: Balance) -> Self {
        OrderInternal { acct, amount }
    }
}


#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct OrderStruct {
    pub bids: Vec<OrderInternal>,
    pub asks: Vec<OrderInternal>,
    // base_token_account: AccountId,
}

declare_storage_trait!(OrdersStorage, OrderStruct);

pub trait OrderStorageInternal {
    fn bids(&self) -> &Vec<OrderInternal>;
    fn asks(&self) -> &Vec<OrderInternal>;
    fn bids_mut(&mut self) -> &mut Vec<OrderInternal>;
    fn asks_mut(&mut self) -> &mut Vec<OrderInternal>;
}

impl<T: OrdersStorage> OrderStorageInternal for T {
    fn bids(&self) -> &Vec<OrderInternal> {
        &OrdersStorage::get(self).bids
    }
    fn asks(&self) -> &Vec<OrderInternal> {
        &OrdersStorage::get(self).asks
    }
    fn bids_mut(&mut self) -> &mut Vec<OrderInternal> {
        &mut OrdersStorage::get_mut(self).bids
    }
    fn asks_mut(&mut self) -> &mut Vec<OrderInternal> {
        &mut OrdersStorage::get_mut(self).asks
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

    fn _queue_get_mut(&mut self, side: Side) -> &mut Vec<OrderInternal>;
    fn _queue_get(&self, side: Side) -> &Vec<OrderInternal>;

    // Extrnal get command to retrieve current order for account
    #[ink(message)]
    fn order_get(&self, acct: AccountId) -> Option<Order>;

    // Caller can cancel his order
    #[ink(message)]
    fn order_cancel(&mut self);

    // Return length of queue
    #[ink(message)]
    fn queue_get_length(&self, side: Side) -> u32;

    // Return size of queue
    #[ink(message)]
    fn queue_get_total_amount(&self, side: Side) -> Balance;

    // Exgtern function to matches bids and asks and triggers transaction at the current price
    // Should onlybe called by owner (ownable)
    #[ink(message)]
    fn clear_orders_at_price(&mut self, price: Balance) -> Result<(), PhalanxError>;

}
