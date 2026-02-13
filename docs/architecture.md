# アーキテクチャ概要

## システム全体図

```
┌─────────────┐     stdio      ┌──────────────────┐     HTTPS      ┌──────────────┐
│   Claude     │ ◄────────────► │  websearch-mcp   │ ──────────────► │  DuckDuckGo  │
│  (Host App)  │   JSON-RPC     │   (MCP Server)   │   HTTP GET      │   Servers    │
└─────────────┘                 └──────────────────┘                 └──────────────┘
                                        │
                                        ├── Instant Answer API
                                        │   (api.duckduckgo.com)
                                        │
                                        └── HTML Lite Search
                                            (html.duckduckgo.com)
```

Claude (ホストアプリ) と本サーバーは **stdio** を通じて JSON-RPC 2.0 で通信します。サーバーは受け取ったリクエストに応じて DuckDuckGo の各エンドポイントに HTTP リクエストを送信し、結果をパースして MCP レスポンスとして返却します。

## モジュール構成

```
src/
├── main.rs            # エントリーポイント、サーバー起動
├── server.rs          # MCPサーバー定義、ツールディスパッチ
├── config.rs          # 環境変数からの設定読み込み
├── error.rs           # WebSearchError 型定義
├── http_client.rs     # 共有 reqwest::Client の構築
├── tools/
│   ├── mod.rs         # ツールモジュールの再エクスポート
│   ├── web_search.rs  # web_search ツールの実装
│   └── instant_answer.rs  # instant_answer ツールの実装
└── models/
    ├── mod.rs         # モデルモジュールの再エクスポート
    ├── search.rs      # 検索結果のデータモデル
    └── instant_answer.rs  # Instant Answer APIのレスポンスモデル
```

### 各モジュールの責務

| モジュール | 責務 |
|---|---|
| `main.rs` | tracing の初期化、`Config` の読み込み、MCP サーバーの起動 |
| `server.rs` | `#[tool]` マクロによるツール定義、`ServerHandler` トレイトの実装 |
| `config.rs` | 環境変数 (`WEBSEARCH_*`) の読み込みとデフォルト値の適用 |
| `error.rs` | `WebSearchError` enum の定義、`thiserror` による `Display` / `Error` 実装 |
| `http_client.rs` | `reqwest::Client` の構築（User-Agent、タイムアウト設定） |
| `tools/web_search.rs` | HTML Lite ページの取得・スクレイピング・結果整形 |
| `tools/instant_answer.rs` | Instant Answer API の呼び出し・レスポンスパース・結果整形 |
| `models/search.rs` | `SearchResult` 構造体（title, url, snippet） |
| `models/instant_answer.rs` | `InstantAnswerResponse`, `RelatedTopic` 等の構造体 |

## 主要依存クレート

| クレート | 用途 |
|---|---|
| `rmcp` | MCP プロトコルの実装。`#[tool]` マクロによるツール定義、stdio トランスポート |
| `tokio` | 非同期ランタイム。`#[tokio::main]` によるエントリーポイント |
| `reqwest` | HTTP クライアント。DuckDuckGo への GET リクエスト送信 |
| `serde` / `serde_json` | JSON のシリアライズ / デシリアライズ |
| `schemars` | ツール入力パラメータの JSON Schema 自動生成 |
| `scraper` | HTML パーシング。CSS セレクタによる要素抽出 |
| `thiserror` | エラー型の derive マクロ |
| `tracing` / `tracing-subscriber` | 構造化ログ出力 |

## データフロー

```
1. JSON-RPC リクエスト受信 (stdin)
   │
   ▼
2. rmcp がリクエストをデシリアライズ
   │
   ▼
3. ツール名に基づきディスパッチ
   ├── "web_search"      → tools::web_search::execute()
   └── "instant_answer"  → tools::instant_answer::execute()
   │
   ▼
4. reqwest::Client で DuckDuckGo に HTTP GET
   │
   ▼
5. レスポンスをパース
   ├── HTML Lite → scraper で DOM 解析
   └── JSON API  → serde_json でデシリアライズ
   │
   ▼
6. 結果を Markdown 形式に整形
   │
   ▼
7. CallToolResult として JSON-RPC レスポンス返却 (stdout)
```

## ログ戦略

MCP プロトコルでは **stdout は JSON-RPC 通信専用** です。そのため、アプリケーションのログはすべて **stderr** に出力します。

```rust
// main.rs でのログ初期化例
tracing_subscriber::fmt()
    .with_writer(std::io::stderr)
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .init();
```

- ログレベルは `RUST_LOG` 環境変数で制御
- デフォルトは `info` レベル
- 開発時は `RUST_LOG=debug` または `RUST_LOG=trace` で詳細ログを有効化
