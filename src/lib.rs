// src/lib.rs

extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*, console};

// Include the monitoring module
#[cfg(not(target_arch = "wasm32"))]
pub mod monitoring;

// Define persistent storage using the Solidity ABI.
// `MonitoringContract` will be the entry point.
sol_storage! {
    #[entrypoint]
    pub struct MonitoringContract {
        uint256 transaction_count;  // Track the number of contract interactions
        uint256 latest_value;       // Track the latest value updated in the contract
    }
}

/// Declare that `MonitoringContract` is a contract with the following external methods.
#[public]
impl MonitoringContract {
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

        // Emit a log event for monitoring purposes
        console!("Event: ValueUpdated, Value: {}", new_value);
    }

    /// Increments the transaction count without changing the value (for monitoring interactions).
    pub fn increment_transaction_count(&mut self) {
        let current_count = self.transaction_count.get();
        self.transaction_count.set(current_count + U256::from(1));
        console!(
            "Event: TransactionCountIncremented, Count: {}",
            current_count + U256::from(1)
        );
    }
}


