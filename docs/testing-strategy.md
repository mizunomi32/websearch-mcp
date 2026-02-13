# テスト戦略（TDD）

本ドキュメントでは、websearch-mcp の TDD に基づくテスト戦略を定義します。

## TDD サイクル

本プロジェクトではテスト駆動開発 (TDD) を採用します。以下のサイクルを厳格に守ります。

```
1. テストを書く（RED）
   - 期待される入出力に基づきテストを作成
   - 実装コードは書かない
   │
   ▼
2. テストを実行し、失敗を確認（RED）
   - テストが正しく失敗することを確認
   - コンパイルエラーではなく、アサーションの失敗を目指す
   │
   ▼
3. テストをコミット
   - 失敗するテストの状態でコミット
   - "test: add tests for ..." のコミットメッセージ
   │
   ▼
4. 実装を書く（GREEN）
   - テストをパスさせる最小限の実装を書く
   - テストコードは変更しない
   │
   ▼
5. すべてのテストが通過するまで繰り返す（GREEN）
   │
   ▼
6. リファクタリング（REFACTOR）
   - テストが通る状態を維持しながらコードを整理
   - 実装コミット: "feat: implement ..." or "refactor: ..."
```

---

## テスト階層

テストを 3 つの階層に分けて実装します。

### 1. ユニットテスト

各モジュール内に `#[cfg(test)] mod tests` として配置します。

| テスト対象 | テスト内容 |
|---|---|
| `models/search.rs` | `SearchResult` の構築・フォーマット出力 |
| `models/instant_answer.rs` | `InstantAnswerResponse` の JSON デシリアライズ |
| `tools/web_search.rs` | HTML パース関数が正しく検索結果を抽出するか |
| `tools/instant_answer.rs` | API レスポンスの整形が正しいか |
| `config.rs` | 環境変数からの設定読み込み・デフォルト値の適用 |
| `error.rs` | `WebSearchError` → `CallToolResult` の変換 |

#### ユニットテスト例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_results_extracts_titles() {
        let html = include_str!("../../tests/fixtures/search_results.html");
        let results = parse_html_results(html, 10);

        assert!(!results.is_empty());
        assert!(!results[0].title.is_empty());
        assert!(!results[0].url.is_empty());
    }

    #[test]
    fn test_parse_html_results_respects_max_results() {
        let html = include_str!("../../tests/fixtures/search_results.html");
        let results = parse_html_results(html, 3);

        assert!(results.len() <= 3);
    }

    #[test]
    fn test_parse_html_results_empty_html() {
        let html = "<html><body></body></html>";
        let results = parse_html_results(html, 10);

        assert!(results.is_empty());
    }
}
```

### 2. 統合テスト（wiremock）

`tests/` ディレクトリに配置し、`wiremock` でモック HTTP サーバーを立ててテストします。実際の DuckDuckGo へのリクエストは行いません。

| テスト対象 | テスト内容 |
|---|---|
| `web_search` E2E | モック HTML レスポンスに対してツール全体が正しく動作するか |
| `instant_answer` E2E | モック JSON レスポンスに対してツール全体が正しく動作するか |
| HTTP エラー | サーバーエラー (500) 時のエラーハンドリング |
| タイムアウト | レスポンス遅延時のタイムアウト処理 |

#### 統合テスト例

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};

#[tokio::test]
async fn test_web_search_returns_results() {
    let mock_server = MockServer::start().await;
    let html_body = include_str!("fixtures/search_results.html");

    Mock::given(method("GET"))
        .and(path("/html/"))
        .and(query_param("q", "rust"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html_body))
        .mount(&mock_server)
        .await;

    // モックサーバーの URL を使用してツールを実行
    let result = execute_web_search(&mock_server.uri(), "rust", 10).await;

    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Rust"));
}

#[tokio::test]
async fn test_web_search_handles_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/html/"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = execute_web_search(&mock_server.uri(), "rust", 10).await;

    assert!(result.is_err());
}
```

### 3. E2E テスト（MCP JSON-RPC）

MCP プロトコルレベルでの E2E テストです。実際の JSON-RPC メッセージを stdin/stdout で送受信してサーバー全体の動作を検証します。

| テスト対象 | テスト内容 |
|---|---|
| `tools/list` | ツール一覧が正しく返却されるか |
| `tools/call` (web_search) | JSON-RPC 経由での web_search 呼び出し |
| `tools/call` (instant_answer) | JSON-RPC 経由での instant_answer 呼び出し |
| 不明なツール | 存在しないツール名を指定した場合のエラー |

---

## テストデータ管理

テスト用の固定データは `tests/fixtures/` ディレクトリに配置します。

```
tests/
└── fixtures/
    ├── search_results.html          # DuckDuckGo HTML Lite の検索結果ページサンプル
    ├── search_results_empty.html    # 検索結果 0 件のページサンプル
    ├── instant_answer.json          # Instant Answer API の正常レスポンスサンプル
    ├── instant_answer_empty.json    # 情報なしの API レスポンスサンプル
    └── instant_answer_disambig.json # 曖昧さ回避レスポンスサンプル
```

### フィクスチャの取得方法

実際の DuckDuckGo レスポンスを保存してフィクスチャとして使用します：

```bash
# HTML Lite の検索結果を取得
curl -o tests/fixtures/search_results.html \
  "https://html.duckduckgo.com/html/?q=rust+programming"

# Instant Answer API のレスポンスを取得
curl -o tests/fixtures/instant_answer.json \
  "https://api.duckduckgo.com/?q=rust+programming&format=json&no_html=1"
```

---

## TDD 実装順序

以下の順序で TDD サイクルを実施します。依存関係の少ないモジュールから順に進めます。

### Phase 1: データモデル

1. `models/search.rs` - `SearchResult` 構造体の定義とテスト
2. `models/instant_answer.rs` - `InstantAnswerResponse` のデシリアライズテスト
3. `config.rs` - `Config` 構造体と環境変数読み込みのテスト
4. `error.rs` - `WebSearchError` の定義と変換テスト

### Phase 2: ビジネスロジック

5. `tools/web_search.rs` - HTML パース関数のテスト（フィクスチャ使用）
6. `tools/instant_answer.rs` - API レスポンス整形のテスト（フィクスチャ使用）

### Phase 3: HTTP 統合

7. `http_client.rs` - クライアント構築のテスト
8. `tools/web_search.rs` - wiremock を使った統合テスト
9. `tools/instant_answer.rs` - wiremock を使った統合テスト

### Phase 4: MCP サーバー統合

10. `server.rs` - MCP ツール登録とディスパッチのテスト
11. E2E テスト - JSON-RPC メッセージによるサーバー全体テスト

---

## テスト用依存クレート

`Cargo.toml` の `[dev-dependencies]` に以下を追加します。

| クレート | 用途 |
|---|---|
| `wiremock` | HTTP モックサーバー。統合テストで DuckDuckGo をモック |
| `tokio-test` | 非同期テストユーティリティ |
| `assert_matches` | パターンマッチによるアサーション |

```toml
[dev-dependencies]
wiremock = "0.6"
tokio-test = "0.4"
assert_matches = "1.5"
tokio = { version = "1", features = ["test-util", "macros"] }
```

---

## テスト実行コマンド

```bash
# 全テスト実行
cargo test

# 特定モジュールのテスト
cargo test --lib models::search

# 統合テストのみ
cargo test --test '*'

# テスト出力を表示
cargo test -- --nocapture

# 特定テストの実行
cargo test test_parse_html_results
```
