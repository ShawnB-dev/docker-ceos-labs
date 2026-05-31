use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting Lab 07 Automation Tool...");
    
    let devices = vec!["10.99.0.21", "10.99.0.22"];
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(10))
        .build()?;

    for device in devices {
        println!("\n--- Querying device: {} ---", device);
        match fetch_device_info(&client, device).await {
            Ok(info) => {
                println!("Hostname: {}", info["hostname"]);
                println!("Version:  {}", info["version"]);
                println!("VLANs:    {}", info["vlan_count"]);
                println!("BGP Peers: {}", info["bgp_summary"]);
            }
            Err(e) => eprintln!("❌ Error querying {}: {}", device, e),
        }
    }

    println!("\n✅ Automation run complete.");
    Ok(())
}

async fn fetch_device_info(client: &Client, ip: &str) -> Result<Value> {
    let url = format!("https://{}/command-api", ip);
    
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "runCmds",
        "params": {
            "format": "json",
            "cmds": [
                "show hostname",
                "show version",
                "show vlan",
                "show bgp evpn summary"
            ],
            "version": 1
        },
        "id": "lab07-auth"
    });

    let resp = client
        .post(url)
        .basic_auth("admin", Some("admin"))
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Extract results from JSON-RPC response
    let results = &resp["result"];
    
    let hostname = results[0]["hostname"]
        .as_str()
        .unwrap_or("unknown");
        
    let version = results[1]["version"]
        .as_str()
        .unwrap_or("unknown");

    let vlan_count = results[2]["vlans"]
        .as_object()
        .map(|v| v.len())
        .unwrap_or(0);

    // Extract BGP summary from the 4th command result (index 3)
    let bgp_summary = results[3]["vrfs"]["default"]["peers"]
        .as_object()
        .map(|peers| {
            let total = peers.len();
            let established = peers.values()
                .filter(|p| p["peerState"] == "Established")
                .count();
            format!("{}/{} Established", established, total)
        })
        .unwrap_or_else(|| "BGP Not Running".to_string());

    Ok(json!({
        "hostname": hostname,
        "version": version,
        "vlan_count": vlan_count,
        "bgp_summary": bgp_summary
    }))
}