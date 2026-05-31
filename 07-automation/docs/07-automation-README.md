# Lab 07 – Automation with Rust, eAPI, and BGP/EVPN State

## 1. Overview

In this lab, I demonstrated how to use a Rust application to:

- Connect to Arista cEOS devices over eAPI (HTTPS/JSON-RPC)
- Collect operational state (hostname, version, VLANs, interfaces)
- Prepare for configuration pushes and state validation across multiple devices


## 2. Topology

- leaf-1 (10.99.0.21)
- leaf-2 (10.99.0.22)
- rust-automation (10.99.0.50)

All connected via `mgmt-net`.

## 3. Quick Start

```bash
cd 07-automation
docker compose up -d
docker logs -f rust-automation
```
You should see JSON output for:

- `show hostname`

- `show version`

- `show vlan`

- `show ip interface brief`

## Next Steps (Extensions)
- Add commands:
  - show bgp evpn summary
  -  show vxlan

- Implement a “drift check”:

    - Compare VLAN lists between leaf-1 and leaf-2

- Implement a “config push”:

    - Create VLAN 20 on both leaves if missing