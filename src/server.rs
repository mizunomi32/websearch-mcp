use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo};
use rmcp::schemars;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use serde::Deserialize;

use crate::config::Config;
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
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Server {
    pub fn new(client: reqwest::Client, config: Config) -> Self {
        Self {
            client,
            config,
            html_base_url: DUCKDUCKGO_HTML_BASE_URL.to_string(),
            api_base_url: DUCKDUCKGO_API_BASE_URL.to_string(),
            tool_router: Self::tool_router(),
        }
    }

    pub fn with_base_urls(
        client: reqwest::Client,
        config: Config,
        html_base_url: String,
        api_base_url: String,
    ) -> Self {
        Self {
            client,
            config,
            html_base_url,
            api_base_url,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search the web using DuckDuckGo and return results as Markdown")]
    async fn web_search(
        &self,
        _params: Parameters<WebSearchParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        todo!()
    }

    #[tool(description = "Get an instant answer from DuckDuckGo for a given query")]
    async fn instant_answer(
        &self,
        _params: Parameters<InstantAnswerParams>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        todo!()
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
