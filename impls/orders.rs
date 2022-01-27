pub use crate::traits::orders::*;
pub use brush::contracts::psp22::*;
use brush::{
    declare_storage_trait,
    traits::{
        AccountId,
        Balance,
    },
};
pub use ink_prelude::vec::Vec;

use ink_storage::traits::{
    PackedLayout,
    SpreadLayout,
};

#[cfg(feature = "std")]
use ink_storage::traits::StorageLayout;

#[derive(
    Copy,
    PartialEq,
    //  Eq,
    Debug,
    Clone,
    scale::Encode,
    scale::Decode,
    PackedLayout,
    SpreadLayout,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
struct OrderInternal {
    acct: AccountId,
    amount: Balance,
}

impl OrderInternal {
    fn new(acct: AccountId, amount: Balance) -> Self {
        OrderInternal { acct, amount }
    }
}

#[derive(Default, Debug, SpreadLayout)]
#[cfg_attr(feature = "std", derive(StorageLayout))]
pub struct OrderStruct {
    bids: Vec<OrderInternal>,
    asks: Vec<OrderInternal>,
    base_token_account: AccountId,
}

declare_storage_trait!(OrdersStorage, OrderStruct);

impl<T: OrdersStorage + OrdersBaseInternal> Orders for T {
    fn order(&mut self, side: Side, amount: Balance) -> Result<(), PSP22Error> {
        // Check Caller Account is valid?
        // If not, return an error?
        let acct = T::env().caller();

        // // Locate an existing order for this account in bids and asks queues (only 1 order per account)
        // // If order found, remove it. Will be replace by a the new order (can change side)
        // match self._queue_account_get(acct) {
        //     Some(queue_pointer) => {
        //         match queue_pointer.side {
        //             Side::Bid => {
        //                 self.bids.remove(queue_pointer.index);
        //             }
        //             Side::Ask => {
        //                 self.asks.remove(queue_pointer.index);
        //             }
        //         }
        //     }
        //     None => {}
        // };

        // // Now accunt has no order in queue. Add order
        // let order = OrderInternal::new(acct, amount);
        // match side {
        //     Side::Bid => {
        //         self.bids().push(order);
        //     }
        //     Side::Ask => {
        //         self.asks().push(order);
        //     }
        // }

        Ok(())
    }

    // Internal function to find side and queue position of an account
    fn _queue_account_get(&self, acct: AccountId) -> Option<QueuePointer> {
        let mut o_queue_pointer = None;
        // let o_acct_pos_bid = self.bids.iter().position(|&x| x.acct == acct);
        // match o_acct_pos_bid {
        //     Some(index) => {
        //         o_queue_pointer = Some(QueuePointer::new(Side::Bid, index));
        //     }
        //     None => {
        //         let o_acct_pos_ask = self.asks().iter().position(|&x| x.acct == acct);
        //         match o_acct_pos_ask {
        //             Some(index) => {
        //                 o_queue_pointer = Some(QueuePointer::new(Side::Ask, index));
        //             }
        //             None => {}
        //         }
        //     }
        // }
        o_queue_pointer
    }
}

// pub trait OrdersBidsInternal {
//     fn bids(&self) -> &mut Vec<OrderInternal>;
// }

// impl<T: OrdersStorage> OrdersBidsInternal for T {
//     fn bids(&self) -> &mut Vec<OrderInternal> {
//         &OrdersStorage::get(self).bids
//     }
// }

// pub trait OrdersAsksInternal {
//     fn asks(&self) -> &Vec<OrderInternal>;
// }

// impl<T: OrdersStorage> OrdersAsksInternal for T {
//     fn asks(&self) -> &Vec<OrderInternal> {
//         &OrdersStorage::get(self).asks
//     }
// }

pub trait OrdersBaseInternal {
    fn base(&self) -> &PSP22Ref;
}

impl<T: OrdersStorage> OrdersBaseInternal for T {
    fn base(&self) -> &PSP22Ref {
        &OrdersStorage::get(self).base_token_account
    }
}
