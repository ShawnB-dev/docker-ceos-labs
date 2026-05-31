# Lab 06 – Rust ツールキットの L2 / L3 / EVPN 統合

## 1. 概要

このラボでは、私の自作 Rust ネットワークツールが以下の環境でどのように動作するかを検証しました。

- L2 スイッチング（VLAN / STP）
- L3 ルーティング（OSPF / BGP）
- EVPN-VXLAN ファブリック（Type-2 / Type-3 ルート）

## 2. シナリオ

### 1. L2 動作
- ARP スキャン
- MAC 学習
- ブロードキャスト解析

### 2. L3 ルーティング
- トレースルート
- ポートスキャン
- ICMP スイープ

### 3. EVPN-VXLAN
- ARP 抑制
- EVPN MAC/IP ルート
- VXLAN カプセル化の観察

## 3. 実証した Rust ツール

- `arp_scan`
- `pingsweep`
- `portscan`
- `sniffer`
- `traceroute`

## 4. クイックスタート

```bash
docker compose up -d
docker logs -f rust-toolkit
```

各シナリオは scenarios/ ディレクトリにあります。