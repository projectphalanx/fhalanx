pub use crate::traits::orders::OrdersStorage;
pub use crate::traits::orders::*;
pub use crate::traits::errors::*;
pub use crate::traits::trade_tokens::*;
pub use crate::traits::phalanx_tokens::*;

pub use brush::contracts::psp22::*;
use brush::{
    traits::{
        AccountId,
        Balance,
    },
};
pub use ink_prelude::vec::Vec;


    impl<T: OrdersStorage + OrderStorageInternal + PhalanxPSP22TokensStorage + PhalanxPSP22TokensBaseInternal + TradePSP22Tokens> Orders for T {

        fn order(&mut self, side: Side, amount: Balance) -> Result<(), PSP22Error> {
        // Check Caller Account is valid?
        // If not, return an error?
        let acct = T::env().caller();
        // Locate an existing order for this account in bids and asks queues (only 1 order per account)
        // If order found, remove it. Will be replace by a the new order (can change side)
        // Maybe reuse order_cancel or call common internal
        match self._queue_account_get(acct) {
            Some(queue_pointer) => {
                self._queue_get_mut(queue_pointer.side).remove(queue_pointer.index);
            }
            None => {}
        };

        // Now accunt has no order in queue. Add order
        let order = OrderInternal::new(acct, amount);
        self._queue_get_mut(side).push(order);

        Ok(())
    }

    fn order_get(&self, acct: AccountId) -> Option<Order> {
        // Maybe better Result<Order> with error mgmt
        match self._queue_account_get(acct) {
            Some(qp) => {
                // code below is to handle usize=>u32 conv err. Should return some error.
                // But contract should not allow this to happen.
                let i: u32 = qp.index.try_into().map_or(0, |i| i);
                if i == 0 {
                    return None
                };

                Some(Order::new(acct, self._queue_get(qp.side)[qp.index].amount, qp.side, i))
            }
            None => None,
        }
    }

    fn order_cancel(&mut self) {
        let acct = T::env().caller();
        // Locate an existing order for this account in bids and asks queues (only 1 order per account)
        // If order found, remove it. Will be replace by a the new order (can change side)
        match self._queue_account_get(acct) {
            Some(queue_pointer) => {
                self._queue_get_mut(queue_pointer.side).remove(queue_pointer.index);
            }
            None => {}
        };
    }

    fn queue_get_length(&self, side: Side) -> u32 {
        self._queue_get(side).len() as u32
    }

    fn queue_get_total_amount(&self, side: Side) -> Balance {
        self._queue_get(side).iter().map(|x| x.amount).sum()
    }

    fn _queue_account_get(&self, acct: AccountId) -> Option<QueuePointer> {
        // let o_acct_pos_bid = OrdersStorage::get(self).bids.iter().position(|&x| x.acct == acct);
        let o_acct_pos_bid = self.bids().iter().position(|&x| x.acct == acct);
        match o_acct_pos_bid {
            Some(index) => Some(QueuePointer::new(Side::Bid, index)),
            None => {
                let o_acct_pos_ask = self.asks().iter().position(|&x| x.acct == acct);
                match o_acct_pos_ask {
                    Some(index) => Some(QueuePointer::new(Side::Ask, index)),
                    None => None,
                }
            }
        }
    }

    fn _queue_get_mut(&mut self, side: Side) -> &mut Vec<OrderInternal> {
        match side {
            Side::Bid => self.bids_mut(),
            Side::Ask => self.asks_mut(),
        }
    }

    fn _queue_get(&self, side: Side) -> &Vec<OrderInternal> {
        match side {
            Side::Bid => &self.bids(),
            Side::Ask => &self.asks(),
        }
    }

    fn clear_orders_at_price(&mut self, price: Balance) -> Result<(), PhalanxError> {
        // Repeat until 1 queue is empty
        //  Take 1st orders in both queues
        //  Create a transaction at price between the 2 accounts
        //  Remove smallest order and reduce largest order accordingly (or remove both is same size)

        // If any of the transactions fails?? (ex: lack of gas, network issues)
        // Probably resolve anyways and clear the queue
        // trade exec should be async in some ways. Not possible to wait for result else order book would be stuck

        loop {
            if self.bids().len() == 0 {
                break
            }
            if self.asks().len() == 0 {
                break
            }

            let trade_amount = core::cmp::min(
                self.bids().first().unwrap().amount,
                self.asks().first().unwrap().amount,
            );

            let result = self._trade_tokens(
                self.bids().first().unwrap().acct,
                self.asks().first().unwrap().acct,
                trade_amount,
                price,
            ); 
            
            // Check for success or error.
            if result.is_err() {
                // Lack of bidder or asker  allowance
                // => Cancel bidder or asker order then loop again

                // Overflow. Halve the amount to be cleared and do 2 trades?

                // Transfer error. Bad, bad. 
            }

            if self.bids().first().unwrap().amount == trade_amount {
                self.bids_mut().remove(0);
            } else {
                self.bids_mut().first_mut().unwrap().amount -= trade_amount;
            };
            if self.asks().first().unwrap().amount == trade_amount {
                self.asks_mut().remove(0);
            } else {
                self.asks_mut().first_mut().unwrap().amount -= trade_amount;
            };
        }

        Ok(())
    }

  
}

