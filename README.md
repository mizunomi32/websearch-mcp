# websearch-mcp

[![CI](https://github.com/mizunomi32/websearch-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/mizunomi32/websearch-mcp/actions/workflows/ci.yml)

DuckDuckGo を使った Web 検索機能を提供する MCP (Model Context Protocol) サーバーです。Rust で実装されており、API キー不要で利用できます。

## 機能一覧

本サーバーは以下の 2 つの MCP ツールを提供します。

| ツール名 | 説明 | データソース |
|---|---|---|
| `web_search` | キーワードによる Web 検索を実行し、検索結果一覧を返す | DuckDuckGo HTML Lite (`html.duckduckgo.com`) をスクレイピング |
| `instant_answer` | クエリに対する即時回答（定義・要約・関連トピック等）を返す | DuckDuckGo Instant Answer API (`api.duckduckgo.com`) |

## 技術スタック

| カテゴリ | クレート / 技術 |
|---|---|
| MCP SDK | [rmcp](https://crates.io/crates/rmcp) (公式 Rust SDK) |
| HTTP クライアント | [reqwest](https://crates.io/crates/reqwest) |
| HTML パーサー | [scraper](https://crates.io/crates/scraper) |
| 非同期ランタイム | [tokio](https://crates.io/crates/tokio) |
| シリアライズ | [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) |
| JSON Schema 生成 | [schemars](https://crates.io/crates/schemars) |
| エラーハンドリング | [thiserror](https://crates.io/crates/thiserror) |
| ログ | [tracing](https://crates.io/crates/tracing) / [tracing-subscriber](https://crates.io/crates/tracing-subscriber) |

## 前提条件

- Rust toolchain (1.75 以上推奨)
- ネットワーク接続（DuckDuckGo へのアクセスが必要）

## インストール

### Homebrew (macOS)

```bash
brew tap mizunomi32/websearch-mcp https://github.com/mizunomi32/websearch-mcp.git
brew install websearch-mcp
```

### APT (Debian/Ubuntu)

```bash
# GPG 鍵を追加
curl -fsSL https://mizunomi32.github.io/websearch-mcp/websearch-mcp-keyring.gpg \
  | sudo tee /usr/share/keyrings/websearch-mcp-keyring.gpg > /dev/null

# リポジトリを追加
echo "deb [signed-by=/usr/share/keyrings/websearch-mcp-keyring.gpg] https://mizunomi32.github.io/websearch-mcp stable main" \
  | sudo tee /etc/apt/sources.list.d/websearch-mcp.list > /dev/null

# インストール
sudo apt update
sudo apt install websearch-mcp
```

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/mizunomi32/websearch-mcp.git
cd websearch-mcp

# リリースビルド
cargo build --release

# バイナリは target/release/websearch-mcp に生成されます
```

## 使い方

### Claude Desktop での設定

`claude_desktop_config.json` に以下を追加します。

```json
{
  "mcpServers": {
    "websearch": {
      "command": "/path/to/websearch-mcp"
    }
  }
}
```

### Claude Code での設定

`.mcp.json` に以下を追加します。

```json
{
  "mcpServers": {
    "websearch": {
      "command": "/path/to/websearch-mcp"
    }
  }
}
```

サーバーは stdio トランスポートで MCP 通信を行います。

## 設定

環境変数で動作をカスタマイズできます。

| 環境変数 | 説明 | デフォルト値 |
|---|---|---|
| `WEBSEARCH_MAX_RESULTS` | `web_search` のデフォルト最大検索結果数 | `10` |
| `WEBSEARCH_TIMEOUT_SECS` | HTTP リクエストのタイムアウト（秒） | `10` |
| `WEBSEARCH_USER_AGENT` | HTTP リクエストに使用する User-Agent 文字列 | `websearch-mcp/0.1` |
| `WEBSEARCH_CACHE_TTL_SECS` | レスポンスキャッシュの TTL（秒） | `300` |
| `WEBSEARCH_RATE_LIMIT_MS` | リクエスト間の最小間隔（ミリ秒） | `1000` |
| `WEBSEARCH_MAX_RETRIES` | 429/5xx/タイムアウト時の最大リトライ回数 | `3` |

## 開発

### テスト実行

```bash
cargo test -- --test-threads=1
```

> **Note:** 一部のテストがグローバルな状態（環境変数など）を共有するため、`--test-threads=1` でシングルスレッド実行する必要があります。

### Lint・フォーマット

```bash
# フォーマットチェック
cargo fmt --all --check

# Lint チェック
cargo clippy --all-targets -- -D warnings
```

### CI

GitHub Actions により、push および Pull Request 時にテスト・Lint・フォーマットチェックが自動実行されます。

## プロジェクト構造

```
src/
├── main.rs           # エントリーポイント
├── lib.rs            # モジュール再エクスポート
├── server.rs         # MCP サーバー定義
├── config.rs         # 環境変数読み込み
├── error.rs          # エラー型定義
├── http_client.rs    # HTTP クライアント構築
├── cache.rs          # TTL 付きインメモリキャッシュ
├── rate_limiter.rs   # リクエスト間隔制御
├── retry.rs          # Exponential Backoff リトライ
├── tools/
│   ├── web_search.rs      # Web 検索（HTML パース）
│   └── instant_answer.rs  # Instant Answer（API 連携）
└── models/
    ├── search.rs          # SearchResult 構造体
    └── instant_answer.rs  # API レスポンスモデル
```

## ライセンス

MIT License
