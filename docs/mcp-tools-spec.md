# MCP ツール仕様

本ドキュメントでは、websearch-mcp が提供する 2 つの MCP ツールの仕様を定義します。

## `web_search` ツール

DuckDuckGo HTML Lite (`https://html.duckduckgo.com/html/`) をスクレイピングして Web 検索結果を返すツールです。

### パラメータ

| パラメータ | 型 | 必須 | デフォルト | 説明 |
|---|---|---|---|---|
| `query` | `string` | はい | - | 検索キーワード |
| `max_results` | `integer` | いいえ | `10` | 返却する検索結果の最大数（1〜30） |

### 入力スキーマ (JSON Schema)

```json
{
  "type": "object",
  "properties": {
    "query": {
      "type": "string",
      "description": "検索キーワード"
    },
    "max_results": {
      "type": "integer",
      "description": "返却する検索結果の最大数（デフォルト: 10）",
      "default": 10,
      "minimum": 1,
      "maximum": 30
    }
  },
  "required": ["query"]
}
```

### 戻り値

検索結果を Markdown 形式で整形したテキストを `CallToolResult` の `content` として返します。

#### 成功時の出力フォーマット

```markdown
## Web Search Results for "Rust programming"

### 1. The Rust Programming Language
**URL:** https://www.rust-lang.org/
Rust is a systems programming language focused on safety, speed, and concurrency.

---

### 2. Rust (programming language) - Wikipedia
**URL:** https://en.wikipedia.org/wiki/Rust_(programming_language)
Rust is a multi-paradigm, general-purpose programming language...

---

_Source: DuckDuckGo (3 results)_
```

#### 結果なしの場合

```markdown
## Web Search Results for "xyzzy12345noresult"

No results found.

_Source: DuckDuckGo_
```

### エラー時

`CallToolResult` の `is_error` フラグを `true` に設定し、エラーメッセージをテキストとして返します。

---

## `instant_answer` ツール

DuckDuckGo Instant Answer API (`https://api.duckduckgo.com/?format=json`) を使用して、クエリに対する即時回答を返すツールです。

### パラメータ

| パラメータ | 型 | 必須 | デフォルト | 説明 |
|---|---|---|---|---|
| `query` | `string` | はい | - | 検索キーワード |

### 入力スキーマ (JSON Schema)

```json
{
  "type": "object",
  "properties": {
    "query": {
      "type": "string",
      "description": "検索キーワード"
    }
  },
  "required": ["query"]
}
```

### 戻り値

Instant Answer API のレスポンスを Markdown 形式で整形したテキストを返します。

#### 成功時の出力フォーマット

```markdown
## Instant Answer for "Rust programming language"

### Abstract
Rust is a multi-paradigm, general-purpose programming language
that emphasizes performance, type safety, and concurrency.

**Source:** Wikipedia
**URL:** https://en.wikipedia.org/wiki/Rust_(programming_language)

### Related Topics
- **Cargo** - The Rust package manager
- **Crates.io** - The Rust community's crate registry

_Source: DuckDuckGo Instant Answer API_
```

#### 情報なしの場合

Instant Answer API が該当する情報を返さない場合（`Abstract` が空で `RelatedTopics` も空の場合）：

```markdown
## Instant Answer for "xyzzy12345noresult"

No instant answer available for this query.

_Source: DuckDuckGo Instant Answer API_
```

### エラー時

`CallToolResult` の `is_error` フラグを `true` に設定し、エラーメッセージをテキストとして返します。

---

## `tools/list` レスポンス

MCP の `tools/list` リクエストに対して、以下の 2 ツールを返します。

```json
{
  "tools": [
    {
      "name": "web_search",
      "description": "Search the web using DuckDuckGo and return a list of results with titles, URLs, and snippets.",
      "inputSchema": {
        "type": "object",
        "properties": {
          "query": {
            "type": "string",
            "description": "検索キーワード"
          },
          "max_results": {
            "type": "integer",
            "description": "返却する検索結果の最大数（デフォルト: 10）",
            "default": 10,
            "minimum": 1,
            "maximum": 30
          }
        },
        "required": ["query"]
      }
    },
    {
      "name": "instant_answer",
      "description": "Get an instant answer from DuckDuckGo, including abstracts, definitions, and related topics.",
      "inputSchema": {
        "type": "object",
        "properties": {
          "query": {
            "type": "string",
            "description": "検索キーワード"
          }
        },
        "required": ["query"]
      }
    }
  ]
}
```
