# Lab 08 – Telemetry (Rust + Arista eAPI Observability)

## 1. Overview
In this lab, I built a lightweight telemetry pipeline that collects operational state from Arista cEOS devices using Rust and eAPI (HTTPS/JSON‑RPC). This continues the automation work I established in Lab 07 and extends it into real‑time observability.

## 2. Topology
- ceos-1 (10.99.0.21)
- ceos-2 (10.99.0.22)
- ceos-3 (10.99.0.23)
- ceos-4 (10.99.0.24)
- rust-telemetry (10.99.0.50)

## 3. Implementation Details
- **Periodic Polling**: The Rust application uses `tokio::time::sleep` to poll the network every 30 seconds.
- **JSON-RPC parsing**: I parsed complex nested JSON structures to extract `inOctets`, `outOctets`, and BGP `peerState`.
- **Observability**: Real-time stats are printed to the console/container logs.

## 4. Housekeeping & Stability
I have optimized this lab with several best practices:
- **Resource Limits**: Each container is constrained to specific CPU and Memory limits to prevent host starvation.
- **Health Checks**: The nodes include health checks to ensure the management plane is ready before collection begins.

## 5. Quick Start
```bash
cd 08-telemetry
docker compose up -d
docker logs -f rust-telemetry
```

## 5. Next Steps
- Integrate an external database like InfluxDB to store the collected metrics.
- Add threshold alerts (e.g., if a BGP peer leaves the 'Established' state).