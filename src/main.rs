use rmcp::transport::stdio;
use rmcp::ServiceExt;
use tracing_subscriber::EnvFilter;
use websearch_mcp::config::Config;
use websearch_mcp::http_client::build_http_client;
use websearch_mcp::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env();
    let client = build_http_client(&config)?;
    let server = Server::new(client, config);

    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
