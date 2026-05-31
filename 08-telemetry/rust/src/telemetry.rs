use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceCounters {
    pub name: String,
    pub in_octets: u64,
    pub out_octets: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BgpSession {
    pub peer: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub hostname: String,
    pub version: String,
    pub mgmt_ip: String,
    pub interfaces: Vec<InterfaceCounters>,
    pub bgp_peers: Vec<BgpSession>,
}