extern crate alloc;

#[derive(Clone)]
pub struct CongestionControl {
    pub current_rate: u32,  // Current processing rate of transactions
    pub max_rate: u32,      // Maximum rate allowed
    pub _threshold: u32,     // Congestion threshold used to adjust rate
}

impl CongestionControl {
    /// Initializes a new instance of CongestionControl with a specified maximum rate
    pub fn new(max_rate: u32) -> Self {
        CongestionControl {
            current_rate: 1,          // Start with a conservative initial rate
            max_rate,                 // Set maximum rate
            _threshold: max_rate / 2,  // Define a threshold, for example, half of the max rate
        }
    }

    /// Adjusts the transaction rate based on the presence of congestion
    pub fn adjust_rate(&mut self, congestion_signal: bool) {
        if congestion_signal {
            // If there is congestion, reduce the rate to prevent overloading
            self.current_rate = (self.current_rate / 2).max(1); // Ensure the rate doesn't drop below 1
        } else if self.current_rate < self.max_rate {
            // If no congestion, attempt to increase the rate
            self.current_rate = (self.current_rate * 2).min(self.max_rate); // Double the rate up to the max rate
        }
    }

    /// Returns the current processing rate of transactions
    pub fn get_current_rate(&self) -> u32 {
        self.current_rate
    }
}