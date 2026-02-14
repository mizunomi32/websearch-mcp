use std::future::Future;
use std::time::Duration;

use crate::error::WebSearchError;

pub async fn retry_with_backoff<F, Fut>(max_retries: u32, f: F) -> Result<String, WebSearchError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<String, WebSearchError>>,
{
    let mut last_err = None;
    for attempt in 0..=max_retries {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if !e.is_retryable() || attempt == max_retries {
                    return Err(e);
                }
                let backoff = Duration::from_secs(1 << attempt);
                tokio::time::sleep(backoff).await;
                last_err = Some(e);
            }
        }
    }
    Err(last_err.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_succeeds_on_first_try() {
        let call_count = Arc::new(AtomicU32::new(0));
        let count = call_count.clone();
        let result = retry_with_backoff(3, || {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Ok("success".to_string())
            }
        })
        .await;
        assert_eq!(result.unwrap(), "success");
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn test_retries_on_retryable_error_then_succeeds() {
        let call_count = Arc::new(AtomicU32::new(0));
        let count = call_count.clone();
        let result = retry_with_backoff(3, || {
            let count = count.clone();
            async move {
                let n = count.fetch_add(1, Ordering::SeqCst);
                if n < 2 {
                    Err(WebSearchError::Timeout(10))
                } else {
                    Ok("recovered".to_string())
                }
            }
        })
        .await;
        assert_eq!(result.unwrap(), "recovered");
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test(start_paused = true)]
    async fn test_gives_up_after_max_retries() {
        let call_count = Arc::new(AtomicU32::new(0));
        let count = call_count.clone();
        let result = retry_with_backoff(2, || {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err(WebSearchError::Timeout(10))
            }
        })
        .await;
        assert!(result.is_err());
        // 1 initial + 2 retries = 3 calls
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_does_not_retry_non_retryable_error() {
        let call_count = Arc::new(AtomicU32::new(0));
        let count = call_count.clone();
        let result = retry_with_backoff(3, || {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err(WebSearchError::EmptyQuery)
            }
        })
        .await;
        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_zero_retries_only_tries_once() {
        let call_count = Arc::new(AtomicU32::new(0));
        let count = call_count.clone();
        let result = retry_with_backoff(0, || {
            let count = count.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err(WebSearchError::Timeout(10))
            }
        })
        .await;
        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }
}
