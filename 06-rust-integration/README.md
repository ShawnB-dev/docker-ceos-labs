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

### Management Addressing
- spine-1: `10.99.0.11`
- spine-2: `10.99.0.12`
- leaf-1:  `10.99.0.21`
- leaf-2:  `10.99.0.22`
- rust-toolkit: `10.99.0.50` (for eAPI access)


### Interpreting Traceroute Output in ECMP Environments
When using `traceroute` in an ECMP (Equal-Cost Multi-Path) environment like our spine-leaf fabric, you might observe a few key behaviors:
- **Varying Paths**: Different runs of `traceroute` to the same destination might show different intermediate hops. This is because the load-balancing algorithm (often hash-based) can select different equal-cost paths for each new flow or even for different packets within the same flow, depending on the implementation.
- **Multiple Next-Hops**: Some `traceroute` implementations (or by observing the output closely) might show multiple IP addresses for a single hop number. This indicates that the router at that hop has multiple equal-cost paths to the destination and is load-balancing traffic across them.
- **Packet Loss**: Occasionally, you might see "packet loss" or asterisks (`*`) for a hop. This can sometimes be a side effect of load-balancing, where different packets of the `traceroute` probe take different paths, and not all return paths are perfectly symmetrical or timely. It doesn't necessarily indicate a network problem if the final destination is still reachable.

In our Lab 06 topology, `traceroute` to a remote leaf's loopback (e.g., `10.255.1.2`) should show paths through either `spine-1` or `spine-2`, demonstrating the active ECMP underlay.

## 4. Quick Start

```bash
docker compose up -d
docker logs -f rust-toolkit
```

Then run each scenario in `scenarios/.`
