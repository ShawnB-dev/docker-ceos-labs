# Lab 04 – Spine‑Leaf Fabric (L3 Underlay)

## 1. Overview
In this lab, I built a minimal but realistic 2‑spine × 4‑leaf L3 underlay fabric using Arista cEOS containers running in Docker.

Key features I implemented:
- A pure L3 routed underlay
- OSPF area 0 across the entire fabric
- Loopback interfaces for the future overlay (EVPN in Lab 06)
- ECMP across both spines
- `docker-compose` wiring using isolated p2p networks

## 2. Folder Structure
```text
04-spine-leaf/
├── docker-compose.yml
├── configs/
│   ├── spine1.cfg
│   ├── spine2.cfg
│   ├── leaf1.cfg
│   ├── leaf2.cfg
│   ├── leaf3.cfg
│   └── leaf4.cfg
├── diagrams/
│   └── topology.png
├── README.md
└── README.jp.md
```

### Topology (ASCII)


```text
                         +----------------+
                         |    spine1      |
                         |  Lo0 10.255.0.1|
                         +--+--+--+--+----+
                            |  |  |  |
        10.0.1.0/31 --------+  |  |  +-------- 10.0.4.0/31
                               |  |
        10.0.2.0/31 -----------+  +----------- 10.0.3.0/31


                         +----------------+
                         |    spine2      |
                         |  Lo0 10.255.0.2|
                         +--+--+--+--+----+
                            |  |  |  |
        10.0.1.2/31 --------+  |  |  +-------- 10.0.4.2/31
                               |  |
        10.0.2.2/31 -----------+  +----------- 10.0.3.2/31


   +----------------+   +----------------+   +----------------+   +----------------+
   |     leaf1      |   |     leaf2      |   |     leaf3      |   |     leaf4      |
   | Lo0 10.255.1.1 |   | Lo0 10.255.1.2 |   | Lo0 10.255.1.3 |   | Lo0 10.255.1.4 |
   +-------+--------+   +-------+--------+   +-------+--------+   +-------+--------+
           |                    |                    |                    |
   10.0.1.1/31           10.0.2.1/31           10.0.3.1/31           10.0.4.1/31
           |                    |                    |                    |
           |                    |                    |                    |
   10.0.1.3/31           10.0.2.3/31           10.0.3.3/31           10.0.4.3/31
           |                    |                    |                    |
```

## Addressing plan

### Leaf ↔ Spine links (/31)

| Link |	Subnet |	Spine IP |	Leaf IP |
|:-----:|:-------:|:---------:|:--------:|
|leaf1 ↔ spine1|10.0.1.0/31|10.0.1.0|10.0.1.1|
|leaf1 ↔ spine2|10.0.1.2/31|10.0.1.2|10.0.1.3|
|leaf2 ↔ spine1|10.0.2.0/31|10.0.2.0|10.0.2.1|
|leaf2 ↔ spine2|10.0.2.2/31|10.0.2.2|10.0.2.3|
|leaf3 ↔ spine1|10.0.3.0/31|10.0.3.0|10.0.3.1|
|leaf3 ↔ spine2|10.0.3.2/31|10.0.3.2|10.0.3.3|
|leaf4 ↔ spine1|10.0.4.0/31|10.0.4.0|10.0.4.1|
|leaf4 ↔ spine2|10.0.4.2/31|10.0.4.2|10.0.4.3|


### Loopbacks (for future overlay)

|Node|Loopback0|
|:---:|:-------:|
|spine1|10.255.0.1/32|
|spine2|10.255.0.2/32|
|leaf1|10.255.1.1/32|
|leaf2|10.255.1.2/32|
|leaf3|10.255.1.3/32|
|leaf4|10.255.1.4/32|


## Routing
All links run OSPF area 0.

- Loopbacks are passive

- ECMP is automatic (two equal‑cost paths via spine1 and spine2)

- Management:
  - spine1: 10.99.0.11
  - spine2: 10.99.0.12
  - leaf1: 10.99.0.21
  - leaf2: 10.99.0.22
  - leaf3: 10.99.0.23
  - leaf4: 10.99.0.24

- No BGP yet — that begins in Lab 06 (EVPN)

## Bring‑up
1. Start the fabric
```bash
docker compose up -d
```
2. Verify containers
```bash
docker ps --format "table {{.Names}}\t{{.Status}}"
```
You should see all 6 nodes running.

## Validation
### Check OSPF neighbors
On any leaf:

```bash
leaf1# show ip ospf neighbor

Neighbor ID     Instance VRF Pri State   Dead Time Address     Interface
10.255.0.1      1        default 1  Full    00:00:33 10.0.1.0   Ethernet1
10.255.0.2      1        default 1  Full    00:00:33 10.0.1.2   Ethernet2
```
On a spine:

```bash
spine1# show ip ospf neighbor

Neighbor ID     Pri State   Dead Time Address     Interface
10.255.1.1      1   Full    00:00:33 10.0.1.1     Ethernet1
10.255.1.2      1   Full    00:00:33 10.0.2.1     Ethernet2
10.255.1.3      1   Full    00:00:33 10.0.3.1     Ethernet3
10.255.1.4      1   Full    00:00:33 10.0.4.1     Ethernet4
```

### Check routing table
On leaf1:

```bash
leaf1# show ip route

C 10.0.1.0/31 is directly connected, Ethernet1
C 10.0.1.2/31 is directly connected, Ethernet2
O 10.0.2.0/31 [110/20] via 10.0.1.0, Ethernet1
O 10.0.2.2/31 [110/20] via 10.0.1.2, Ethernet2
O 10.255.1.2/32 [110/30] via 10.0.1.0, Ethernet1
O 10.255.1.2/32 [110/30] via 10.0.1.2, Ethernet2
...
```
You should see ECMP routes (two next‑hops) for all remote leaves.

Ping loopbacks across the fabric
From leaf1:

```bash
leaf1# ping 10.255.1.4 source 10.255.1.1
```
Expected: success via ECMP.

## 8. Housekeeping & Stability
I have optimized this lab with several best practices:
- **Resource Limits**: All spine and leaf nodes are constrained to specific CPU and Memory limits to prevent host resource starvation during fabric boot.
- **Health Checks**: Every cEOS node includes a health check to ensure the switch management plane is fully initialized before attempting validation.

## 9. Cleanup
```bash
docker compose down
```