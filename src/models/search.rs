#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

pub fn format_results_markdown(query: &str, results: &[SearchResult]) -> String {
    let mut output = format!("## Web Search Results for \"{query}\"\n\n");
    if results.is_empty() {
        output.push_str("No results found.\n\n_Source: DuckDuckGo_");
        return output;
    }
    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!("### {}. {}\n", i + 1, result.title));
        output.push_str(&format!("**URL:** {}\n", result.url));
        output.push_str(&format!("{}\n\n---\n\n", result.snippet));
    }
    output.push_str(&format!("_Source: DuckDuckGo ({} results)_", results.len()));
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult {
            title: "Rust Programming".to_string(),
            url: "https://www.rust-lang.org/".to_string(),
            snippet: "A systems programming language".to_string(),
        };
        assert_eq!(result.title, "Rust Programming");
        assert_eq!(result.url, "https://www.rust-lang.org/");
        assert_eq!(result.snippet, "A systems programming language");
    }

    #[test]
    fn test_format_results_markdown() {
        let results = vec![
            SearchResult {
                title: "The Rust Programming Language".to_string(),
                url: "https://www.rust-lang.org/".to_string(),
                snippet: "Rust is a systems programming language.".to_string(),
            },
            SearchResult {
                title: "Rust - Wikipedia".to_string(),
                url: "https://en.wikipedia.org/wiki/Rust".to_string(),
                snippet: "Rust is a multi-paradigm language.".to_string(),
            },
        ];
        let output = format_results_markdown("Rust programming", &results);
        assert!(output.contains("## Web Search Results for \"Rust programming\""));
        assert!(output.contains("### 1. The Rust Programming Language"));
        assert!(output.contains("**URL:** https://www.rust-lang.org/"));
        assert!(output.contains("### 2. Rust - Wikipedia"));
        assert!(output.contains("_Source: DuckDuckGo (2 results)_"));
    }

    #[test]
    fn test_format_results_markdown_empty() {
        let results: Vec<SearchResult> = vec![];
        let output = format_results_markdown("no results query", &results);
        assert!(output.contains("No results found."));
        assert!(output.contains("_Source: DuckDuckGo_"));
    }
}
