# Implementation Issues

本ドキュメントでは、websearch-mcp の実装に必要な作業項目を GitHub Issue として起票できる粒度で列挙します。TDD の実装順序（`docs/testing-strategy.md` の Phase 1〜4）に沿い、依存関係の少ないモジュールから順に並べています。

---

## 1. プロジェクト初期セットアップ

### Issue #1: プロジェクトの Cargo.toml を作成する

- **概要**: `Cargo.toml` に依存クレート（`rmcp`, `reqwest`, `scraper`, `tokio`, `serde`, `serde_json`, `schemars`, `thiserror`, `tracing`, `tracing-subscriber`）と dev-dependencies（`wiremock`, `tokio-test`, `assert_matches`）を定義する。
- **対象ファイル**: `Cargo.toml`
- **関連ドキュメント**: `docs/architecture.md`（主要依存クレート）、`docs/testing-strategy.md`（テスト用依存クレート）

### Issue #2: ソースコードのディレクトリ構造を作成する

- **概要**: `src/`, `src/tools/`, `src/models/`, `tests/`, `tests/fixtures/` の各ディレクトリと、`src/tools/mod.rs`, `src/models/mod.rs` の再エクスポートモジュールを作成する。`src/main.rs` と `src/lib.rs` の空ファイルも用意する。
- **対象ファイル**: `src/main.rs`, `src/lib.rs`, `src/tools/mod.rs`, `src/models/mod.rs`
- **関連ドキュメント**: `docs/architecture.md`（モジュール構成）

### Issue #3: .gitignore と LICENSE ファイルを追加する

- **概要**: Rust プロジェクト向けの `.gitignore`（`/target` 等）と MIT License ファイルを追加する。
- **対象ファイル**: `.gitignore`, `LICENSE`
- **関連ドキュメント**: `README.md`（ライセンス）

---

## 2. Phase 1: データモデル

### Issue #4: SearchResult 構造体を定義する

- **概要**: `models/search.rs` に `SearchResult` 構造体（`title: String`, `url: String`, `snippet: String`）を定義する。`#[derive(Debug, Clone)]` を付与する。また、検索結果を Markdown 形式にフォーマットする関数のテストと実装を行う。
- **対象ファイル**: `src/models/search.rs`, `src/models/mod.rs`
- **関連ドキュメント**: `docs/duckduckgo-integration.md`（SearchResult 構造体）、`docs/mcp-tools-spec.md`（出力フォーマット）

### Issue #5: InstantAnswerResponse 関連の構造体を定義する

- **概要**: `models/instant_answer.rs` に `InstantAnswerResponse`, `RelatedTopic`（enum）, `ResultItem` の 3 構造体を定義する。`serde::Deserialize` によるフィールドのリネーム（`#[serde(rename = "Abstract")]` 等）を含む。JSON デシリアライズのテストを作成し、正常レスポンス・空レスポンス・曖昧さ回避レスポンスのケースを検証する。
- **対象ファイル**: `src/models/instant_answer.rs`, `src/models/mod.rs`
- **関連ドキュメント**: `docs/duckduckgo-integration.md`（InstantAnswerResponse, RelatedTopic, ResultItem）

### Issue #6: Config 構造体と環境変数読み込みを実装する

- **概要**: `config.rs` に `Config` 構造体（`max_results: usize`, `timeout_secs: u64`, `user_agent: String`）を定義する。環境変数 `WEBSEARCH_MAX_RESULTS`（デフォルト: 10）、`WEBSEARCH_TIMEOUT_SECS`（デフォルト: 10）、`WEBSEARCH_USER_AGENT`（デフォルト: `websearch-mcp/0.1`）から読み込むロジックのテストと実装を行う。
- **対象ファイル**: `src/config.rs`
- **関連ドキュメント**: `docs/architecture.md`（config.rs の責務）、`README.md`（環境変数一覧）

### Issue #7: WebSearchError 型を定義し MCP エラー変換を実装する

- **概要**: `error.rs` に `WebSearchError` enum（`HttpError`, `HtmlParseError`, `JsonParseError`, `EmptyQuery`, `Timeout`, `NoResults`）を `thiserror` で定義する。各バリアントから `CallToolResult` への変換メソッド `to_tool_result()` と、ユーザー向けメッセージを返す `user_message()` のテストと実装を行う。`NoResults` は `is_error: false` で返す点に注意。
- **対象ファイル**: `src/error.rs`
- **関連ドキュメント**: `docs/error-handling.md`（エラー型定義、MCP エラーへの変換、変換対応表）

---

## 3. Phase 2: ビジネスロジック

### Issue #8: テストフィクスチャ（HTML）を用意する

- **概要**: DuckDuckGo HTML Lite から実際の検索結果ページを取得し、`tests/fixtures/search_results.html`（正常結果）と `tests/fixtures/search_results_empty.html`（0 件結果）として保存する。
- **対象ファイル**: `tests/fixtures/search_results.html`, `tests/fixtures/search_results_empty.html`
- **関連ドキュメント**: `docs/testing-strategy.md`（テストデータ管理、フィクスチャの取得方法）

### Issue #9: テストフィクスチャ（JSON）を用意する

- **概要**: DuckDuckGo Instant Answer API から実際のレスポンスを取得し、`tests/fixtures/instant_answer.json`（正常レスポンス）、`tests/fixtures/instant_answer_empty.json`（情報なし）、`tests/fixtures/instant_answer_disambig.json`（曖昧さ回避）として保存する。
- **対象ファイル**: `tests/fixtures/instant_answer.json`, `tests/fixtures/instant_answer_empty.json`, `tests/fixtures/instant_answer_disambig.json`
- **関連ドキュメント**: `docs/testing-strategy.md`（テストデータ管理、フィクスチャの取得方法）

### Issue #10: web_search の HTML パース関数を実装する

- **概要**: `tools/web_search.rs` に `parse_html_results(html: &str, max_results: usize) -> Vec<SearchResult>` 関数を実装する。`scraper` クレートで CSS セレクタ（`.result`, `.result__a`, `.result__snippet`）を使い、タイトル・URL・スニペットを抽出する。フィクスチャ HTML を使ったユニットテストで、結果の抽出・`max_results` の制限・空 HTML のケースを検証する。
- **対象ファイル**: `src/tools/web_search.rs`, `src/tools/mod.rs`
- **関連ドキュメント**: `docs/duckduckgo-integration.md`（HTML Lite パース戦略、CSS セレクタ、パース実装例）

### Issue #11: web_search の結果を Markdown にフォーマットする関数を実装する

- **概要**: `tools/web_search.rs` に、`Vec<SearchResult>` を `docs/mcp-tools-spec.md` で定義された Markdown 形式に整形する関数を実装する。結果あり・結果なしの両ケースのテストを含む。末尾に `_Source: DuckDuckGo (N results)_` を付与する。
- **対象ファイル**: `src/tools/web_search.rs`
- **関連ドキュメント**: `docs/mcp-tools-spec.md`（web_search 出力フォーマット）、`docs/duckduckgo-integration.md`（帰属表示の義務）

### Issue #12: instant_answer の API レスポンス整形関数を実装する

- **概要**: `tools/instant_answer.rs` に、`InstantAnswerResponse` を Markdown 形式に整形する関数を実装する。Abstract セクション（テキスト・出典・URL）と Related Topics セクションを含む。フィクスチャ JSON を使ったユニットテストで、正常レスポンス・情報なし・曖昧さ回避レスポンスのケースを検証する。末尾に `_Source: DuckDuckGo Instant Answer API_` を付与する。
- **対象ファイル**: `src/tools/instant_answer.rs`, `src/tools/mod.rs`
- **関連ドキュメント**: `docs/mcp-tools-spec.md`（instant_answer 出力フォーマット）、`docs/duckduckgo-integration.md`（帰属表示の義務）

---

## 4. Phase 3: HTTP 統合

### Issue #13: HTTP クライアント構築関数を実装する

- **概要**: `http_client.rs` に `build_http_client(config: &Config) -> Result<Client, reqwest::Error>` 関数を実装する。`Config` の `user_agent` と `timeout_secs` を反映した `reqwest::Client` を構築する。クライアントが正しく構築できることを検証するテストを含む。
- **対象ファイル**: `src/http_client.rs`
- **関連ドキュメント**: `docs/duckduckgo-integration.md`（HTTP クライアント設定、構築例）

### Issue #14: web_search の HTTP 統合テスト（wiremock）を実装する

- **概要**: `tests/` ディレクトリに wiremock を使った `web_search` の統合テストを作成する。モック HTML レスポンスに対するツール全体の動作、サーバーエラー（500）時のエラーハンドリング、タイムアウト時の処理を検証する。
- **対象ファイル**: `tests/web_search_integration.rs`
- **関連ドキュメント**: `docs/testing-strategy.md`（統合テスト例）、`docs/error-handling.md`（変換対応表）

### Issue #15: instant_answer の HTTP 統合テスト（wiremock）を実装する

- **概要**: `tests/` ディレクトリに wiremock を使った `instant_answer` の統合テストを作成する。モック JSON レスポンスに対するツール全体の動作、サーバーエラー（500）時のエラーハンドリング、タイムアウト時の処理を検証する。
- **対象ファイル**: `tests/instant_answer_integration.rs`
- **関連ドキュメント**: `docs/testing-strategy.md`（統合テスト例）、`docs/error-handling.md`（変換対応表）

---

## 5. Phase 4: MCP サーバー統合

### Issue #16: MCP サーバー（server.rs）を実装する

- **概要**: `server.rs` に `#[tool]` マクロを使った `web_search` と `instant_answer` の 2 ツールを定義し、`ServerHandler` トレイトを実装する。ツールのディスパッチ処理、入力バリデーション（空クエリチェック）、エラーハンドリング（`WebSearchError` → `CallToolResult` 変換）を含む。
- **対象ファイル**: `src/server.rs`
- **関連ドキュメント**: `docs/architecture.md`（server.rs の責務、データフロー）、`docs/mcp-tools-spec.md`（tools/list レスポンス）

### Issue #17: エントリーポイント（main.rs）を実装する

- **概要**: `main.rs` に `#[tokio::main]` エントリーポイントを実装する。`tracing_subscriber` の初期化（stderr への出力）、`Config` の読み込み、HTTP クライアントの構築、MCP サーバーの stdio トランスポートでの起動を行う。
- **対象ファイル**: `src/main.rs`
- **関連ドキュメント**: `docs/architecture.md`（main.rs の責務、ログ戦略）

### Issue #18: MCP E2E テストを実装する

- **概要**: MCP プロトコルレベルの E2E テストを作成する。JSON-RPC メッセージを stdin/stdout で送受信し、`tools/list` が 2 ツールを返すこと、`tools/call` で `web_search` と `instant_answer` が正しく動作すること、存在しないツール名を指定した場合のエラーを検証する。
- **対象ファイル**: `tests/e2e.rs`
- **関連ドキュメント**: `docs/testing-strategy.md`（E2E テスト）、`docs/mcp-tools-spec.md`（tools/list レスポンス）

---

## 6. CI/CD

### Issue #19: GitHub Actions ワークフローを作成する

- **概要**: `.github/workflows/ci.yml` に CI ワークフローを作成する。`cargo fmt --check`、`cargo clippy`、`cargo test` をステップとして実行する。Rust のバージョンは stable を使用する。
- **対象ファイル**: `.github/workflows/ci.yml`
- **関連ドキュメント**: `docs/testing-strategy.md`（テスト実行コマンド）

---

## Issue 依存関係

```
Issue #1 (Cargo.toml)
Issue #2 (ディレクトリ構造)     ──┐
Issue #3 (.gitignore, LICENSE)   │
                                 ▼
Issue #4 (SearchResult)        ──┐
Issue #5 (InstantAnswerResponse) │
Issue #6 (Config)                │
Issue #7 (WebSearchError)        │
                                 ▼
Issue #8 (HTML フィクスチャ)   ──┐
Issue #9 (JSON フィクスチャ)     │
                                 ▼
Issue #10 (HTML パース)        ──┐
Issue #11 (web_search 整形)      │
Issue #12 (instant_answer 整形)  │
                                 ▼
Issue #13 (HTTP クライアント)  ──┐
Issue #14 (web_search 統合)      │
Issue #15 (instant_answer 統合)  │
                                 ▼
Issue #16 (server.rs)          ──┐
Issue #17 (main.rs)              │
Issue #18 (E2E テスト)           │
                                 ▼
Issue #19 (CI/CD)
```
