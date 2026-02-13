# DuckDuckGo API 統合仕様

本ドキュメントでは、DuckDuckGo の 2 つのエンドポイントとの統合方法を定義します。

## 2 つのエンドポイント

### 1. Instant Answer API

| 項目 | 値 |
|---|---|
| URL | `https://api.duckduckgo.com/` |
| メソッド | GET |
| レスポンス形式 | JSON |
| 認証 | 不要 |
| レートリミット | 明示的な制限なし（常識的な利用を前提） |

#### クエリパラメータ

| パラメータ | 値 | 説明 |
|---|---|---|
| `q` | 検索キーワード | 検索クエリ |
| `format` | `json` | レスポンス形式の指定 |
| `no_html` | `1` | HTML タグを除去 |
| `skip_disambig` | `1` | 曖昧さ回避ページをスキップ |

#### リクエスト例

```
GET https://api.duckduckgo.com/?q=rust+programming&format=json&no_html=1&skip_disambig=1
```

### 2. HTML Lite Search

| 項目 | 値 |
|---|---|
| URL | `https://html.duckduckgo.com/html/` |
| メソッド | GET |
| レスポンス形式 | HTML |
| 認証 | 不要 |
| レートリミット | 明示的な制限なし（常識的な利用を前提） |

#### クエリパラメータ

| パラメータ | 値 | 説明 |
|---|---|---|
| `q` | 検索キーワード | 検索クエリ |

#### リクエスト例

```
GET https://html.duckduckgo.com/html/?q=rust+programming
```

---

## Instant Answer API レスポンス構造体

API レスポンスを Rust の構造体にマッピングします。

### `InstantAnswerResponse`

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct InstantAnswerResponse {
    /// 要約テキスト
    #[serde(rename = "Abstract")]
    pub abstract_text: String,

    /// 要約の出典元
    #[serde(rename = "AbstractSource")]
    pub abstract_source: String,

    /// 要約の出典 URL
    #[serde(rename = "AbstractURL")]
    pub abstract_url: String,

    /// 直接的な回答（計算結果など）
    #[serde(rename = "Answer")]
    pub answer: String,

    /// 定義テキスト
    #[serde(rename = "Definition")]
    pub definition: String,

    /// 定義の出典元
    #[serde(rename = "DefinitionSource")]
    pub definition_source: String,

    /// 定義の出典 URL
    #[serde(rename = "DefinitionURL")]
    pub definition_url: String,

    /// 関連トピック一覧
    #[serde(rename = "RelatedTopics")]
    pub related_topics: Vec<RelatedTopic>,

    /// レスポンスの種類 ("A": article, "D": disambiguation, etc.)
    #[serde(rename = "Type")]
    pub response_type: String,
}
```

### `RelatedTopic`

DuckDuckGo の `RelatedTopics` は 2 種類の形式を含みます：

1. **トピック項目** - `Text` と `FirstURL` を持つ個別のトピック
2. **カテゴリグループ** - `Name` と `Topics` を持つグループ

```rust
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RelatedTopic {
    /// 個別のトピック項目
    Topic(ResultItem),
    /// カテゴリでグループ化されたトピック
    Category {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "Topics")]
        topics: Vec<ResultItem>,
    },
}
```

### `ResultItem`

```rust
#[derive(Debug, Deserialize)]
pub struct ResultItem {
    /// トピックのテキスト説明
    #[serde(rename = "Text")]
    pub text: String,

    /// トピックの URL
    #[serde(rename = "FirstURL")]
    pub first_url: String,
}
```

---

## HTML Lite パース戦略

`scraper` クレートの CSS セレクタを使用して、HTML Lite の検索結果ページから情報を抽出します。

### HTML 構造と CSS セレクタ

DuckDuckGo HTML Lite の検索結果は以下の構造を持ちます：

```html
<div class="result results_links results_links_deep web-result">
  <div class="links_main links_deep result__body">
    <h2 class="result__title">
      <a class="result__a" href="https://example.com">Result Title</a>
    </h2>
    <a class="result__snippet" href="https://example.com">
      Result snippet text...
    </a>
    <a class="result__url" href="https://example.com">
      example.com
    </a>
  </div>
</div>
```

### 使用するセレクタ

| 要素 | CSS セレクタ | 取得方法 |
|---|---|---|
| 検索結果コンテナ | `.result` | 各結果のルート要素 |
| タイトル | `.result__a` | `element.text()` でテキスト取得 |
| URL | `.result__a` | `element.value().attr("href")` で href 属性取得 |
| スニペット | `.result__snippet` | `element.text()` でテキスト取得 |

### パース実装例

```rust
use scraper::{Html, Selector};
use crate::models::search::SearchResult;

pub fn parse_html_results(html: &str, max_results: usize) -> Vec<SearchResult> {
    let document = Html::parse_document(html);
    let result_selector = Selector::parse(".result").unwrap();
    let title_selector = Selector::parse(".result__a").unwrap();
    let snippet_selector = Selector::parse(".result__snippet").unwrap();

    document
        .select(&result_selector)
        .filter_map(|result| {
            let title_el = result.select(&title_selector).next()?;
            let title = title_el.text().collect::<String>().trim().to_string();
            let url = title_el.value().attr("href")?.to_string();
            let snippet = result
                .select(&snippet_selector)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            Some(SearchResult { title, url, snippet })
        })
        .take(max_results)
        .collect()
}
```

### `SearchResult` 構造体

```rust
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 検索結果のタイトル
    pub title: String,
    /// 検索結果の URL
    pub url: String,
    /// 検索結果のスニペット（説明文）
    pub snippet: String,
}
```

---

## HTTP クライアント設定

すべての HTTP リクエストで共有する `reqwest::Client` を構築します。

### 設定項目

| 設定項目 | 値 | ソース |
|---|---|---|
| User-Agent | `websearch-mcp/0.1` | `WEBSEARCH_USER_AGENT` 環境変数またはデフォルト値 |
| タイムアウト | 10 秒 | `WEBSEARCH_TIMEOUT_SECS` 環境変数またはデフォルト値 |
| リダイレクト | 自動追従 | reqwest のデフォルト |

### 構築例

```rust
use reqwest::Client;
use std::time::Duration;
use crate::config::Config;

pub fn build_http_client(config: &Config) -> Result<Client, reqwest::Error> {
    Client::builder()
        .user_agent(&config.user_agent)
        .timeout(Duration::from_secs(config.timeout_secs))
        .build()
}
```

---

## 帰属表示の義務

DuckDuckGo の利用規約に基づき、検索結果の出典を明示する必要があります。

### 対応方針

- `web_search` の結果末尾に `_Source: DuckDuckGo_` を付与
- `instant_answer` の結果末尾に `_Source: DuckDuckGo Instant Answer API_` を付与
- Abstract の出典元がある場合は `**Source:** {AbstractSource}` も表示

### 参考

- [DuckDuckGo API Terms of Service](https://duckduckgo.com/api)
- Instant Answer API は非商用利用で無料、帰属表示が必要
