#!/bin/bash
# Test L2 connectivity over EVPN using the Rust pingsweep tool

echo "Running pingsweep on VLAN 10 USERS segment..."
/tools/pingsweep 192.168.10.0/24

echo "Running ARP scan to detect VTEP-learned MACs..."
/tools/arp_scan 192.168.10.0/24