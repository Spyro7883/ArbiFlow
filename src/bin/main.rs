// src/bin/main.rs

#[cfg(not(feature = "export-abi"))]
use arbi_flow::monitoring;

#[cfg(feature = "export-abi")]
fn main() {
    arbi_flow::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}

#[cfg(not(feature = "export-abi"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    monitoring::start_monitoring().await
}
