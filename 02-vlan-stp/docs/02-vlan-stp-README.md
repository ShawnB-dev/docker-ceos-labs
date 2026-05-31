# Lab 02 – VLAN + STP (Multi-Switch L2 Network)

## 1. Overview

In this lab, I extended Lab 01 by introducing a multi-switch Layer 2 network. I implemented VLANs, trunking, and Rapid-PVST spanning tree. I configured two cEOS switches to form a small L2 domain, with `ceos-1` acting as the STP root bridge. I distributed hosts across the switches to demonstrate MAC learning, STP behavior, and inter-switch forwarding.

## 2. Topology

- **ceos-1**: I configured `ceos-1` as the STP root bridge and provided SVIs for VLAN 10 and VLAN 20.
- **ceos-2**: I set up `ceos-2` as an access switch with a trunk uplink.
- **host-a**: I placed `host-a` in VLAN 10.
- **host-b**: I placed `host-b` in VLAN 10.
- **rust-toolkit**: I placed `rust-toolkit` in VLAN 20.
  

```text
                   mgmt-net (10.99.0.0/24)
                           |
                +-----------------------+
                |       ceos-1          |
                |  STP Root Bridge      |
                |  Vlan10: 192.168.10.1 |
                |  Vlan20: 192.168.20.1 |
                +----------+------------+
                           |
                           | Trunk (VLANs 10,20)
                           |
                +----------+------------+
                |        ceos-2         |
                |  Access + Trunk       |
                +-----+-----------+-----+
                      |           |
                      |           |
            Access (10)|           | Access (20)
                      |           |
                +-----------+   +-----------------+
                |  host-a   |   |  rust-toolkit   |
                |10.10/24   |   |20.10/24         |
                +-----------+   +-----------------+

                +-----------+
                |  host-b   |
                |10.11/24   |
                +-----------+
```

(See diagrams/topology.png)

## 3. Addressing

### VLAN 10 – USERS
- Subnet: `192.168.10.0/24`
- Gateway: `192.168.10.1` (ceos-1)
- host-a: `192.168.10.10`
- host-b: `192.168.10.11`

### VLAN 20 – TOOLS
- Subnet: `192.168.20.0/24`
- Gateway: `192.168.20.1` (ceos-1)
- rust-toolkit: `192.168.20.10`

### Management
- ceos-1: `10.99.0.11`
- ceos-2: `10.99.0.12`

## 4. Key Concepts Demonstrated

### 1. VLAN Trunking

I verified VLAN trunking on both switches using:
```bash
show interfaces trunk
show vlan
```
### 2. STP Root Election
```bash
show spanning-tree
show spanning-tree vlan 10
show spanning-tree vlan 20
```

### 3. MAC Learning
```bash
show mac address-table
```
### 4. Host Connectivity
- host-a ↔ host-b (same VLAN)
- rust-toolkit ↔ host-a (inter-switch L2)
- rust-toolkit ↔ host-b

### 5. Rust Toolkit L2 Scanning
From rust-toolkit:
```bash
./arp_scan 192.168.10.0/24
./sniffer eth0
```
## Quick Start
```bash
docker compose up -d
docker exec -it ceos-1 Cli
docker exec -it ceos-2 Cli
```

## Troubleshooting

- **STP blocking**  
  Check:
`show spanning-tree detail`


- **VLAN mismatch**  
Ensure both switches allow VLANs 10 and 20 on the trunk.

- **MAC not learning**  
Check:
`show mac address-table dynamic`