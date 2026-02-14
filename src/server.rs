use std::sync::Arc;
use std::time::Duration;

use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo};
use rmcp::schemars;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use serde::Deserialize;

use crate::cache::TtlCache;
use crate::config::Config;
use crate::rate_limiter::RateLimiter;
use crate::retry::retry_with_backoff;
use crate::tools::instant_answer::execute_instant_answer;
use crate::tools::web_search::execute_web_search;

const DUCKDUCKGO_HTML_BASE_URL: &str = "https://html.duckduckgo.com";
const DUCKDUCKGO_API_BASE_URL: &str = "https://api.duckduckgo.com";

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WebSearchParams {
    pub query: String,
    pub max_results: Option<usize>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InstantAnswerParams {
    pub query: String,
}

#[derive(Debug, Clone)]
pub struct Server {
    client: reqwest::Client,
    config: Config,
    html_base_url: String,
    api_base_url: String,
    cache: Arc<TtlCache>,
    rate_limiter: Arc<RateLimiter>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Server {
    pub fn new(client: reqwest::Client, config: Config) -> Self {
        let cache = Arc::new(TtlCache::new(Duration::from_secs(config.cache_ttl_secs)));
        let rate_limiter = Arc::new(RateLimiter::new(Duration::from_millis(
            config.rate_limit_ms,
        )));
        Self {
            client,
            config,
            html_base_url: DUCKDUCKGO_HTML_BASE_URL.to_string(),
            api_base_url: DUCKDUCKGO_API_BASE_URL.to_string(),
            cache,
            rate_limiter,
            tool_router: Self::tool_router(),
        }
    }

    pub fn with_base_urls(
        client: reqwest::Client,
        config: Config,
        html_base_url: String,
        api_base_url: String,
    ) -> Self {
        let cache = Arc::new(TtlCache::new(Duration::from_secs(config.cache_ttl_secs)));
        let rate_limiter = Arc::new(RateLimiter::new(Duration::from_millis(
            config.rate_limit_ms,
        )));
        Self {
            client,
            config,
            html_base_url,
            api_base_url,
            cache,
            rate_limiter,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search the web using DuckDuckGo and return results as Markdown")]
    async fn web_search(
        &self,
        params: Parameters<WebSearchParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let max_results = params.0.max_results.unwrap_or(self.config.max_results);
        let cache_key = format!("web_search:{}:{}", params.0.query, max_results);

        if let Some(cached) = self.cache.get(&cache_key).await {
            return Ok(CallToolResult::success(vec![Content::text(cached)]));
        }

        self.rate_limiter.acquire().await;

        let client = self.client.clone();
        let html_base_url = self.html_base_url.clone();
        let query = params.0.query.clone();
        let timeout_secs = self.config.timeout_secs;
        let max_retries = self.config.max_retries;

        let result = retry_with_backoff(max_retries, || {
            let client = client.clone();
            let html_base_url = html_base_url.clone();
            let query = query.clone();
            async move {
                execute_web_search(&client, &html_base_url, &query, max_results, timeout_secs).await
            }
        })
        .await;

        Ok(match result {
            Ok(markdown) => {
                self.cache.set(cache_key, markdown.clone()).await;
                CallToolResult::success(vec![Content::text(markdown)])
            }
            Err(e) => e.to_tool_result(),
        })
    }

    #[tool(description = "Get an instant answer from DuckDuckGo for a given query")]
    async fn instant_answer(
        &self,
        params: Parameters<InstantAnswerParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        let cache_key = format!("instant_answer:{}", params.0.query);

        if let Some(cached) = self.cache.get(&cache_key).await {
            return Ok(CallToolResult::success(vec![Content::text(cached)]));
        }

        self.rate_limiter.acquire().await;

        let client = self.client.clone();
        let api_base_url = self.api_base_url.clone();
        let query = params.0.query.clone();
        let timeout_secs = self.config.timeout_secs;
        let max_retries = self.config.max_retries;

        let result = retry_with_backoff(max_retries, || {
            let client = client.clone();
            let api_base_url = api_base_url.clone();
            let query = query.clone();
            async move { execute_instant_answer(&client, &api_base_url, &query, timeout_secs).await }
        })
        .await;

        Ok(match result {
            Ok(markdown) => {
                self.cache.set(cache_key, markdown.clone()).await;
                CallToolResult::success(vec![Content::text(markdown)])
            }
            Err(e) => e.to_tool_result(),
        })
    }
}

#[tool_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "websearch-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                ..Default::default()
            },
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client::build_http_client;

    fn create_test_server() -> Server {
        let config = Config {
            max_results: 10,
            timeout_secs: 10,
            user_agent: "test-agent".to_string(),
            cache_ttl_secs: 300,
            rate_limit_ms: 1000,
            max_retries: 3,
        };
        let client = build_http_client(&config).unwrap();
        Server::new(client, config)
    }

    #[test]
    fn test_tool_router_has_two_tools() {
        let server = create_test_server();
        assert_eq!(server.tool_router.list_all().len(), 2);
    }

    #[test]
    fn test_tool_router_has_correct_names() {
        let server = create_test_server();
        let tools = server.tool_router.list_all();
        let names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
        assert!(names.contains(&"web_search".to_string()));
        assert!(names.contains(&"instant_answer".to_string()));
    }

    #[test]
    fn test_server_info_name() {
        let server = create_test_server();
        let info = server.get_info();
        assert_eq!(info.server_info.name, "websearch-mcp");
    }

    #[test]
    fn test_server_info_has_tools_capability() {
        let server = create_test_server();
        let info = server.get_info();
        assert!(info.capabilities.tools.is_some());
    }
}
