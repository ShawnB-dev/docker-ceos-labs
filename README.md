# Arista cEOS Network Labs with Rust Integration
# Arista cEOS ネットワークラボ (Rust 自動化・テレメトリ統合)

## 1. Project Overview / プロジェクト概要

**English:**
This repository contains a comprehensive series of networking labs built using **Arista cEOS** and **Docker**. Over the course of 10 labs, I have evolved this project from basic Layer 2 switching to a sophisticated, multi-tier Enterprise network. A key feature of this project is the integration of a custom **Rust-based toolkit** used for active network scanning, automated configuration management via eAPI, and real-time telemetry collection.

**日本語:**
このリポジトリには、**Arista cEOS** と **Docker** を使用して構築された包括的なネットワークラボシリーズが含まれています。全10回のラボを通じて、基本的なレイヤ 2 スイッチングから洗練されたマルチティア・エンタープライズネットワークへとプロジェクトを発展させてきました。このプロジェクトの大きな特徴は、アクティブネットワークスキャン、eAPI による自動構成管理、およびリアルタイムテレメトリ収集に使用される、自作の **Rust ベースのツールキット**を統合している点です。

---

## 2. Lab Index / ラボ一覧

### [01. cEOS Basics](./01-ceos-basics/README.md) ([JA](./01-ceos-basics/docs/README.jp.md)) / cEOS 基本設定
- **EN:** Basic L2/L3 setup with VLANs, SVIs, and Rust-based scanning tools.
- **JP:** VLAN、SVI、および Rust ベースのスキャンツールを使用した基本的な L2/L3 設定。

### [02. VLAN & STP](./02-vlan-stp/README.md) ([JA](./02-vlan-stp/docs/README.jp.md)) / VLAN とスパニングツリー
- **EN:** Multi-switch L2 environment implementing Rapid-PVST and trunking.
- **JP:** Rapid-PVST とトランキングを実装した複数スイッチによる L2 環境。

### [03. Routing (OSPF/BGP)](./03-routing-ospf-bgp/README.md) ([JA](./03-routing-ospf-bgp/docs/README.jp.md)) / ルーティング (OSPF/BGP)
- **EN:** Routed fabric with OSPF underlay, iBGP overlay, and an eBGP edge.
- **JP:** OSPF アンダーレイ、iBGP オーバーレイ、および eBGP エッジを備えたルーテッドファブリック。

### [04. Spine-Leaf Fabric](./04-spine-leaf/docs/README.md) ([JA](./04-spine-leaf/docs/README.jp.md)) / スパイン・リーフ ファブリック
- **EN:** 2-Spine x 4-Leaf L3 underlay utilizing OSPF and ECMP for path redundancy.
- **JP:** パス冗長性のために OSPF と ECMP を利用した 2 スパイン x 4 リーフの L3 アンダーレイ。

### [05. EVPN-VXLAN](./05-evpn/README.md) ([JA](./05-evpn/docs/README.jp.md)) / EVPN-VXLAN ファブリック
- **EN:** Full EVPN-VXLAN deployment with anycast gateways and L2 stretch validation.
- **JP:** エニキャストゲートウェイと L2 ストレッチ検証を備えたフル EVPN-VXLAN 展開。

### [06. Rust Integration](./06-rust-integration/README.md) ([JA](./06-rust-integration/docs/README.jp.md)) / Rust ツールの統合
- **EN:** Validating the Rust toolkit’s behavior across L2, L3, and EVPN-VXLAN scenarios.
- **JP:** L2、L3、および EVPN-VXLAN の各シナリオにおける Rust ツールキットの動作検証。

### [07. Automation (eAPI)](./07-automation/docs/07-automation-README.md) ([JA](./07-automation/docs/README.jp.md)) / ネットワーク自動化 (eAPI)
- **EN:** Programmatic device state collection using Rust and Arista eAPI (JSON-RPC).
- **JP:** Rust と Arista eAPI (JSON-RPC) を使用したプログラムによるデバイス状態の収集。

### [08. Telemetry](./08-telemetry/docs/08-telemetry-README.md) ([JA](./08-telemetry/docs/README.jp.md)) / テレメトリ
- **EN:** Real-time observability pipeline for interface counters and routing protocol states.
- **JP:** インターフェースカウンタとルーティングプロトコル状態のリアルタイム可観測性パイプライン。

### [09. Security](./09-security/README.md) ([JA](./09-security/docs/README.jp.md)) / セキュリティ
- **EN:** Management plane hardening with AAA, ACLs, CoPP, and SSH security policies.
- **JP:** AAA、ACL、CoPP、および SSH セキュリティポリシーによる管理プレーンの要塞化。

### [10. Enterprise Capstone](./10-enterprise/docs/10-enterprise-README.md) ([JA](./10-enterprise/docs/README.jp.md)) / エンタープライズ統合 (最終課題)
- **EN:** The final integration: EVPN core, WAN edge, DMZ firewall, and branch connectivity.
- **JP:** 最終統合：EVPN コア、WAN エッジ、DMZ ファイアウォール、およびブランチ接続。

---

## 3. Prerequisites / 前提条件

**English:**
- **Docker & Docker Compose:** Required to orchestrate the containers.
- **Arista cEOS Image:** `ceos:4.36.0.1F` (must be imported locally).
- **Rust:** Required if you wish to build/modify the automation toolkit.

**日本語:**
- **Docker & Docker Compose:** コンテナのオーケストレーションに必要。
- **Arista cEOS イメージ:** `ceos:4.36.0.1F` (ローカルへのインポートが必要)。
- **Rust:** 自動化ツールキットのビルドや変更を行う場合に必要。

---

## 4. Getting Started / はじめに

**English:**
1. Clone the repository.
2. Navigate to a specific lab directory (e.g., `cd 04-spine-leaf`).
3. Start the lab: `docker compose up -d`.
4. Access nodes via: `docker exec -it <node_name> FastCli -p 15`.

**日本語:**
1. リポジトリをクローンします。
2. 特定のラボディレクトリに移動します (例: `cd 04-spine-leaf`)。
3. ラボを開始します: `docker compose up -d`。
4. 次のコマンドでノードにアクセスします: `docker exec -it <ノード名> FastCli -p 15`。

---

## 5. Summary / まとめ

**English:**
By following these labs, I have built a deep understanding of modern network architectures. I have successfully combined traditional routing protocols (OSPF, BGP) with advanced Data Center technologies (EVPN-VXLAN) and integrated them with software development practices (Rust, JSON-RPC automation).

**日本語:**
これらのラボを通じて、現代のネットワークアーキテクチャに対する深い理解を築きました。従来のルーティングプロトコル (OSPF, BGP) と高度なデータセンター技術 (EVPN-VXLAN) を組み合わせ、さらにソフトウェア開発手法 (Rust, JSON-RPC 自動化) との統合に成功しました。

---
**Author:** Shawn
**License:** MIT