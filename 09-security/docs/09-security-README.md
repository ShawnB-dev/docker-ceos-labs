# Lab 09 – Security (AAA, ACLs, CoPP, and Management-Plane Hardening)

In this lab, I implemented foundational security controls across my cEOS management network.  
My goal was to demonstrate how I secure the management plane, restrict access, enforce AAA, and apply control-plane protections in a modern data‑center environment.

This lab builds on the earlier automation and telemetry work (Labs 07–08) and shows how I apply real-world security best practices to the same fabric.

---

## 1. Overview

In this lab, I implemented:

- AAA (local authentication + role separation)
- Management-plane ACLs (mgmt-acl)
- Infrastructure ACLs (iACL)
- SSH hardening (ciphers, key exchange, idle timeout)
- eAPI access restriction
- Control-plane policing (CoPP)
- Syslog forwarding
- Login banners and compliance text

These controls form the baseline security posture for any production network.

---

## 2. Topology

This lab uses the standard management topology used across the automation series:

mgmt-net (10.99.0.0/24)
┌──────────┬──────────┬──────────┬──────────┐
│          │          │          │          │
10.99.0.11 10.99.0.12 10.99.0.13 10.99.0.14  10.99.0.50
ceos-1     ceos-2     ceos-3     ceos-4    rust-host
│          │          │          │          │
└──────────┴──────────┴──────────┴──────────┴──────────┘

Code

- **ceos-1 → ceos-4**: Devices receiving the security baseline  
- **rust-host (10.99.0.50)**: The only permitted management client (automation + telemetry)

---

## 3. Security Controls Implemented

### AAA + RBAC
I created two roles:

- **admin** (privilege 15)
- **ops** (privilege 5)

AAA is configured for local authentication and authorization.

### Management ACL (mgmt-acl)
Only the automation/telemetry host (10.0.0.50) is allowed to access:

- SSH
- eAPI (HTTPS/JSON-RPC)

All other sources are denied and logged.

### Infrastructure ACL (iACL)
Protects the management interface from unwanted traffic:

- Allows ICMP
- Allows established TCP sessions
- Denies traffic sourced from the mgmt subnet (anti-spoofing)
- Permits everything else

### SSH Hardening
I enforced:

- Strong ciphers
- Strong key exchange
- Idle timeout
- Password authentication only (lab environment)

### eAPI Hardening
eAPI is restricted by mgmt-acl and only enabled in the default VRF.

### CoPP (Control Plane Policing)
I applied a simple three-class CoPP policy:

- Critical: 2000 pps
- Important: 1000 pps
- Normal: 500 pps

### Logging
All devices forward logs to 10.0.0.50.

---

## 4. Running the Lab

Start the environment:

```bash
docker compose up -d
Verify devices are reachable:

bash
ssh admin@10.0.0.11
Attempt access from an unauthorized host to confirm ACL enforcement.

5. Validation Checklist
I validated the security posture using the following checks:

AAA
Code
show aaa authentication
show aaa authorization
show users
SSH Hardening
Code
show management ssh
Management ACL
Code
show ip access-lists mgmt-acl
show management api http-commands
Infrastructure ACL
Code
show ip access-lists iacl
show interfaces Management1
CoPP
Code
show policy-map copp-system
Logging
Code
show logging
Negative Testing
Attempt SSH from a non-allowed IP → denied

Attempt eAPI access from unauthorized host → denied

Confirm logs show ACL hits

6. Cleanup
bash
docker compose down
7. Summary
This lab demonstrates how I secure the management plane of a network fabric using:

AAA

ACLs

CoPP

Logging

SSH/eAPI hardening

It completes the “core labs packet” by adding a realistic security baseline to the environment I built in Labs 01–08.