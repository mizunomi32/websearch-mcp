use std::time::Duration;

use reqwest::Client;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use websearch_mcp::tools::web_search::execute_web_search;

fn build_test_client(timeout_secs: u64) -> Client {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_web_search_returns_formatted_results() {
    let server = MockServer::start().await;
    let html = include_str!("fixtures/search_results.html");

    Mock::given(method("GET"))
        .and(path("/html/"))
        .and(query_param("q", "rust programming"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&server)
        .await;

    let client = build_test_client(10);
    let result = execute_web_search(&client, &server.uri(), "rust programming", 10, 10)
        .await
        .unwrap();

    assert!(result.contains("Web Search Results for \"rust programming\""));
    assert!(result.contains("The Rust Programming Language"));
    assert!(result.contains("https://www.rust-lang.org/"));
    assert!(result.contains("reliable and efficient software"));
}

#[tokio::test]
async fn test_web_search_handles_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/html/"))
        .and(query_param("q", "test"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let client = build_test_client(10);
    let result = execute_web_search(&client, &server.uri(), "test", 10, 10).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        websearch_mcp::error::WebSearchError::HttpError(_)
    ));
}

#[tokio::test]
async fn test_web_search_handles_timeout() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/html/"))
        .and(query_param("q", "slow query"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
        .mount(&server)
        .await;

    let client = build_test_client(1);
    let result = execute_web_search(&client, &server.uri(), "slow query", 10, 1).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        websearch_mcp::error::WebSearchError::Timeout(1)
    ));
}

#[tokio::test]
async fn test_web_search_rejects_empty_query() {
    let server = MockServer::start().await;
    let client = build_test_client(10);
    let result = execute_web_search(&client, &server.uri(), "", 10, 10).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        websearch_mcp::error::WebSearchError::EmptyQuery
    ));
}

#[tokio::test]
async fn test_web_search_handles_empty_results() {
    let server = MockServer::start().await;
    let html = include_str!("fixtures/search_results_empty.html");

    Mock::given(method("GET"))
        .and(path("/html/"))
        .and(query_param("q", "xyzzy12345noresult"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&server)
        .await;

    let client = build_test_client(10);
    let result = execute_web_search(&client, &server.uri(), "xyzzy12345noresult", 10, 10)
        .await
        .unwrap();

    assert!(result.contains("No results found."));
}
