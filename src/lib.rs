// src/lib.rs

#![no_std]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::*,
    console,
};

use core::convert::TryFrom; // Import TryFrom

mod congestion;
mod transaction;

use crate::congestion::CongestionControl;
use crate::transaction::Transaction;

sol_storage! {
    #[entrypoint]
    pub struct MonitoringContract {
        uint256 transaction_count;
        uint256 latest_value;
        uint256 current_rate;
        uint256 max_rate;
    }
}

#[public]
impl MonitoringContract {
    /// Initializes the congestion control parameters
    pub fn initialize_congestion_control(&mut self, max_rate: U256) {
        self.current_rate.set(U256::from(1));
        self.max_rate.set(max_rate);
    }

    /// Adjusts the congestion control rate based on a congestion signal
    pub fn adjust_congestion_control(&mut self, congestion_signal: bool) {
        let current_rate_u256 = self.current_rate.get();
        let max_rate_u256 = self.max_rate.get();

        // Convert U256 to u32 safely
         let current_rate: u32 = u32::try_from(current_rate_u256).expect("current_rate exceeds u32");
        let max_rate: u32 = u32::try_from(max_rate_u256).expect("max_rate exceeds u32");

        // Reconstruct the CongestionControl struct
        let mut congestion_control = CongestionControl {
            current_rate,
            max_rate,
            _threshold: max_rate / 2,
        };

        // Adjust the rate based on congestion
        congestion_control.adjust_rate(congestion_signal);

        // Update the current rate in storage
        self.current_rate
            .set(U256::from(congestion_control.current_rate));
    }

    /// Processes a transaction
    pub fn process_transaction(&mut self, sender: Address, recipient: Address, amount: U256) {
        let _transaction = Transaction {
            sender,
            recipient,
            amount,
        };

        console!("Processing transaction: {}", _transaction);

        let current_count = self.transaction_count.get();
        self.transaction_count.set(current_count + U256::from(1));
    }

    /// Gets the transaction count from storage.
    pub fn get_transaction_count(&self) -> U256 {
        self.transaction_count.get()
    }

    /// Gets the latest value from storage.
    pub fn get_latest_value(&self) -> U256 {
        self.latest_value.get()
    }

    /// Updates the latest value in storage and increments the transaction count.
    pub fn update_value(&mut self, new_value: U256) {
        let current_count = self.transaction_count.get();
        self.transaction_count.set(current_count + U256::from(1));
        self.latest_value.set(new_value);

        console!("Event: ValueUpdated, Value: {}", new_value);
    }

    /// Increments the transaction count without changing the value.
    pub fn increment_transaction_count(&mut self) {
        let current_count = self.transaction_count.get();
        self.transaction_count.set(current_count + U256::from(1));
        console!(
            "Event: TransactionCountIncremented, Count: {}",
            current_count + U256::from(1)
        );
    }
}
