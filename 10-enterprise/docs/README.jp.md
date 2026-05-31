# ラボ 10 – エンタープライズネットワーク (最終課題)

## 1. 起動手順

コンテナを起動する前に、Compose ファイルを検証して、未定義のネットワークや参照の欠落を確認してください：

```bash
docker compose config
```

ラボを開始するには：

```bash
docker compose up -d
```

コンテナが正常な状態でない場合の完全なクリーン再起動：

```bash
docker compose down --remove-orphans
docker network prune -f
docker compose up -d
```

---

## 2. 設定ガイドライン

### WAN Edge ゲートウェイ
安定性を確保するため、`wan-net` はブリッジゲートウェイとして `203.0.113.254` を使用します。これにより、Docker が `wan-edge` コンテナに割り当てられている `.1` を占有するのを防ぎます。
アクセスレイヤーノードには `10.99.0.4x` の範囲が割り当てられ、リーフノードは `10.99.0.2x` の範囲を使用します。
cEOS コンテナは独自の init プロセスを管理することに注意してください。compose ファイルに `command:` オーバーライドを含めないでください。

### 設定ファイル
各 cEOS ノードは、`EOS_CONFIG_FILE` 環境変数を介して `/mnt/flash/<node>.cfg` からスタートアップ設定を読み込みます。マップされたボリュームディレクトリに設定ファイルが存在しない場合、コンテナの起動に失敗します。起動前に以下の設定ファイルが存在することを確認してください：

| ディレクトリ | 必要なファイル |
|---|---|
| `./configs/core/` | `spine-1.cfg`, `spine-2.cfg`, `leaf-1.cfg` – `leaf-4.cfg` |
| `./configs/distribution/` | `dist-1.cfg`, `dist-2.cfg` |
| `./configs/access/` | `access-1.cfg` – `access-4.cfg` |
| `./configs/wan/` | `wan-edge.cfg`, `branch-1.cfg`, `branch-2.cfg` |

---

## 3. トラブルシューティング
初期ビルド中に遭遇した問題（IP 競合や Docker Compose のバージョンに関する問題など）の詳細なログについては、トラブルシューティングログを参照してください。

## 4. トポロジ
!ラボ 10 トポロジ