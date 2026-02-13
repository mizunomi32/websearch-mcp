use scraper::{Html, Selector};

use crate::error::WebSearchError;
use crate::models::search::{format_results_markdown, SearchResult};

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

pub async fn execute_web_search(
    client: &reqwest::Client,
    base_url: &str,
    query: &str,
    max_results: usize,
    timeout_secs: u64,
) -> Result<String, WebSearchError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_extracts_all_fields() {
        let html = include_str!("../../tests/fixtures/search_results.html");
        let results = parse_html_results(html, 10);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].title, "The Rust Programming Language");
        assert_eq!(results[0].url, "https://www.rust-lang.org/");
        assert!(results[0].snippet.contains("reliable and efficient software"));
    }

    #[test]
    fn test_parse_multiple_results() {
        let html = include_str!("../../tests/fixtures/search_results.html");
        let results = parse_html_results(html, 10);
        assert_eq!(results[1].title, "Rust (programming language) - Wikipedia");
        assert_eq!(results[2].title, "The Rust Programming Language - Rust Book");
    }

    #[test]
    fn test_parse_respects_max_results() {
        let html = include_str!("../../tests/fixtures/search_results.html");
        let results = parse_html_results(html, 2);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_parse_empty_html() {
        let results = parse_html_results("<html><body></body></html>", 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_empty_search_fixture() {
        let html = include_str!("../../tests/fixtures/search_results_empty.html");
        let results = parse_html_results(html, 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_trims_whitespace() {
        let html = r#"
        <div class="result">
          <h2 class="result__title">
            <a class="result__a" href="https://example.com">  Spaced Title  </a>
          </h2>
          <a class="result__snippet" href="https://example.com">  Spaced snippet  </a>
        </div>
        "#;
        let results = parse_html_results(html, 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Spaced Title");
        assert_eq!(results[0].snippet, "Spaced snippet");
    }

    #[test]
    fn test_parse_missing_snippet_uses_empty_string() {
        let html = r#"
        <div class="result">
          <h2 class="result__title">
            <a class="result__a" href="https://example.com">Title Only</a>
          </h2>
        </div>
        "#;
        let results = parse_html_results(html, 10);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Title Only");
        assert_eq!(results[0].snippet, "");
    }

    #[test]
    fn test_parse_missing_href_skips_result() {
        let html = r#"
        <div class="result">
          <h2 class="result__title">
            <a class="result__a">No Href</a>
          </h2>
        </div>
        "#;
        let results = parse_html_results(html, 10);
        assert!(results.is_empty());
    }

    #[test]
    fn test_parse_max_results_zero() {
        let html = include_str!("../../tests/fixtures/search_results.html");
        let results = parse_html_results(html, 0);
        assert!(results.is_empty());
    }
}
