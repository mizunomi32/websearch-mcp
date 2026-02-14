#[derive(Debug, Clone)]
pub struct Config {
    pub max_results: usize,
    pub timeout_secs: u64,
    pub user_agent: String,
    pub cache_ttl_secs: u64,
    pub rate_limit_ms: u64,
    pub max_retries: u32,
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
            cache_ttl_secs: std::env::var("WEBSEARCH_CACHE_TTL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300),
            rate_limit_ms: std::env::var("WEBSEARCH_RATE_LIMIT_MS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            max_retries: std::env::var("WEBSEARCH_MAX_RETRIES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3),
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

    #[test]
    fn test_default_cache_ttl_secs() {
        std::env::remove_var("WEBSEARCH_CACHE_TTL_SECS");
        let config = Config::from_env();
        assert_eq!(config.cache_ttl_secs, 300);
    }

    #[test]
    fn test_default_rate_limit_ms() {
        std::env::remove_var("WEBSEARCH_RATE_LIMIT_MS");
        let config = Config::from_env();
        assert_eq!(config.rate_limit_ms, 1000);
    }

    #[test]
    fn test_default_max_retries() {
        std::env::remove_var("WEBSEARCH_MAX_RETRIES");
        let config = Config::from_env();
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_custom_cache_ttl_secs() {
        std::env::set_var("WEBSEARCH_CACHE_TTL_SECS", "600");
        let config = Config::from_env();
        assert_eq!(config.cache_ttl_secs, 600);
        std::env::remove_var("WEBSEARCH_CACHE_TTL_SECS");
    }

    #[test]
    fn test_custom_rate_limit_ms() {
        std::env::set_var("WEBSEARCH_RATE_LIMIT_MS", "2000");
        let config = Config::from_env();
        assert_eq!(config.rate_limit_ms, 2000);
        std::env::remove_var("WEBSEARCH_RATE_LIMIT_MS");
    }

    #[test]
    fn test_custom_max_retries() {
        std::env::set_var("WEBSEARCH_MAX_RETRIES", "5");
        let config = Config::from_env();
        assert_eq!(config.max_retries, 5);
        std::env::remove_var("WEBSEARCH_MAX_RETRIES");
    }

    #[test]
    fn test_invalid_cache_ttl_uses_default() {
        std::env::set_var("WEBSEARCH_CACHE_TTL_SECS", "abc");
        let config = Config::from_env();
        assert_eq!(config.cache_ttl_secs, 300);
        std::env::remove_var("WEBSEARCH_CACHE_TTL_SECS");
    }

    #[test]
    fn test_invalid_rate_limit_uses_default() {
        std::env::set_var("WEBSEARCH_RATE_LIMIT_MS", "abc");
        let config = Config::from_env();
        assert_eq!(config.rate_limit_ms, 1000);
        std::env::remove_var("WEBSEARCH_RATE_LIMIT_MS");
    }

    #[test]
    fn test_invalid_max_retries_uses_default() {
        std::env::set_var("WEBSEARCH_MAX_RETRIES", "abc");
        let config = Config::from_env();
        assert_eq!(config.max_retries, 3);
        std::env::remove_var("WEBSEARCH_MAX_RETRIES");
    }
}
