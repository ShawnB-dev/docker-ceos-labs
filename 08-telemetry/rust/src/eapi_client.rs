use reqwest::Client;
use serde_json::{json, Value};
use anyhow::Result;
use crate::telemetry::{TelemetrySnapshot, InterfaceCounters, BgpSession};

pub async fn get_snapshot(client: &Client, ip: &str) -> Result<TelemetrySnapshot> {
    let url = format!("https://{}/command-api", ip);
    
    let payload = json!({
        "jsonrpc": "2.0",
        "method": "runCmds",
        "params": {
            "format": "json",
            "cmds": ["show hostname", "show version", "show interfaces Ethernet1 counters", "show bgp evpn summary"],
            "version": 1
        },
        "id": "telemetry-poll"
    });

    let resp = client
        .post(url)
        .basic_auth("admin", Some("admin"))
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let results = &resp["result"];
    
    let hostname = results[0]["hostname"].as_str().unwrap_or("unknown").to_string();
    let version = results[1]["version"].as_str().unwrap_or("unknown").to_string();

    let mut interfaces = Vec::new();
    if let Some(eth1) = results[2]["interfaces"]["Ethernet1"].as_object() {
        interfaces.push(InterfaceCounters {
            name: "Ethernet1".to_string(),
            in_octets: eth1["inOctets"].as_u64().unwrap_or(0),
            out_octets: eth1["outOctets"].as_u64().unwrap_or(0),
        });
    }

    let mut bgp_peers = Vec::new();
    if let Some(peers) = results[3]["vrfs"]["default"]["peers"].as_object() {
        for (peer_ip, data) in peers {
            bgp_peers.push(BgpSession {
                peer: peer_ip.clone(),
                state: data["peerState"].as_str().unwrap_or("Unknown").to_string(),
            });
        }
    }

    Ok(TelemetrySnapshot {
        hostname,
        version,
        mgmt_ip: ip.to_string(),
        interfaces,
        bgp_peers,
    })
}