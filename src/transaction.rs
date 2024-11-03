extern crate alloc;

use stylus_sdk::alloy_primitives::{U256, Address};
use core::fmt; // Import fmt from core

/// Transaction structure to represent each transaction
#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: Address,    // Address of the sender
    pub recipient: Address, // Address of the recipient
    pub amount: U256,       // Amount being transferred
}

// Implement Display for Transaction to use in console messages
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Transaction {{ sender: {:?}, recipient: {:?}, amount: {} }}",
            self.sender, self.recipient, self.amount
        )
    }
}