# Lab 01 – Single cEOS switch with two hosts and Rust toolkit

## Overview

This lab builds a small but realistic Layer 2/Layer 3 environment using Docker and Arista cEOS.
A single cEOS switch provides inter-VLAN routing between a user VLAN and a tools VLAN.
Two Linux hosts represent end users, and a Rust toolkit container is used for active scanning
and traffic analysis.

## Technologies used:

- Docker / Docker Compose
- Arista cEOS 4.36.0.1F
- Basic VLANs and SVIs
- Inter-VLAN routing
- Rust-based networking tools (port scanner, ping sweeper, ARP scanner, sniffer)

## Topology

```text
                 mgmt-net (10.99.0.0/24)
                         |
                  +--------------+
                  |  ceos-1      |
                  | Mgmt1:10.0.0.11
                  | Vlan10:192.168.10.1/24
                  | Vlan20:192.168.20.1/24
                  +---+-----+----+
                      |     |
          Eth1        |     |        Eth3
        (VLAN 10)     |     |      (VLAN 20)
                      |     |
        --------------+-----+--- data-net (172.18.0.0/24)
                      |     |
                      |     |
                +-----------+-----------+
                |           |           |
          +-----------+ +-----------+ +-----------------+
          |  host-a   | |  host-b   | |  rust-toolkit   |
          | eth0      | | eth0      | | eth0            |
          | 192.168.  | | 192.168.  | | 192.168.20.10   |
          | 10.10/24  | | 10.11/24  | | (via VLAN 20 GW)|
          +-----------+ +-----------+ +-----------------+

```
## Addressing
#### Management network (mgmt-net)

* 10.99.0.0/24

* ceos-1 Management1: 10.99.0.11/24

#### User VLAN (VLAN 10 – USERS)

* 192.168.10.0/24

* ceos-1 Vlan10: 192.168.10.1/24 (default gateway)

* host-a: 192.168.10.10/24

* host-b: 192.168.10.11/24

#### Tools VLAN (VLAN 20 – TOOLS)
* 192.168.20.0/24

* ceos-1 Vlan20: 192.168.20.1/24 (default gateway)

* rust-toolkit: 192.168.20.10/24 (via VLAN 20)

## Prerequisites
* Docker and Docker Compose installed

* Access to an Arista `cEOS 4.36.0.1F` image (properly imported as `ceos: 4.36.0.1F`)

* A Rust networking toolkit repository mounted at 
`../rust-toolkit` 
relative to this lab

## Quick start
```bash
git clone <your-repo-url> docker-ceos-labs
cd docker-ceos-labs/01-ceos-basics
```

## Start the lab
```bash
docker compose up -d
```
## Access cEOS CLI
```bash
docker exec -it ceos-1 Cli
```
## Access hosts
```bash
docker exec -it host-a sh
docker exec -it host-b sh
```
## Access Rust toolkit container
```bash
docker exec -it rust-toolkit bash
```

## Configuration details
* cEOS-1
Provides Layer 2 switching on Ethernet1–3

* VLAN 10 (USERS) on Ethernet1 and Ethernet2

* VLAN 20 (TOOLS) on Ethernet3

* SVIs on Vlan10 and Vlan20 provide default gateways

* I enabled `ip routing` for inter-VLAN routing.

The full configuration is in configs/ceos-1.cfg.

## Hosts
host-a and host-b are Alpine containers with static IPs configured at startup:

* host-a: `192.168.10.10/24`, default gateway `192.168.10.1`

* host-b: `192.168.10.11/24`, default gateway `192.168.10.1`


## Rust toolkit container
The rust-toolkit container mounts your Rust tools into /tools and joins the same data network.

Traffic to `192.168.10.0/24` and `192.168.20.0/24` is routed via cEOS.

Example usage (inside rust-toolkit):

```bash
cd /tools
```
# Ping sweep user VLAN

```bash
./pingsweep 192.168.10.0/24
```

### Port scan host-a
```bash
./portscan 192.168.10.10 1-1024
```
### ARP scan user VLAN
```bash
./arp_scan 192.168.10.0/24
```
### Run packet sniffer on eth0
```bash
./sniffer eth0
```

## Verification

### From host-a:

```bash
ping 192.168.10.11      # host-b
ping 192.168.20.10      # rust-toolkit (via inter-VLAN routing)
ping 192.168.10.1       # default gateway
```
### From host-b:

```bash
ping 192.168.10.10
ping 192.168.20.10
```
### From ceos-1:

```text
show vlan
show ip interface brief
show ip route
show interfaces status
ping 192.168.10.10
ping 192.168.20.10
```
### From rust-toolkit:

```bash
ping 192.168.10.10
ping 192.168.10.11
```

## 10. Housekeeping & Stability
I have optimized this lab with several best practices:
- **Resource Limits**: Each container is constrained to specific CPU and Memory limits to prevent host starvation.
- **Health Checks**: The cEOS node includes a health check to ensure the switch management plane is fully initialized before attempting verification.

## 11. Cleanup
```bash
docker compose down
```

## 12. Troubleshooting

### cEOS does not start

* Check that the image ceos:4.36.0.1F exists locally:

```bash
docker images | grep ceos
No connectivity between hosts
```
### Verify VLAN membership and SVIs on cEOS:

```text
show vlan
show ip interface brief
```
### Check that ip routing is enabled.

***Rust tools cannot reach hosts***

Confirm rust-toolkit can ping the SVIs:

```bash
ping 192.168.20.1
ping 192.168.10.1
```