# Lab 03 – OSPF Underlay, iBGP Overlay, and eBGP Edge

## 1. Overview

In this lab, I built a realistic routed fabric using three cEOS routers and one edge router.

- I configured OSPF as the underlay IGP between all point-to-point links.
- I established an iBGP full-mesh between loopbacks for the overlay control plane.
- I set up eBGP peering to a small “ISP” edge router, which advertises a public-style prefix.

This pattern mirrors modern enterprise and data center designs (underlay/overlay) and shows how internal routing interacts with an external BGP edge.

## 2. Topology

- ceos-1, ceos-2, ceos-3 form a routed triangle
- OSPF area 0 on all inter-switch links
- Loopbacks used as router-IDs and iBGP update sources
- edge router connected to ceos-1 via eBGP, advertising 203.0.113.0/24
```text
                       +----------------------+
                       |       ceos-1         |
                       |   Loopback 1.1.1.1   |
                       |   OSPF Area 0        |
                       +----+------------+----+
                            |            |
                            |            |
                     10.0.12.1/30   10.0.31.1/30
                            |            |
                            |            |
                       +----+------------+----+
                       |       ceos-2         |
                       |   Loopback 2.2.2.2   |
                       +----+------------+----+
                            |            |
                            |            |
                     10.0.23.1/30   10.0.23.2/30
                            |            |
                            |            |
                       +----+------------+----+
                       |       ceos-3         |
                       |   Loopback 3.3.3.3   |
                       +----------------------+
```
## Addressing

- Loopbacks:
  - ceos-1: 1.1.1.1/32
  - ceos-2: 2.2.2.2/32
  - ceos-3: 3.3.3.3/32
  - edge: 203.0.113.1/32

- P2P links:
  - ceos-1 ↔ ceos-2: 10.0.12.0/30
  - ceos-2 ↔ ceos-3: 10.0.23.0/30
  - ceos-3 ↔ ceos-1: 10.0.31.0/30
  - ceos-1 ↔ edge: 10.0.14.0/30

- Management:
  - ceos-1: 10.99.0.11
  - ceos-2: 10.99.0.12
  - ceos-3: 10.99.0.13
  - edge:   10.99.0.14

## Protocol Design

### OSPF Underlay

- Process 1 on all cEOS routers
- All P2P links and loopbacks in area 0
- Verifications:
  - `show ip ospf neighbor`
  - `show ip route ospf`
  - `show ip ospf database`

### iBGP Overlay

- AS 65000 on ceos-1/2/3
- Full-mesh iBGP between loopbacks:
  - 1.1.1.1 ↔ 2.2.2.2
  - 1.1.1.1 ↔ 3.3.3.3
  - 2.2.2.2 ↔ 3.3.3.3
- Verifications:
  - `show ip bgp summary`
  - `show ip bgp`
  - `show ip bgp neighbors`

### eBGP Edge

- edge in AS 65001
- eBGP between ceos-1 (AS 65000) and edge (AS 65001)
- edge originates 203.0.113.0/24
- Verifications:
  - `show ip bgp` on ceos-1/2/3
  - `show ip route 203.0.113.0/24`

## Quick Start

```bash
cd 03-routing
docker compose up -d

docker exec -it ceos-1 Cli
docker exec -it ceos-2 Cli
docker exec -it ceos-3 Cli
docker exec -it edge Cli
```
## 6. Verification Steps

## 7. Housekeeping & Stability
I have optimized this lab with several best practices:
- **Resource Limits**: Each cEOS container is constrained to specific CPU and Memory limits to prevent host starvation.
- **Health Checks**: The cEOS nodes include health checks to ensure the switch management plane is fully initialized before attempting verification.

## 8. Cleanup
```bash
docker compose down
```

### OSPF neighbors

```bash
show ip ospf neighbor
show ip route ospf
```
### iBGP sessions

```bash
show ip bgp summary
show ip bgp
```
### eBGP edge

```bash
show ip bgp
show ip route 203.0.113.0/24
```
### Rust toolkit usage

From `rust-toolkit` container:

```bash
cd /tools
./traceroute 203.0.113.1
./portscan 203.0.113.1 1-1024
```