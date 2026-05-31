# Lab 05 – EVPN-VXLAN Spine/Leaf Fabric

## 1. Overview

In this lab, I built a small but realistic EVPN-VXLAN fabric:

- 2× spines as EVPN route reflectors
- 2× leaves as VTEPs with VLAN/VNI mapping
- Anycast gateway for user and tools VLANs
- Hosts attached to different leaves to demonstrate L2 stretch

## 2. Topology

- Spines: spine-1, spine-2 (AS 65000)
- Leaves: leaf-1 (AS 65101), leaf-2 (AS 65102)
- host-a: VLAN 10 on leaf-1
- host-b: VLAN 10 on leaf-2
- rust-toolkit: VLAN 20 on leaf-1

## 3. Key Technologies

- OSPF underlay (Area 0)
- BGP EVPN overlay
- VXLAN data plane
- Anycast gateway for VLAN 10 and 20

## 4. Quick Start

```bash
cd 05-evpn
docker compose up -d

docker exec -it spine-1 Cli
docker exec -it spine-2 Cli
docker exec -it leaf-1 Cli
docker exec -it leaf-2 Cli
```

## 5. Verification
### Underlay
```bash
show ip ospf neighbor
show ip route
```
### EVPN / VXLAN
```bash
show bgp evpn summary
show bgp evpn route-type mac-ip
show vxlan
show vxlan vni
show mac address-table dynamic
```

### End-to-end
From host-a:
```bash
ping 192.168.10.11 (host-b)
```
From rust-toolkit:
```bash
ping 192.168.10.10

ping 192.168.10.11
```
I also ran the Rust ARP and sniffer tools on the VLAN 10 and 20 segments.

## 6. Housekeeping & Stability
I have optimized this lab with several best practices:
- **Resource Limits**: Each cEOS container is constrained to specific CPU and Memory limits to prevent host starvation.
- **Health Checks**: Every node includes a health check to ensure the switch management plane is fully initialized before attempting verification.

## 7. Cleanup
```bash
docker compose down
```

## 8. Summary
- Understanding of EVPN control plane (Type-2/Type-3 routes)
- VXLAN VNI and VLAN mapping
- Anycast gateway across multiple leaves
- Integration of custom tooling (Rust) into an EVPN fabric