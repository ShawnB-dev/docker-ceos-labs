# Lab 07 – Rust + eAPI によるネットワーク自動化

## 1. 概要

このラボでは、Rust アプリケーションを用いて Arista cEOS に対して以下の操作を行う方法を実証しました：

- eAPI（HTTPS / JSON-RPC）で接続
- ホスト名 / バージョン / VLAN / インターフェース情報を取得
- 将来的なコンフィグ投入・状態検証の基盤を作成


## 2. トポロジー

- leaf-1 (10.99.0.21)
- leaf-2 (10.99.0.22)
- rust-automation (10.99.0.50)

すべて `mgmt-net` 上で接続されています。

## 3. クイックスタート

```bash
cd 07-automation
docker compose up -d
docker logs -f rust-automation
```

以下のコマンド結果が JSON 形式で表示されます。

- `show hostname`

- `show version`

- `show vlan`

- `show ip interface brief`

## 拡張アイデア
- 取得コマンドを追加:

  - show bgp evpn summary

  - show vxlan

- ドリフト検知:

   - leaf-1 / leaf-2 の VLAN 一覧を比較

- コンフィグ投入:

  - VLAN 20 が存在しない場合に自動作成