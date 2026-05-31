# Lab 10 – Enterprise Network (Capstone)

## 1. Bring-Up

Before starting the lab, validate the Compose file to catch undefined networks or missing references before any containers start:

```bash
docker compose config
```

To start the lab:

```bash
docker compose up -d
```

Full clean restart if containers are in a bad state:

```bash
docker compose down --remove-orphans
docker network prune -f
docker compose up -d
```

---

## 2. Configuration Guidelines

### WAN Edge Gateway
To ensure stability, the `wan-net` uses `203.0.113.254` as the bridge gateway. This prevents Docker from claiming `.1`, which is assigned to the `wan-edge` container.
Access layer nodes are assigned the `10.99.0.4x` range, while Leaf nodes occupy the `10.99.0.2x` range.
Note that cEOS containers manage their own init processes; do not include a `command:` override in the compose file.

### Config Files
Each cEOS node reads its startup config from `/mnt/flash/<node>.cfg` via the `EOS_CONFIG_FILE` environment variable. If a config file is missing from the mapped volume directory, the container will fail to start. Ensure the following config files exist before bring-up:

| Directory | Required Files |
|---|---|
| `./configs/core/` | `spine-1.cfg`, `spine-2.cfg`, `leaf-1.cfg` – `leaf-4.cfg` |
| `./configs/distribution/` | `dist-1.cfg`, `dist-2.cfg` |
| `./configs/access/` | `access-1.cfg` – `access-4.cfg` |
| `./configs/wan/` | `wan-edge.cfg`, `branch-1.cfg`, `branch-2.cfg` |

---

## 3. Troubleshooting
For a detailed log of issues encountered during the initial build (including IP conflicts and Docker Compose versioning), see the Troubleshooting Log.

## 4. Topology
![lab 10 topology](<../diagrams/lab 10 topology.png>)
