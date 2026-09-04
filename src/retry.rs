use std::time::{Duration, SystemTime};

/// Configuration for retry behavior when requests encounter rate limiting (HTTP 429)
///
/// RetryPolicy controls how the client handles rate limit responses, including
/// exponential backoff, maximum retry attempts, and respect for server-provided
/// `Retry-After` headers.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts before giving up
    pub max_retries: u32,

    /// Initial delay before the first retry
    pub base_delay: Duration,

    /// Maximum delay between retries (caps exponential backoff)
    pub max_delay: Duration,

    /// Whether to respect the `Retry-After` header from the server
    ///
    /// When true, the client will use the server-provided delay instead of
    /// the calculated exponential backoff. Highly recommended to keep enabled.
    pub respect_retry_after: bool,

    /// Add random jitter to retry delays to prevent thundering herd
    ///
    /// When enabled, adds up to 25% random variation to retry delays to avoid
    /// multiple clients retrying simultaneously after a rate limit.
    pub use_jitter: bool,
}

impl RetryPolicy {
    /// Conservative retry policy suitable for production environments
    ///
    /// - **Max retries**: 3 attempts
    /// - **Base delay**: 1 second
    /// - **Max delay**: 30 seconds
    /// - **Respects** `Retry-After` headers from server
    /// - **Includes** jitter to prevent thundering herd
    ///
    /// Use this when you want to gracefully handle occasional rate limits
    /// without overwhelming the server with retry attempts.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::{FleetClient, RetryPolicy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_retry_policy(RetryPolicy::conservative())
    ///     .with_token("token")?
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn conservative() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
            respect_retry_after: true,
            use_jitter: true,
        }
    }

    /// Aggressive retry policy for testing or high-availability scenarios
    ///
    /// - **Max retries**: 5 attempts
    /// - **Base delay**: 500ms
    /// - **Max delay**: 60 seconds
    /// - **Respects** `Retry-After` headers from server
    /// - **Includes** jitter to prevent thundering herd
    ///
    /// Use this in integration tests or when you need to be more persistent
    /// about completing requests despite rate limiting. The faster base delay
    /// and higher retry count make this more suitable for automated testing.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::{FleetClient, RetryPolicy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_retry_policy(RetryPolicy::aggressive())
    ///     .with_token("token")?
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn aggressive() -> Self {
        Self {
            max_retries: 5,
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(60),
            respect_retry_after: true,
            use_jitter: true,
        }
    }

    /// Create a custom retry policy with no retries (fail immediately on rate limit)
    ///
    /// Use this when you want to handle rate limiting yourself or when retries
    /// are not appropriate for your use case.
    ///
    /// # Example
    /// ```no_run
    /// # use fleetdm_api_client::{FleetClient, RetryPolicy};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FleetClient::builder("https://fleet.example.com")?
    ///     .with_retry_policy(RetryPolicy::none())
    ///     .with_token("token")?
    ///     .build();
    /// # Ok(())
    /// # }
    /// ```
    pub fn none() -> Self {
        Self {
            max_retries: 0,
            base_delay: Duration::from_secs(0),
            max_delay: Duration::from_secs(0),
            respect_retry_after: false,
            use_jitter: false,
        }
    }

    /// Calculate the delay for a given retry attempt
    ///
    /// This implements exponential backoff with optional jitter:
    /// - Delay = base_delay * 2^attempt_number
    /// - Capped at max_delay
    /// - Optionally adds random jitter (0-25% of calculated delay)
    pub(crate) fn calculate_delay(&self, attempt: u32) -> Duration {
        let multiplier = 2u32.checked_pow(attempt).unwrap_or(u32::MAX);
        let mut delay = self.base_delay.saturating_mul(multiplier);

        // Cap at max_delay
        if delay > self.max_delay {
            delay = self.max_delay;
        }

        // Add jitter if enabled (0-25% random variation)
        if self.use_jitter {
            let jitter_factor = fastrand::f64() * 0.25;
            let jitter = delay.as_millis() as f64 * jitter_factor;
            delay = delay.saturating_add(Duration::from_millis(jitter as u64));
        }

        delay.min(self.max_delay)
    }

    /// Parse a Retry-After header value and return the delay duration
    ///
    /// Supports both:
    /// - Delay-seconds format: "120" (wait 120 seconds)
    /// - HTTP-date format: "Wed, 21 Oct 2015 07:28:00 GMT"
    pub(crate) fn parse_retry_after(&self, retry_after: &str) -> Option<Duration> {
        if !self.respect_retry_after {
            return None;
        }

        // Try parsing as seconds
        if let Ok(seconds) = retry_after.parse::<u64>() {
            return Some(Duration::from_secs(seconds).min(self.max_delay));
        }

        let retry_at = httpdate::parse_http_date(retry_after).ok()?;
        Some(
            retry_at
                .duration_since(SystemTime::now())
                .unwrap_or_default()
                .min(self.max_delay),
        )
    }
}

impl Default for RetryPolicy {
    /// Default retry policy is conservative
    fn default() -> Self {
        Self::conservative()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservative_policy() {
        let policy = RetryPolicy::conservative();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.base_delay, Duration::from_secs(1));
        assert_eq!(policy.max_delay, Duration::from_secs(30));
        assert!(policy.respect_retry_after);
        assert!(policy.use_jitter);
    }

    #[test]
    fn test_aggressive_policy() {
        let policy = RetryPolicy::aggressive();
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.base_delay, Duration::from_millis(500));
        assert_eq!(policy.max_delay, Duration::from_secs(60));
        assert!(policy.respect_retry_after);
        assert!(policy.use_jitter);
    }

    #[test]
    fn test_none_policy() {
        let policy = RetryPolicy::none();
        assert_eq!(policy.max_retries, 0);
        assert!(!policy.respect_retry_after);
    }

    #[test]
    fn parses_http_date_retry_after() {
        let policy = RetryPolicy {
            max_delay: Duration::from_secs(30),
            ..RetryPolicy::conservative()
        };
        let retry_at = SystemTime::now() + Duration::from_secs(5);
        let delay = policy
            .parse_retry_after(&httpdate::fmt_http_date(retry_at))
            .expect("HTTP-date should parse");

        assert!(delay <= Duration::from_secs(5));
        assert!(delay >= Duration::from_secs(3));
    }

    #[test]
    fn test_exponential_backoff() {
        let mut policy = RetryPolicy::conservative();
        policy.use_jitter = false; // Disable for predictable testing

        assert_eq!(policy.calculate_delay(0), Duration::from_secs(1));
        assert_eq!(policy.calculate_delay(1), Duration::from_secs(2));
        assert_eq!(policy.calculate_delay(2), Duration::from_secs(4));
        assert_eq!(policy.calculate_delay(3), Duration::from_secs(8));
    }

    #[test]
    fn test_max_delay_cap() {
        let mut policy = RetryPolicy::conservative();
        policy.use_jitter = false;

        // After enough retries, should cap at max_delay (30 seconds)
        assert_eq!(policy.calculate_delay(10), Duration::from_secs(30));
    }

    #[test]
    fn test_parse_retry_after_seconds() {
        let policy = RetryPolicy::conservative();
        assert_eq!(
            policy.parse_retry_after("120"),
            Some(Duration::from_secs(30))
        );
    }

    #[test]
    fn test_large_attempt_does_not_overflow() {
        let mut policy = RetryPolicy::conservative();
        policy.use_jitter = false;
        assert_eq!(policy.calculate_delay(u32::MAX), policy.max_delay);
    }

    #[test]
    fn test_parse_retry_after_disabled() {
        let policy = RetryPolicy::none();
        assert_eq!(policy.parse_retry_after("120"), None);
    }
}
