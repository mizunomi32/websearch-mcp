# エラーハンドリング戦略

本ドキュメントでは、websearch-mcp のエラーハンドリング方針を定義します。

## エラー型定義

`thiserror` クレートを使用して `WebSearchError` enum を定義します。

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebSearchError {
    /// HTTP リクエストの送信に失敗
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// HTML パースに失敗（期待する要素が見つからない等）
    #[error("Failed to parse HTML response: {0}")]
    HtmlParseError(String),

    /// JSON デシリアライズに失敗
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] serde_json::Error),

    /// クエリが空文字列
    #[error("Query must not be empty")]
    EmptyQuery,

    /// HTTP リクエストがタイムアウト
    #[error("Request timed out after {0} seconds")]
    Timeout(u64),

    /// 検索結果が 0 件（エラーではなく正常ケースとして扱うが、ログ用に定義）
    #[error("No results found for query: {0}")]
    NoResults(String),
}
```

## MCP エラーへの変換

内部の `WebSearchError` は MCP の `CallToolResult` に変換して返却します。MCP プロトコルではツール呼び出しのエラーを `is_error: true` フラグで表現します。

### 変換対応表

| WebSearchError | is_error | ユーザーへのメッセージ | ログレベル |
|---|---|---|---|
| `HttpError` | `true` | `"Failed to fetch search results. Please try again later."` | ERROR |
| `HtmlParseError` | `true` | `"Failed to parse search results. The page structure may have changed."` | ERROR |
| `JsonParseError` | `true` | `"Failed to parse API response."` | ERROR |
| `EmptyQuery` | `true` | `"Query must not be empty."` | WARN |
| `Timeout` | `true` | `"Request timed out. Please try again."` | WARN |
| `NoResults` | `false` | `"No results found."`（正常レスポンスとして返却） | INFO |

### 変換実装例

```rust
use rmcp::model::CallToolResult;

impl WebSearchError {
    pub fn to_tool_result(&self) -> CallToolResult {
        match self {
            WebSearchError::NoResults(_) => {
                // 結果なしは正常ケースとして扱う
                CallToolResult::success(vec![self.to_string().into()])
            }
            _ => {
                // その他はエラーとして返却
                CallToolResult::error(vec![self.user_message().into()])
            }
        }
    }

    fn user_message(&self) -> &str {
        match self {
            WebSearchError::HttpError(_) => {
                "Failed to fetch search results. Please try again later."
            }
            WebSearchError::HtmlParseError(_) => {
                "Failed to parse search results. The page structure may have changed."
            }
            WebSearchError::JsonParseError(_) => {
                "Failed to parse API response."
            }
            WebSearchError::EmptyQuery => {
                "Query must not be empty."
            }
            WebSearchError::Timeout(_) => {
                "Request timed out. Please try again."
            }
            WebSearchError::NoResults(_) => {
                "No results found."
            }
        }
    }
}
```

### 設計方針

- **内部エラーの詳細はユーザーに公開しない**: `reqwest::Error` や `serde_json::Error` の詳細メッセージはログに記録し、ユーザーには汎用的なメッセージを返す
- **NoResults は正常ケース**: 検索結果が 0 件であることはエラーではなく、正常なレスポンスとして `is_error: false` で返却する
- **EmptyQuery はバリデーションエラー**: MCP リクエストを処理する前に入力を検証し、空クエリは早期にエラーを返す

---

## リトライ戦略

### 初期バージョン (v0.1)

初期バージョンでは**リトライを実装しない**。理由：

1. MCP ツール呼び出しは LLM が再試行を判断できる
2. 実装の複雑さを抑えることを優先する
3. DuckDuckGo へのリクエスト頻度を抑制する

### 将来の方針 (v0.2 以降)

将来的にリトライを実装する場合の方針：

| 項目 | 値 |
|---|---|
| リトライ対象 | `HttpError`（ネットワークエラー）、`Timeout` のみ |
| 最大リトライ回数 | 2 回 |
| バックオフ戦略 | 指数バックオフ（1 秒 → 2 秒） |
| リトライ対象外 | `EmptyQuery`, `HtmlParseError`, `JsonParseError` |

---

## ログレベル方針

`tracing` クレートのログレベルを以下の基準で使い分けます。

| レベル | 用途 | 例 |
|---|---|---|
| `ERROR` | リクエスト処理が失敗した場合 | HTTP エラー、パースエラー |
| `WARN` | 処理は完了するが注意が必要な場合 | 空クエリ、タイムアウト |
| `INFO` | 正常な処理の主要ポイント | ツール呼び出し開始・完了、結果件数 |
| `DEBUG` | 開発時のデバッグ情報 | リクエスト URL、レスポンスサイズ |
| `TRACE` | 詳細なデバッグ情報 | レスポンスボディ全体、パース中間結果 |

### ログ出力例

```rust
// INFO: ツール呼び出し開始
tracing::info!(tool = "web_search", query = %query, "Tool invoked");

// DEBUG: リクエスト送信
tracing::debug!(url = %url, "Sending HTTP request");

// INFO: 正常完了
tracing::info!(tool = "web_search", results = count, "Tool completed");

// ERROR: HTTP エラー
tracing::error!(error = %err, "HTTP request failed");

// WARN: 空クエリ
tracing::warn!("Empty query received");
```
