use rmcp::model::{CallToolResult, Content};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebSearchError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Failed to parse HTML response: {0}")]
    HtmlParseError(String),
    #[error("Failed to parse JSON response: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Query must not be empty")]
    EmptyQuery,
    #[error("Request timed out after {0} seconds")]
    Timeout(u64),
    #[error("No results found for query: {0}")]
    NoResults(String),
}

impl WebSearchError {
    pub fn user_message(&self) -> &str {
        match self {
            Self::HttpError(_) => "Failed to fetch search results. Please try again later.",
            Self::HtmlParseError(_) => {
                "Failed to parse search results. The page structure may have changed."
            }
            Self::JsonParseError(_) => "Failed to parse API response.",
            Self::EmptyQuery => "Query must not be empty.",
            Self::Timeout(_) => "Request timed out. Please try again.",
            Self::NoResults(_) => "No results found.",
        }
    }

    pub fn to_tool_result(&self) -> CallToolResult {
        match self {
            Self::NoResults(_) => CallToolResult::success(vec![Content::text(self.user_message())]),
            _ => CallToolResult::error(vec![Content::text(self.user_message())]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_message_http_error() {
        let err = WebSearchError::HtmlParseError("test".to_string());
        assert_eq!(
            err.user_message(),
            "Failed to parse search results. The page structure may have changed."
        );
    }

    #[test]
    fn test_user_message_empty_query() {
        let err = WebSearchError::EmptyQuery;
        assert_eq!(err.user_message(), "Query must not be empty.");
    }

    #[test]
    fn test_user_message_timeout() {
        let err = WebSearchError::Timeout(10);
        assert_eq!(err.user_message(), "Request timed out. Please try again.");
    }

    #[test]
    fn test_user_message_no_results() {
        let err = WebSearchError::NoResults("test".to_string());
        assert_eq!(err.user_message(), "No results found.");
    }

    #[test]
    fn test_to_tool_result_no_results_is_not_error() {
        let err = WebSearchError::NoResults("test".to_string());
        let result = err.to_tool_result();
        assert_eq!(result.is_error, Some(false));
    }

    #[test]
    fn test_to_tool_result_empty_query_is_error() {
        let err = WebSearchError::EmptyQuery;
        let result = err.to_tool_result();
        assert_eq!(result.is_error, Some(true));
    }
}
