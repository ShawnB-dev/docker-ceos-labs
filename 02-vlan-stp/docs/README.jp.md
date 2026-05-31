# ラボ 02 – VLAN + STP (マルチスイッチ L2 ネットワーク)

## 1. 概要

このラボでは、ラボ 01 を拡張し、VLAN、トランキング、Rapid-PVST スパニングツリーを使用したマルチスイッチ レイヤ 2 ネットワークを導入しました。私は 2 台の cEOS スイッチを構成して小さな L2 ドメインを形成し、`ceos-1` を STP ルートブリッジとして機能させました。MAC 学習、STP の動作、およびスイッチ間転送を実証するために、ホストをスイッチ全体に分散させました。

## 2. トポロジ

- **ceos-1**: 私は `ceos-1` を STP ルートブリッジとして構成し、VLAN 10 および VLAN 20 の SVI を提供しました。
- **ceos-2**: 私は `ceos-2` をトランクアップリンクを持つアクセススイッチとして設定しました。
- **host-a**: 私は `host-a` を VLAN 10 に配置しました。
- **host-b**: 私は `host-b` を VLAN 10 に配置しました。
- **rust-toolkit**: 私は `rust-toolkit` を VLAN 20 に配置しました。
  

```text
                   mgmt-net (10.99.0.0/24)
                           |
                +-----------------------+
                |       ceos-1          |
                |  STP Root Bridge      |
                |  Vlan10: 192.168.10.1 |
                |  Vlan20: 192.168.20.1 |
                +----------+------------+
                           |
                           | Trunk (VLANs 10,20)
                           |
                +----------+------------+
                |        ceos-2         |
                |  Access + Trunk       |
                +-----+-----------+-----+
                      |           |
                      |           |
            Access (10)|           | Access (20)
                      |           |
                +-----------+   +-----------------+
                |  host-a   |   |  rust-toolkit   |
                |10.10/24   |   |20.10/24         |
                +-----------+   +-----------------+

                +-----------+
                |  host-b   |
                |10.11/24   |
                +-----------+
```

(diagrams/topology.png を参照)

## 3. アドレス設計

### VLAN 10 – USERS
- サブネット: `192.168.10.0/24`
- ゲートウェイ: `192.168.10.1` (ceos-1)
- host-a: `192.168.10.10`
- host-b: `192.168.10.11`

### VLAN 20 – TOOLS
- サブネット: `192.168.20.0/24`
- ゲートウェイ: `192.168.20.1` (ceos-1)
- rust-toolkit: `192.168.20.10`

### 管理
- ceos-1: `10.99.0.11`
- ceos-2: `10.99.0.12`

## 4. 実証した主要な概念

### 1. VLAN トランキング

私は両方のスイッチで VLAN トランキングを以下のコマンドで確認しました:
```bash
show interfaces trunk
show vlan
```
### 2. STP ルート選出
私は以下のコマンドを使用して STP ルート選出を観察しました:
```bash
show spanning-tree
show spanning-tree vlan 10
show spanning-tree vlan 20
```

### 3. MAC 学習
私は MAC 学習を以下のコマンドで調べました:
```bash
show mac address-table
```
### 4. ホスト接続性
私はホスト接続性をテストしました:
- `host-a` ↔ `host-b` (同じ VLAN 内)
- `rust-toolkit` ↔ `host-a` (スイッチ間 L2)
- `rust-toolkit` ↔ `host-b`

### 5. Rust ツールキット L2 スキャン
`rust-toolkit` コンテナから、私は L2 スキャンを実行しました:
```bash
./arp_scan 192.168.10.0/24
./sniffer eth0
```
## 5. クイック スタート
私は以下のコマンドを使用してラボを開始しました:
```bash
docker compose up -d
docker exec -it ceos-1 Cli
docker exec -it ceos-2 Cli
```

## 6. トラブルシューティング

- **STP ブロッキング**  
  私は STP ブロッキングを以下のコマンドで確認しました:
`show spanning-tree detail`


- **VLAN ミスマッチ**  
私は両方のスイッチでトランク上で VLAN 10 と 20 が許可されていることを確認しました。

- **MAC が学習されない**  
私は MAC 学習を以下のコマンドで確認しました:
`show mac address-table dynamic`