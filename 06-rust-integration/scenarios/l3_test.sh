#!/bin/bash
# Test L3 reachability using traceroute and portscan

echo "Tracing path to host-b loopback via routed fabric..."
/tools/traceroute 10.255.1.2

echo "Scanning common ports on leaf-1 eAPI..."
/tools/portscan 10.99.0.21 443