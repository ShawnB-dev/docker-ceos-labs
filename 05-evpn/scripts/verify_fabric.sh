#!/bin/bash

# Lab 05 Verification Script - EVPN-VXLAN Fabric Status

echo "=========================================================="
echo "🚀 Starting Lab 05 Fabric Verification"
echo "=========================================================="

echo -e "\n--- 1. Underlay Check: OSPF Neighbors ---"
for node in spine-1 spine-2 leaf-1 leaf-2; do
    echo -n "[$node] "
    docker exec $node FastCli -p 15 -c "show ip ospf neighbor" | grep -c "Full" | xargs echo "Neighbors in Full state:"
done

echo -e "\n--- 2. Overlay Check: BGP EVPN Summary ---"
for node in spine-1 spine-2 leaf-1 leaf-2; do
    echo "[$node]"
    docker exec $node FastCli -p 15 -c "show bgp evpn summary" | grep -A 5 "Neighbor"
done

echo -e "\n--- 3. VXLAN Check: VTEPs and VNIs ---"
for node in leaf-1 leaf-2; do
    echo "[$node]"
    docker exec $node FastCli -p 15 -c "show vxlan vni"
done

echo -e "\n--- 4. MAC Learning (EVPN) ---"
for node in leaf-1 leaf-2; do
    echo "[$node] Dynamic MAC Table:"
    docker exec $node FastCli -p 15 -c "show mac address-table dynamic"
done

echo -e "\n--- 5. End-to-End Connectivity Check ---"
echo "[host-a -> host-b (192.168.10.11)]"
docker exec host-a ping -c 3 192.168.10.11 | grep "packet loss"

echo -e "\n[rust-toolkit -> host-a (192.168.10.10)]"
docker exec rust-toolkit ping -c 3 192.168.10.10 | grep "packet loss"

echo -e "\n[rust-toolkit -> host-b (192.168.10.11)]"
docker exec rust-toolkit ping -c 3 192.168.10.11 | grep "packet loss"

echo -e "\n=========================================================="
echo "✅ Verification Complete"
echo "=========================================================="