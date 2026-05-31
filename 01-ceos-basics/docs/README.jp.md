# ラボ 01 – 1台の cEOS スイッチ、2台のホスト、および Rust ツールキット

## 1. 概要

このラボでは、Docker と Arista cEOS を使用して、小規模ながら現実的な Layer 2/Layer 3 環境を構築します。
1台の cEOS スイッチが、ユーザー VLAN とツール用 VLAN 間の VLAN 間ルーティングを提供します。
2台の Linux ホストはエンドユーザーを表し、Rust ツールキットコンテナはアクティブスキャンやトラフィック分析に使用されます。

## 2. 使用技術

- Docker / Docker Compose
- Arista cEOS 4.36.0.1F
- 基本的な VLAN と SVI
- VLAN 間ルーティング
- Rust ベースのネットワークツール（ポートスキャナー、ピングスイーパー、ARP スキャナー、スニッファー）

## 3. トポロジー

```text
                 管理ネットワーク (10.99.0.0/24)
                         |
                  +--------------+
                  |  ceos-1      |
                  | Mgmt1: 10.99.0.11
                  | Vlan10: 192.168.10.1/24
                  | Vlan20: 192.168.20.1/24
                  +---+-----+----+
                      |     |
          Eth1        |     |        Eth3
        (VLAN 10)     |     |      (VLAN 20)
                      |     |
        --------------+-----+--- データネットワーク (172.18.0.0/24)
                      |     |
                      |     |
                +-----------+-----------+
                |           |           |
          +-----------+ +-----------+ +-----------------+
          |  host-a   | |  host-b   | |  rust-toolkit   |
          | eth0      | | eth0      | | eth0            |
          | 192.168.  | | 192.168.  | | 192.168.20.10   |
          | 10.10/24  | | 10.11/24  | | (VLAN 20 GW経由)|
          +-----------+ +-----------+ +-----------------+
```

## 4. アドレス設計

#### 管理ネットワーク (mgmt-net)
* 10.99.0.0/24
* ceos-1 Management1: 10.99.0.11/24

#### ユーザー VLAN (VLAN 10 – USERS)
* 192.168.10.0/24
* ceos-1 Vlan10: 192.168.10.1/24 (デフォルトゲートウェイ)
* host-a: 192.168.10.10/24
* host-b: 192.168.10.11/24

#### ツール用 VLAN (VLAN 20 – TOOLS)
* 192.168.20.0/24
* ceos-1 Vlan20: 192.168.20.1/24 (デフォルトゲートウェイ)
* rust-toolkit: 192.168.20.10/24 (VLAN 20 経由)

## 5. 前提条件
* Docker および Docker Compose がインストールされていること
* Arista `cEOS 4.36.0.1F` イメージが利用可能であること（`ceos:4.36.0.1F` としてインポート済みであること）
* このラボの相対パス `../rust-toolkit` に Rust ネットワークツールキットのリポジトリがマウントされていること

## 6. クイックスタート

```bash
git clone <your-repo-url> docker-ceos-labs
cd docker-ceos-labs/01-ceos-basics
```

### ラボの開始
```bash
docker compose up -d
```

### cEOS CLI へのアクセス
```bash
docker exec -it ceos-1 Cli
```

### ホストへのアクセス
```bash
docker exec -it host-a sh
docker exec -it host-b sh
```

### Rust ツールキットコンテナへのアクセス
```bash
docker exec -it rust-toolkit bash
```

## 7. 設定の詳細
* **ceos-1**: Ethernet1–3 で Layer 2 スイッチングを提供します。
* Ethernet1 および Ethernet2 は **VLAN 10 (USERS)**。
* Ethernet3 は **VLAN 20 (TOOLS)**。
* Vlan10 および Vlan20 の SVI がデフォルトゲートウェイを提供します。
* VLAN 間ルーティングのために `ip routing` を有効化しています。
* 詳細な設定は `configs/ceos-1.cfg` にあります。

## 8. ホスト
host-a および host-b は、起動時に静的 IP が設定される Alpine コンテナです。
* host-a: `192.168.10.10/24`, デフォルトゲートウェイ `192.168.10.1`
* host-b: `192.168.10.11/24`, デフォルトゲートウェイ `192.168.10.1`

## 9. Rust ツールキットコンテナ
rust-toolkit コンテナは、Rust ツールを `/tools` にマウントし、同じデータネットワークに参加します。
`192.168.10.0/24` および `192.168.20.0/24` へのトラフィックは cEOS 経由でルーティングされます。

使用例 (rust-toolkit 内):
```bash
cd /tools
# ユーザー VLAN のピングスイープ
./pingsweep 192.168.10.0/24
# host-a のポートスキャン
./portscan 192.168.10.10 1-1024
# ユーザー VLAN の ARP スキャン
./arp_scan 192.168.10.0/24
# eth0 でパケットスニッファーを実行
./sniffer eth0
```

## 10. 動作確認
### host-a から:
```bash
ping 192.168.10.11      # host-b
ping 192.168.20.10      # rust-toolkit (VLAN 間ルーティング経由)
ping 192.168.10.1       # デフォルトゲートウェイ
```

### ceos-1 から:
```text
show vlan
show ip interface brief
show ip route
show interfaces status
```

## 11. 管理と安定性
このラボでは、以下のベストプラクティスを適用しています。
- **リソース制限**: ホストマシンのリソース枯渇を防ぐため、各コンテナに CPU とメモリの制限を設定しています。
- **ヘルスチェック**: 検証を開始する前にスイッチの管理プレーンが完全に初期化されていることを確認するため、cEOS ノードにヘルスチェックを含めています。

## 12. クリーンアップ
```bash
docker compose down
```

## 13. トラブルシューティング
* **cEOS が起動しない**: ローカルに `ceos:4.36.0.1F` イメージが存在するか確認してください。
* **ホスト間の疎通がない**: cEOS の VLAN メンバーシップ、SVI、および `ip routing` が有効であることを確認してください。
* **Rust ツールがホストに到達できない**: rust-toolkit が SVI (`192.168.20.1`) に ping できるか確認してください。