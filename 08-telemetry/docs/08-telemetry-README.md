# Lab 08 – Telemetry (Rust + Arista eAPI Observability)
In this lab, I built a lightweight telemetry pipeline that collects operational state from Arista cEOS devices using Rust and eAPI (HTTPS/JSON‑RPC).
This continues the automation work I established in Lab 07 and extends it into real‑time observability.

My goal was to demonstrate how a modern data‑center environment can expose real‑time state (*interface counters, routing protocol status, device metadata*) and how a Rust‑based collector can unify that data into a single telemetry snapshot.

## 1.Overview
- In this lab, I implemented:

- A Rust‑based telemetry collector

- eAPI‑enabled cEOS nodes

    - JSON‑RPC polling for:

    - Hostname and EOS version

    - Interface counters

    - BGP session state

    - OSPF neighbor state

- A unified TelemetrySnapshot struct for each device

- Automated execution via Docker Compose

This lab completes the “core automation + observability” foundation of my network engineering toolkit.

## 2. Topology
I used a simple management‑only topology to keep the focus on telemetry:

```Code
                mgmt-net (10.99.0.0/24)
   ┌──────────┬──────────┬──────────┬──────────┐
   │          │          │          │          │
10.99.0.21  10.99.0.22  10.99.0.23  10.99.0.24   10.99.0.50
 ceos-1     ceos-2     ceos-3     ceos-4    rust-telemetry
   │          │          │          │          │
   └──────────┴──────────┴──────────┴──────────┴──────────┘
```
- **ceos‑1 / ceos‑2 / ceos‑3 / ceos‑4**
 Arista cEOS nodes with eAPI enabled

- **rust‑telemetry**  
Rust application that polls all devices and prints structured telemetry

## 3. Folder Structure
```text
08-telemetry/
├── docker-compose.yml
├── configs/
│   ├── ceos-1.cfg
│   ├── ceos-2.cfg
│   ├── ceos-3.cfg
│   └── ceos-4.cfg
├── rust-telemetry/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── eapi_client.rs
│       └── telemetry.rs
├── diagrams/
│   └── topology.png
├── README.md
└── README.ja.md
```
## 4. Telemetry Collector (Rust)
I extended the eAPI client from Lab 07 and added a dedicated telemetry module that implements:

### System Information
- Hostname
- EOS version

### Interface Counters
- in/out octets
- in/out errors

### BGP Session State
- peer address
- ASN
- session state

### OSPF Neighbor State
- neighbor ID
- interface
- adjacency state
### Unified Snapshot
```rust
pub struct TelemetrySnapshot {
    pub hostname: String,
    pub version: String,
    pub interfaces: Vec<InterfaceCounters>,
    pub bgp: Vec<BgpSession>,
    pub ospf: Vec<OspfNeighbor>,
}
```
Each device produces one snapshot per polling cycle.

## 5. Running the Lab
Start the environment
```bash
cd 08-telemetry
docker compose up -d
```
This launches:

- 4× cEOS nodes

- 1× Rust telemetry collector

The Rust container automatically runs:

```bash
cargo run --release
```
## View telemetry output
```bash
docker logs -f rust-telemetry
```
Example output:

```bash
=== Telemetry for ceos-1 (10.0.0.11) ===
TelemetrySnapshot {
    hostname: "ceos-1",
    version: "4.36.0.1F",
    interfaces: [...],
    bgp: [...],
    ospf: [...],
}
```
This confirms that the collector is successfully retrieving structured state from each device.

## 6. Validation
Check eAPI is reachable
From your host:

```bash
curl -k -u telemetry:telemetry123 https://10.99.0.11/command-api
```
Expected: `JSON‑RPC` error (normal), confirming connectivity.

## Check interface counters
On any cEOS:

```bash
show interfaces counters
```
Compare with Rust output.

## Check BGP/OSPF (if configured)
```bash
show ip bgp summary
show ip ospf neighbor
```
The Rust telemetry matched the live device state.

## 7. Future Extensions
This lab forms the foundation for more advanced observabiliity work and tooling.

The next steps I plan to explore:

### Add Prometheus Exporter
Expose /metrics from Rust and scrape with Prometheus.

### Add Grafana Dashboard
Visualize interface counters and routing protocol state.

### Add Time‑Series Storage
InfluxDB or VictoriaMetrics for historical telemetry.

### Add EVPN Telemetry
Pull:
- EVPN MAC/IP routes
- VTEP state
- VNI statistics

### Add Drift Detection
Compare expected vs. actual state across devices.

## 8. Cleanup
```bash
docker compose down
```
## 9. Summary
Lab 08 demonstrates:

- How to enable eAPI on Arista cEOS

- How to build a Rust telemetry collector

- How to gather structured operational data

- How to integrate automation + observability in a modern network lab
