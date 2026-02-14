use std::collections::HashMap;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

pub struct TtlCache {
    entries: Mutex<HashMap<String, (String, Instant)>>,
    ttl: Duration,
}

impl std::fmt::Debug for TtlCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TtlCache").field("ttl", &self.ttl).finish()
    }
}

impl TtlCache {
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            ttl,
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let entries = self.entries.lock().await;
        let (value, created_at) = entries.get(key)?;
        if created_at.elapsed() < self.ttl {
            Some(value.clone())
        } else {
            None
        }
    }

    pub async fn set(&self, key: String, value: String) {
        let mut entries = self.entries.lock().await;
        entries.retain(|_, (_, created_at)| created_at.elapsed() < self.ttl);
        entries.insert(key, (value, Instant::now()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_returns_none_for_missing_key() {
        let cache = TtlCache::new(Duration::from_secs(60));
        assert!(cache.get("missing").await.is_none());
    }

    #[tokio::test]
    async fn test_set_and_get_returns_value() {
        let cache = TtlCache::new(Duration::from_secs(60));
        cache.set("key1".to_string(), "value1".to_string()).await;
        assert_eq!(cache.get("key1").await, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_expired_entry_returns_none() {
        let cache = TtlCache::new(Duration::from_millis(50));
        cache.set("key1".to_string(), "value1".to_string()).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(cache.get("key1").await.is_none());
    }

    #[tokio::test]
    async fn test_non_expired_entry_returns_value() {
        let cache = TtlCache::new(Duration::from_secs(60));
        cache.set("key1".to_string(), "value1".to_string()).await;
        tokio::time::sleep(Duration::from_millis(10)).await;
        assert_eq!(cache.get("key1").await, Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_set_overwrites_existing_value() {
        let cache = TtlCache::new(Duration::from_secs(60));
        cache.set("key1".to_string(), "old".to_string()).await;
        cache.set("key1".to_string(), "new".to_string()).await;
        assert_eq!(cache.get("key1").await, Some("new".to_string()));
    }

    #[tokio::test]
    async fn test_set_cleans_up_expired_entries() {
        let cache = TtlCache::new(Duration::from_millis(50));
        cache.set("old".to_string(), "value".to_string()).await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        cache.set("new".to_string(), "value".to_string()).await;
        let entries = cache.entries.lock().await;
        assert!(!entries.contains_key("old"));
        assert!(entries.contains_key("new"));
    }

    #[tokio::test]
    async fn test_zero_ttl_always_misses() {
        let cache = TtlCache::new(Duration::from_secs(0));
        cache.set("key1".to_string(), "value1".to_string()).await;
        assert!(cache.get("key1").await.is_none());
    }
}
