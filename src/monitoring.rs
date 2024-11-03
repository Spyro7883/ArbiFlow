// src/monitoring.rs

#![cfg(not(target_arch = "wasm32"))]

use ethers::providers::{Http, Provider,Middleware};
use reqwest::Client;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

pub struct NetworkMetrics {
    pub gas_price: f64,
    pub pending_transactions: u32,
    pub latency: f64,
}

impl NetworkMetrics {
    pub fn new() -> Self {
        NetworkMetrics {
            gas_price: 0.0,
            pending_transactions: 0,
            latency: 0.0,
        }
    }
}

pub type SharedMetrics = Arc<Mutex<NetworkMetrics>>;

pub async fn fetch_gas_price(provider: &Provider<Http>, metrics: SharedMetrics) {
    match provider.get_gas_price().await {
        Ok(price) => {
            let mut metrics = metrics.lock().unwrap();
            metrics.gas_price = price.as_u64() as f64 / 1e9; // Convert wei to gwei
            println!("Gas Price: {} Gwei", metrics.gas_price);
        }
        Err(e) => eprintln!("Error fetching gas price: {:?}", e),
    }
}

pub async fn fetch_pending_transaction_count(metrics: SharedMetrics) {
    let client = Client::new();
    let response = client
        .post("http://localhost:8547")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "txpool_status",
            "params": [],
            "id": 1
        }))
        .send()
        .await;

    match response {
        Ok(response) => {
            if let Ok(json) = response.json::<Value>().await {
                let pending_count = json["result"]["pending"]
                    .as_str()
                    .and_then(|count| count.parse::<u32>().ok())
                    .unwrap_or(0);

                let mut metrics = metrics.lock().unwrap();
                metrics.pending_transactions = pending_count;
                println!("Pending Transactions: {}", metrics.pending_transactions);
            } else {
                eprintln!("Error parsing JSON response for pending transactions");
            }
        }
        Err(e) => eprintln!("Error fetching pending transactions: {:?}", e),
    }
}

pub async fn start_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    // Create an HTTP provider for Arbitrum RPC
    let provider = Provider::<Http>::try_from("http://localhost:8547")?;

    // Shared metrics instance
    let metrics = Arc::new(Mutex::new(NetworkMetrics::new()));

    // Periodically fetch metrics
    loop {
        // Clone metrics to share across tasks
        let metrics_clone = metrics.clone();

        // Fetch gas price
        fetch_gas_price(&provider, metrics_clone.clone()).await;

        // Fetch pending transaction count
        fetch_pending_transaction_count(metrics_clone.clone()).await;

        // Sleep for a specified period before fetching again (e.g., 10 seconds)
        sleep(Duration::from_secs(10)).await;
    }
}