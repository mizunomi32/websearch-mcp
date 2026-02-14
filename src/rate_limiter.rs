use std::time::{Duration, Instant};

use tokio::sync::Mutex;

pub struct RateLimiter {
    min_interval: Duration,
    last_request: Mutex<Option<Instant>>,
}

impl std::fmt::Debug for RateLimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RateLimiter")
            .field("min_interval", &self.min_interval)
            .finish()
    }
}

impl RateLimiter {
    pub fn new(min_interval: Duration) -> Self {
        Self {
            min_interval,
            last_request: Mutex::new(None),
        }
    }

    pub async fn acquire(&self) {
        let mut last = self.last_request.lock().await;
        if let Some(last_time) = *last {
            let elapsed = last_time.elapsed();
            if elapsed < self.min_interval {
                let wait = self.min_interval - elapsed;
                drop(last);
                tokio::time::sleep(wait).await;
                let mut last = self.last_request.lock().await;
                *last = Some(Instant::now());
                return;
            }
        }
        *last = Some(Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_first_acquire_returns_immediately() {
        let limiter = RateLimiter::new(Duration::from_millis(100));
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_second_acquire_waits_for_interval() {
        let limiter = RateLimiter::new(Duration::from_millis(200));
        limiter.acquire().await;
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() >= Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test_acquire_after_interval_returns_immediately() {
        let limiter = RateLimiter::new(Duration::from_millis(50));
        limiter.acquire().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn test_zero_interval_no_delay() {
        let limiter = RateLimiter::new(Duration::from_millis(0));
        limiter.acquire().await;
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() < Duration::from_millis(50));
    }
}
