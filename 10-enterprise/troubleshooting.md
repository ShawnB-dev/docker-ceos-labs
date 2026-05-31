# Troubleshooting Log: Docker cEOS Enterprise Lab Bring-Up

**Environment:** Docker Compose, Arista cEOS 4.36.0.1F, Debian 12 (frr-fw), Ubuntu host  
**Lab:** 10-enterprise — multi-tier EVPN fabric with WAN edge, DMZ firewall, and branch nodes  
**Outcome:** All 27 containers brought up successfully

---

## Issue 1: `branch-1` Fails With "No Command Specified"

### Symptom
```
✘ Container branch-1   Error response from daemon: no command specified
```

### Cause
The cEOS image (`ceos:4.36.0.1F`) does not have a default `CMD` or `ENTRYPOINT` baked in. Docker requires either the image or the Compose service definition to specify a command. All other cEOS containers started because Docker used a cached layer from a previous run; `branch-1` hit the issue fresh.

A secondary cause was a missing config file: `./configs/wan/branch-1.cfg` did not exist on the host, which would have caused cEOS to fail even after the command issue was resolved.

### Fix
- Confirmed that cEOS containers should **not** use `command: /sbin/init` — the image uses its own init mechanism. The correct approach is to let the image entrypoint run unmodified (no `command:` field in Compose).
- Created the missing `branch-1.cfg` in `./configs/wan/`.

### Lesson
When a subset of identical-image containers start and others don't, suspect a missing config file or a Docker layer cache masking the real error on the containers that "work."

---

## Issue 2: Management Network IP Conflicts (`access-3`, `access-4`)

### Symptom
Silent — no immediate error at bring-up, but routing and management access would have been broken at runtime.

### Cause
`leaf-3` and `access-3` were both assigned `10.99.0.23` on `mgmt-net`. `leaf-4` and `access-4` were both assigned `10.99.0.24`. The access layer IPs were copy-pasted from the leaf layer without being incremented.

```yaml
# Conflicting assignments
leaf-3:    10.99.0.23   # correct
access-3:  10.99.0.23   # duplicate — should be 10.99.0.43

leaf-4:    10.99.0.24   # correct
access-4:  10.99.0.24   # duplicate — should be 10.99.0.44
```

### Fix
```yaml
access-3:
  networks:
    mgmt-net:
      ipv4_address: 10.99.0.43

access-4:
  networks:
    mgmt-net:
      ipv4_address: 10.99.0.44
```

### Lesson
Always audit IP assignments across all services in the same network before first bring-up, particularly when services are copy-pasted. A simple grep or table of IPs catches this instantly.

---

## Issue 3: `frr-fw` Stuck in `Created` State, Running `/sbin/init`

### Symptom
```
Container frr-fw   Created   (never transitions to Running)
```
```
docker inspect frr-fw | grep Cmd
"Cmd": ["/sbin/init"]
```

### Cause
During earlier troubleshooting a `command: /sbin/init` line was added to `frr-fw` in `docker-compose.yml`. A Compose `command:` field overrides the Dockerfile `CMD`. The Debian 12 base image does not have `/sbin/init`, so the container could not start. Even after rebuilding with `--no-cache`, the override persisted because it was in the Compose file, not the Dockerfile.

### Fix
Removed the `command:` line from the `frr-fw` service definition in `docker-compose.yml`. The Dockerfile's `CMD ["/entrypoint.sh"]` then ran as intended.

```yaml
# Before (broken)
frr-fw:
  build:
    context: ./firewall
    dockerfile: Dockerfile
  command: /sbin/init   # ← override killing the container

# After (correct)
frr-fw:
  build:
    context: ./firewall
    dockerfile: Dockerfile
  # no command: field — Dockerfile CMD runs
```

### Lesson
`docker inspect <container> | grep Cmd` is the fastest way to confirm what command a container is actually running vs what you expect. A `--no-cache` rebuild will not fix a Compose-level `command:` override — always check both the Dockerfile and the Compose file.

---

## Issue 4: `wan-edge` Fails With "Address Already in Use"

### Symptom
```
Error response from daemon: failed to set up container networking: Address already in use
```

### Cause
Docker's bridge driver automatically assigns the first usable address in a subnet as the bridge gateway. For `wan-net` (`203.0.113.0/24`), Docker claimed `203.0.113.1` as the gateway. `wan-edge` was also assigned `203.0.113.1`, creating a conflict before the container could start.

This was confirmed by the absence of any `203.0.113.x` address on the host (`ip addr show | grep 203.0.113` returned nothing), ruling out a host-level conflict.

### Fix
Explicitly set the gateway to a non-conflicting address in the Compose network definition:

```yaml
wan-net:
  driver: bridge
  ipam:
    config:
      - subnet: 203.0.113.0/24
        gateway: 203.0.113.254
```

This pushed the bridge gateway to the top of the address space, freeing `203.0.113.1` for `wan-edge`.

### Lesson
When assigning `.1` addresses to containers on any network, always explicitly set `gateway:` in the Compose IPAM config. Docker claims `.1` by default and will silently conflict with your container.

---

## Issue 5: `dmz-net` Undefined Network Error

### Symptom
```
service "frr-fw" refers to undefined network dmz-net: invalid compose project
```

### Cause
The `dmz-net` network definition was accidentally deleted from the `networks:` section during editing.

### Fix
Restored the missing network definition:

```yaml
dmz-net:
  driver: bridge
  ipam:
    config:
      - subnet: 192.168.254.0/24
```

### Lesson
After any bulk editing of `docker-compose.yml`, run `docker compose config` to validate the file before attempting bring-up. It catches undefined network and volume references immediately without starting any containers.

---

## Issue 6: `dmz-srv1` / `dmz-srv2` Network Not Found

### Symptom
```
Error response from daemon: failed to set up container networking: network 38ced1630f84... not found
```

### Cause
A race condition or orphaned network state from previous failed bring-up attempts caused the `dmz-srv-net` and `dmz-srv-net2` networks to be removed mid-startup before the containers could attach.

### Fix
```bash
docker compose down --remove-orphans
docker network prune -f
docker compose up -d
```

### Lesson
Hash-based network IDs in error messages indicate Docker is referencing a network object that no longer exists in its state. `--remove-orphans` and `network prune` are the standard cleanup tools for this class of error.

---

## Known Remaining Issues (Not Blocking Startup)

### DMZ Server Default Gateways Unreachable
`dmz-srv1` and `dmz-srv2` configure default routes via gateways that are not reachable from their respective networks:

- `dmz-srv1` routes via `172.16.100.1` (on `lan-net`) but is only attached to `dmz-srv-net` (`172.16.102.0/24`)
- `dmz-srv2` routes via `172.16.101.1` but its gateway is not attached to `dmz-srv-net2`

`frr-fw` is not attached to `dmz-srv-net` or `dmz-srv-net2`, so there is currently no reachable gateway for either server. Traffic will fail at runtime. Resolution requires either attaching `frr-fw` to both DMZ server networks, or correcting the gateway IPs in the container startup commands.

### Version: Deprecation Warning
The `version: "3.9"` attribute at the top of `docker-compose.yml` is obsolete in current Docker Compose and generates a warning on every command. It can be safely removed.

---

## Quick Reference: Diagnostic Commands

| Goal | Command |
|---|---|
| Check what command a container is actually running | `docker inspect <name> \| grep -A 5 "Cmd\|Entrypoint"` |
| See why a container failed | `docker logs <name>` |
| Check container state including exited | `docker ps -a \| grep <name>` |
| Inspect network IP assignments | `docker network inspect <network> --format '{{json .Containers}}' \| python3 -m json.tool` |
| Inspect network gateway | `docker network inspect <network> --format '{{json .IPAM}}'` |
| Validate compose file without starting | `docker compose config` |
| Full clean restart | `docker compose down --remove-orphans && docker network prune -f && docker compose up -d` |
| Force image rebuild | `docker compose build --no-cache <service>` |
