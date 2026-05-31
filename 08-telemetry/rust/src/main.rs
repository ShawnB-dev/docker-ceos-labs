mod telemetry;
mod eapi_client;

use reqwest::Client;
use anyhow::Result;
use tokio::time::{sleep, Duration};
use chrono::Local;

#[tokio::main]
async fn main() -> Result<()> {
    println!("📈 Starting Lab 08 Telemetry Collector (Modular)...");
    
    let devices = vec!["10.99.0.21", "10.99.0.22", "10.99.0.23", "10.99.0.24"];
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(5))
        .build()?;

    loop {
        println!("\n--- Collection Interval: {} ---", Local::now().format("%Y-%m-%d %H:%M:%S"));
        
        for device in &devices {
            match eapi_client::get_snapshot(&client, device).await {
                Ok(snapshot) => {
                    println!("=== Telemetry for {} ({}) ===", snapshot.hostname, snapshot.mgmt_ip);
                    println!("{:#?}", snapshot);
                }
                Err(e) => eprintln!("❌ Error on {}: {}", device, e),
            }
        }
        sleep(Duration::from_secs(30)).await;
    }
}