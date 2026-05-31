#!/bin/bash

# Enable IP forwarding
sysctl -w net.ipv4.ip_forward=1

# Start FRR and nftables
service frr start
nft -f /etc/nftables.conf

exec sleep infinity