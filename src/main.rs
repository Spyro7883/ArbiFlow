// src/main.rs

#[cfg(feature = "export-abi")]
fn main() {
    stylus_hello_world::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}

#[cfg(not(feature = "export-abi"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    stylus_hello_world::monitoring::start_monitoring().await
}
