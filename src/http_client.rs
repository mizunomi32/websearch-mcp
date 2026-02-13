use reqwest::Client;
use std::time::Duration;

use crate::config::Config;

pub fn build_http_client(config: &Config) -> Result<Client, reqwest::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_http_client_succeeds() {
        let config = Config {
            max_results: 10,
            timeout_secs: 10,
            user_agent: "websearch-mcp/0.1".to_string(),
        };
        assert!(build_http_client(&config).is_ok());
    }

    #[test]
    fn test_build_http_client_custom_config() {
        let config = Config {
            max_results: 5,
            timeout_secs: 30,
            user_agent: "custom-agent/2.0".to_string(),
        };
        assert!(build_http_client(&config).is_ok());
    }
}
