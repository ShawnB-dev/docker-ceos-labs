# Lab 06 – Rust Toolkit Integration Across L2, L3, and EVPN

## 1. Overview

In this lab, I demonstrated how my custom Rust networking toolkit behaves across:

- Layer 2 switching (VLAN, STP)
- Layer 3 routing (OSPF, BGP)
- EVPN-VXLAN fabrics (Type-2/3 routes, VTEPs, VNIs)

I validated the toolkit’s ability to observe, scan, and analyze real network behavior.

## 2. Scenarios

### 1. L2 Behavior
- ARP scanning
- MAC learning
- Broadcast domain analysis

### 2. L3 Routing
- Traceroute path discovery
- Port scanning across routed hops
- ICMP sweeps

### 3. EVPN-VXLAN
- ARP suppression
- EVPN MAC/IP route learning
- VXLAN encapsulation visibility

## 3. Rust Tools Demonstrated

- `arp_scan`
- `pingsweep`
- `portscan`
- `sniffer`
- `traceroute`

## 4. Quick Start

```bash
docker compose up -d
docker logs -f rust-toolkit
```

Then run each scenario in `scenarios/.`
