use std::time::Duration;

use reqwest::Client;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use websearch_mcp::tools::instant_answer::execute_instant_answer;

fn build_test_client(timeout_secs: u64) -> Client {
    Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_instant_answer_returns_formatted_response() {
    let server = MockServer::start().await;
    let json = include_str!("fixtures/instant_answer.json");

    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("q", "rust programming"))
        .and(query_param("format", "json"))
        .and(query_param("no_html", "1"))
        .and(query_param("skip_disambig", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json))
        .mount(&server)
        .await;

    let client = build_test_client(10);
    let result = execute_instant_answer(&client, &server.uri(), "rust programming", 10)
        .await
        .unwrap();

    assert!(result.contains("Instant Answer for \"rust programming\""));
    assert!(result.contains("performance, type safety, and concurrency"));
    assert!(result.contains("Wikipedia"));
    assert!(result.contains("Source: DuckDuckGo Instant Answer API"));
}

#[tokio::test]
async fn test_instant_answer_handles_server_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("q", "test"))
        .and(query_param("format", "json"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let client = build_test_client(10);
    let result = execute_instant_answer(&client, &server.uri(), "test", 10).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        websearch_mcp::error::WebSearchError::HttpError(_)
    ));
}

#[tokio::test]
async fn test_instant_answer_handles_timeout() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("q", "slow query"))
        .and(query_param("format", "json"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(10)))
        .mount(&server)
        .await;

    let client = build_test_client(1);
    let result = execute_instant_answer(&client, &server.uri(), "slow query", 1).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        websearch_mcp::error::WebSearchError::Timeout(1)
    ));
}

#[tokio::test]
async fn test_instant_answer_rejects_empty_query() {
    let server = MockServer::start().await;
    let client = build_test_client(10);
    let result = execute_instant_answer(&client, &server.uri(), "", 10).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(
        err,
        websearch_mcp::error::WebSearchError::EmptyQuery
    ));
}

#[tokio::test]
async fn test_instant_answer_handles_empty_response() {
    let server = MockServer::start().await;
    let json = include_str!("fixtures/instant_answer_empty.json");

    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("q", "xyzzy12345noresult"))
        .and(query_param("format", "json"))
        .and(query_param("no_html", "1"))
        .and(query_param("skip_disambig", "1"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json))
        .mount(&server)
        .await;

    let client = build_test_client(10);
    let result = execute_instant_answer(&client, &server.uri(), "xyzzy12345noresult", 10)
        .await
        .unwrap();

    assert!(result.contains("No instant answer available"));
}
