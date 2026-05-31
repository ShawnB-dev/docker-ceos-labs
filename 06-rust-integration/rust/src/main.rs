use std::process::Command;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Starting Lab 06 Integration Scenarios...");

    // Scenario 1: L2 ARP Scan over EVPN
    run_scenario("ARP Scan (VLAN 10)", "/tools/arp_scan", &["192.168.10.0/24"])?;

    // Scenario 2: L3 Port Scan across routed fabric
    run_scenario("Port Scan (Leaf-1 eAPI)", "/tools/portscan", &["10.99.0.21", "443"])?;

    // Scenario 3: ICMP Sweep
    run_scenario("Ping Sweep (VLAN 10)", "/tools/pingsweep", &["192.168.10.0/24"])?;

    // Scenario 4: Traceroute path discovery across the EVPN fabric
    run_scenario("Traceroute (To Leaf-2 Loopback)", "/tools/traceroute", &["10.255.1.2"])?;

    // Scenario 5: Packet Sniffing on the local interface
    run_scenario("Network Sniffer (eth0)", "/tools/sniffer", &["eth0"])?;

    println!("✅ All scenarios executed.");
    Ok(())
}

fn run_scenario(name: &str, bin: &str, args: &[&str]) -> Result<()> {
    println!("\n--- Running Scenario: {} ---", name);
    let output = Command::new(bin)
        .args(args)
        .output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("❌ Scenario failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}