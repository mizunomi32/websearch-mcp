use rmcp::model::CallToolRequestParams;
use rmcp::service::RunningService;
use rmcp::ServiceExt;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use websearch_mcp::config::Config;
use websearch_mcp::http_client::build_http_client;
use websearch_mcp::server::Server;

async fn setup_e2e(
    html_mock: &MockServer,
    api_mock: &MockServer,
) -> RunningService<rmcp::RoleClient, impl rmcp::Service<rmcp::RoleClient>> {
    let config = Config {
        max_results: 10,
        timeout_secs: 10,
        user_agent: "test-agent".to_string(),
    };
    let client = build_http_client(&config).unwrap();
    let server = Server::with_base_urls(client, config, html_mock.uri(), api_mock.uri());

    let (server_transport, client_transport) = tokio::io::duplex(4096);
    tokio::spawn(async move {
        let svc = server.serve(server_transport).await.unwrap();
        svc.waiting().await.unwrap();
    });

    ().serve(client_transport).await.unwrap()
}

#[tokio::test]
async fn test_e2e_list_tools_returns_two_tools() {
    let html_mock = MockServer::start().await;
    let api_mock = MockServer::start().await;
    let client = setup_e2e(&html_mock, &api_mock).await;

    let tools = client.list_all_tools().await.unwrap();
    assert_eq!(tools.len(), 2);

    let names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
    assert!(names.contains(&"web_search".to_string()));
    assert!(names.contains(&"instant_answer".to_string()));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn test_e2e_web_search_success() {
    let html_mock = MockServer::start().await;
    let api_mock = MockServer::start().await;

    let html = include_str!("fixtures/search_results.html");
    Mock::given(method("GET"))
        .and(path("/html/"))
        .and(query_param("q", "rust programming"))
        .respond_with(ResponseTemplate::new(200).set_body_string(html))
        .mount(&html_mock)
        .await;

    let client = setup_e2e(&html_mock, &api_mock).await;

    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "web_search".into(),
            arguments: Some(
                serde_json::json!({
                    "query": "rust programming"
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
            task: None,
        })
        .await
        .unwrap();

    assert_eq!(result.is_error, Some(false));
    let text = result
        .content
        .first()
        .and_then(|c| c.raw.as_text())
        .map(|t| t.text.as_str())
        .expect("Expected text content");
    assert!(text.contains("The Rust Programming Language"));
    assert!(text.contains("https://www.rust-lang.org/"));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn test_e2e_instant_answer_success() {
    let html_mock = MockServer::start().await;
    let api_mock = MockServer::start().await;

    let json = include_str!("fixtures/instant_answer.json");
    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("q", "rust programming"))
        .and(query_param("format", "json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(json))
        .mount(&api_mock)
        .await;

    let client = setup_e2e(&html_mock, &api_mock).await;

    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "instant_answer".into(),
            arguments: Some(
                serde_json::json!({
                    "query": "rust programming"
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
            task: None,
        })
        .await
        .unwrap();

    assert_eq!(result.is_error, Some(false));
    let text = result
        .content
        .first()
        .and_then(|c| c.raw.as_text())
        .map(|t| t.text.as_str())
        .expect("Expected text content");
    assert!(text.contains("Instant Answer"));
    assert!(text.contains("Source: DuckDuckGo Instant Answer API"));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn test_e2e_web_search_empty_query_returns_error() {
    let html_mock = MockServer::start().await;
    let api_mock = MockServer::start().await;
    let client = setup_e2e(&html_mock, &api_mock).await;

    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "web_search".into(),
            arguments: Some(
                serde_json::json!({
                    "query": ""
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
            task: None,
        })
        .await
        .unwrap();

    assert_eq!(result.is_error, Some(true));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn test_e2e_unknown_tool_returns_error() {
    let html_mock = MockServer::start().await;
    let api_mock = MockServer::start().await;
    let client = setup_e2e(&html_mock, &api_mock).await;

    let result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "nonexistent".into(),
            arguments: None,
            task: None,
        })
        .await;

    assert!(result.is_err());

    client.cancel().await.unwrap();
}
