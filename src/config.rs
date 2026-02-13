#[derive(Debug, Clone)]
pub struct Config {
    pub max_results: usize,
    pub timeout_secs: u64,
    pub user_agent: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            max_results: std::env::var("WEBSEARCH_MAX_RESULTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            timeout_secs: std::env::var("WEBSEARCH_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            user_agent: std::env::var("WEBSEARCH_USER_AGENT")
                .unwrap_or_else(|_| "websearch-mcp/0.1".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        std::env::remove_var("WEBSEARCH_MAX_RESULTS");
        std::env::remove_var("WEBSEARCH_TIMEOUT_SECS");
        std::env::remove_var("WEBSEARCH_USER_AGENT");

        let config = Config::from_env();
        assert_eq!(config.max_results, 10);
        assert_eq!(config.timeout_secs, 10);
        assert_eq!(config.user_agent, "websearch-mcp/0.1");
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("WEBSEARCH_MAX_RESULTS", "20");
        std::env::set_var("WEBSEARCH_TIMEOUT_SECS", "30");
        std::env::set_var("WEBSEARCH_USER_AGENT", "custom-agent/1.0");

        let config = Config::from_env();
        assert_eq!(config.max_results, 20);
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.user_agent, "custom-agent/1.0");

        std::env::remove_var("WEBSEARCH_MAX_RESULTS");
        std::env::remove_var("WEBSEARCH_TIMEOUT_SECS");
        std::env::remove_var("WEBSEARCH_USER_AGENT");
    }

    #[test]
    fn test_config_invalid_env_uses_default() {
        std::env::set_var("WEBSEARCH_MAX_RESULTS", "not_a_number");
        let config = Config::from_env();
        assert_eq!(config.max_results, 10);
        std::env::remove_var("WEBSEARCH_MAX_RESULTS");
    }
}
