# ラボ 05 – EVPN-VXLAN スパイン・リーフ ファブリック

## 1. 概要

このラボでは、小規模ながら現実的な EVPN-VXLAN ファブリックを構築しました。

- EVPN ルートリフレクタとしての 2 つのスパイン
- VLAN/VNI マッピングを行う VTEP としての 2 つのリーフ
- ユーザーおよびツール用 VLAN のエニキャストゲートウェイ
- L2 ストレッチを実証するために異なるリーフに接続されたホスト

## 2. トポロジ

- スパイン: spine-1, spine-2 (AS 65000)
- リーフ: leaf-1 (AS 65101), leaf-2 (AS 65102)
- host-a: leaf-1 の VLAN 10 に接続
- host-b: leaf-2 の VLAN 10 に接続
- rust-toolkit: leaf-1 の VLAN 20 に接続

## 3. 主要技術

- OSPF アンダーレイ (エリア 0)
- BGP EVPN オーバーレイ
- VXLAN データプレーン
- VLAN 10 および 20 用のエニキャストゲートウェイ

## 4. クイックスタート

```bash
cd 05-evpn
docker compose up -d

docker exec -it spine-1 Cli
docker exec -it spine-2 Cli
docker exec -it leaf-1 Cli
docker exec -it leaf-2 Cli
```

## 5. 検証
### アンダーレイ
```bash
show ip ospf neighbor
show ip route
```
### EVPN / VXLAN
```bash
show bgp evpn summary
show bgp evpn route-type mac-ip
show vxlan
show vxlan vni
show mac address-table dynamic
```

### エンドツーエンド通信
host-a から: `ping 192.168.10.11` (host-b への疎通確認)

## 6. ハウスキーピングと安定性
各 cEOS コンテナには CPU とメモリの制限を設定し、ヘルスチェックによってスイッチの管理プレーンが完全に起動してから検証を行えるようにしています。

## 8. まとめ
- EVPN コントロールプレーン (Type-2/Type-3 ルート) の理解
- VXLAN VNI と VLAN のマッピング
- 複数リーフにわたるエニキャストゲートウェイの構築
- EVPN ファブリックへの自作ツール (Rust) の統合