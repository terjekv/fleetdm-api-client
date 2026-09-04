use crate::error::{ApiErrorResponse, FleetError, Result};
use crate::models::common::{FileDownload, FileDownloadStream};
use crate::retry::RetryPolicy;
use std::future::Future;
use tokio::time::sleep;

const MAX_JSON_RESPONSE_SIZE: usize = 8 * 1024 * 1024;
const MAX_IN_MEMORY_FILE_SIZE: usize = 64 * 1024 * 1024;
pub(crate) const MAX_RAW_RESPONSE_SIZE: usize = 64 * 1024 * 1024;

#[cfg(feature = "tracing")]
use tracing::{debug, trace, warn};

pub(crate) fn encode_path_segment(segment: &str) -> String {
    urlencoding::encode(segment).into_owned()
}

pub(crate) fn append_query(path: &str, query: Option<&[(&str, &str)]>) -> String {
    match query {
        Some(query) if !query.is_empty() => {
            let query_string = query
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}={}",
                        urlencoding::encode(key),
                        urlencoding::encode(value)
                    )
                })
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", path, query_string)
        }
        _ => path.to_string(),
    }
}

pub(crate) fn append_query_params(path: &str, params: &[(String, String)]) -> String {
    if params.is_empty() {
        return path.to_string();
    }

    let query_string = params
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                urlencoding::encode(key),
                urlencoding::encode(value)
            )
        })
        .collect::<Vec<_>>()
        .join("&");
    format!("{}?{}", path, query_string)
}

/// Handle API responses with proper error handling and optional retry logic
pub async fn handle_response<T>(response: reqwest::Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    handle_response_internal(response).await
}

/// Handle responses that return raw bytes on success.
pub async fn handle_bytes_response(response: reqwest::Response) -> Result<Vec<u8>> {
    let status = response.status();

    if status.is_success() {
        #[cfg(feature = "tracing")]
        trace!("Successful byte response: {}", status);
        return read_limited_body(response, MAX_IN_MEMORY_FILE_SIZE).await;
    }

    handle_error_response(response).await
}

/// Handle a successful file response without discarding its media metadata.
pub async fn handle_file_response(response: reqwest::Response) -> Result<FileDownload> {
    let status = response.status();
    if !status.is_success() {
        return handle_error_response(response).await;
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let content_disposition = response
        .headers()
        .get(reqwest::header::CONTENT_DISPOSITION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let bytes = read_limited_body(response, MAX_IN_MEMORY_FILE_SIZE).await?;
    Ok(FileDownload::from_parts(
        bytes,
        content_type,
        content_disposition,
    ))
}

/// Validate a streaming download response without buffering its successful body.
pub async fn handle_file_stream_response(
    response: reqwest::Response,
) -> Result<FileDownloadStream> {
    if response.status().is_success() {
        Ok(FileDownloadStream::from_response(response))
    } else {
        handle_error_response(response).await
    }
}

pub(crate) async fn read_limited_body(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| usize::try_from(length).map_or(true, |length| length > limit))
    {
        return Err(FleetError::ResponseTooLarge { limit });
    }

    let capacity = response
        .content_length()
        .and_then(|length| usize::try_from(length).ok())
        .unwrap_or(0)
        .min(limit);
    let mut body = Vec::with_capacity(capacity);
    while let Some(chunk) = response.chunk().await? {
        if body
            .len()
            .checked_add(chunk.len())
            .is_none_or(|length| length > limit)
        {
            return Err(FleetError::ResponseTooLarge { limit });
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

/// Send a prepared request and deserialize its response, applying retry when configured.
pub async fn send_request<T>(
    request: reqwest::RequestBuilder,
    retry_policy: Option<&RetryPolicy>,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let response = send_prepared_request(request, retry_policy).await?;
    handle_response(response).await
}

/// Send a prepared request whose successful response body has no typed meaning.
///
/// Fleet versions differ between empty, `{}`, and informational JSON bodies for
/// several mutation endpoints. Any successful status is accepted and consumed.
pub async fn send_empty_request(
    request: reqwest::RequestBuilder,
    retry_policy: Option<&RetryPolicy>,
) -> Result<()> {
    let response = send_prepared_request(request, retry_policy).await?;
    handle_empty_response(response).await
}

/// Accept a successful response without buffering a semantically empty body.
pub async fn handle_empty_response(response: reqwest::Response) -> Result<()> {
    if response.status().is_success() {
        drop(response);
        Ok(())
    } else {
        handle_error_response(response).await
    }
}

/// Send a prepared request and return the raw response, applying retry when configured.
pub async fn send_prepared_request(
    request: reqwest::RequestBuilder,
    retry_policy: Option<&RetryPolicy>,
) -> Result<reqwest::Response> {
    if let Some(policy) = retry_policy.filter(|policy| policy.max_retries > 0) {
        let Some(template) = request.try_clone() else {
            // Streaming and multipart bodies are not cloneable. Sending once is safer than
            // rejecting a request before any network I/O; callers that can rebuild a body
            // should use `send_rebuildable_request` to retain retry behavior.
            return request.send().await.map_err(Into::into);
        };
        send_with_retry(policy, || async {
            let request = template.try_clone().ok_or_else(|| {
                FleetError::Config("Request body cannot be cloned for retry".into())
            })?;
            request.send().await.map_err(Into::into)
        })
        .await
    } else {
        request.send().await.map_err(Into::into)
    }
}

/// Send a request that can be rebuilt for each retry attempt.
///
/// This is the retry-safe path for multipart and streaming requests, whose
/// `reqwest::RequestBuilder` values cannot be cloned.
pub async fn send_rebuildable_request<F>(
    build_request: F,
    retry_policy: Option<&RetryPolicy>,
) -> Result<reqwest::Response>
where
    F: Fn() -> Result<reqwest::RequestBuilder>,
{
    let send_once = || async { build_request()?.send().await.map_err(FleetError::from) };

    match retry_policy.filter(|policy| policy.max_retries > 0) {
        Some(policy) => send_with_retry(policy, send_once).await,
        None => send_once().await,
    }
}

/// Handle API responses with retry support
pub async fn handle_response_with_retry<T, F, Fut>(
    retry_policy: &RetryPolicy,
    request_fn: F,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
    F: Fn() -> Fut,
    Fut: Future<Output = Result<reqwest::Response>>,
{
    let response = send_with_retry(retry_policy, request_fn).await?;
    handle_response_internal(response).await
}

/// Send a request with retry support and return the first non-rate-limited response.
pub async fn send_with_retry<F, Fut>(
    retry_policy: &RetryPolicy,
    request_fn: F,
) -> Result<reqwest::Response>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<reqwest::Response>>,
{
    let mut attempt = 0;

    loop {
        match request_fn().await {
            Ok(response) => {
                let status = response.status();

                // If not a rate limit error, process normally
                if status.as_u16() != 429 {
                    #[cfg(feature = "tracing")]
                    trace!("Response status: {}", status);
                    return Ok(response);
                }

                // Handle rate limiting with retry logic
                if attempt >= retry_policy.max_retries {
                    // Max retries exhausted, return the error
                    #[cfg(feature = "tracing")]
                    warn!(
                        "Max retries ({}) exhausted for 429 response",
                        retry_policy.max_retries
                    );

                    return handle_error_response(response).await;
                }

                // Calculate delay
                let retry_after = response
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok());

                let delay = if let Some(after) = retry_after {
                    retry_policy
                        .parse_retry_after(after)
                        .unwrap_or_else(|| retry_policy.calculate_delay(attempt))
                } else {
                    retry_policy.calculate_delay(attempt)
                };

                #[cfg(feature = "tracing")]
                debug!(
                    "Rate limited (429). Retry attempt {}/{} after {:?}",
                    attempt + 1,
                    retry_policy.max_retries,
                    delay
                );

                sleep(delay).await;
                attempt += 1;
            }
            Err(e) => {
                #[cfg(feature = "tracing")]
                warn!("Request error: {:?}", e);
                return Err(e);
            }
        }
    }
}

/// Internal response handler that processes the actual response
async fn handle_response_internal<T>(response: reqwest::Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        let bytes = read_limited_body(response, MAX_JSON_RESPONSE_SIZE).await?;
        let data = if bytes.is_empty() {
            serde_json::from_value(serde_json::Value::Null).or_else(|_| {
                serde_json::from_value(serde_json::Value::Object(serde_json::Map::new()))
            })?
        } else {
            serde_json::from_slice(&bytes)?
        };
        #[cfg(feature = "tracing")]
        trace!("Successful response: {}", status);
        return Ok(data);
    }

    handle_error_response(response).await
}

async fn handle_error_response<T>(response: reqwest::Response) -> Result<T> {
    let status = response.status();

    // Handle specific error cases
    match status.as_u16() {
        429 => {
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok())
                .map(String::from);
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            debug!("Rate limit error (429). Retry-After: {:?}", retry_after);
            Err(FleetError::RateLimit {
                retry_after,
                response: Some(Box::new(error)),
            })
        }
        404 => {
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            debug!("Not found error (404): {}", error.message);
            Err(FleetError::NotFound(error_message_with_context(&error)))
        }
        401 => {
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            warn!("Authentication error (401): {}", error.message);
            Err(FleetError::Authentication(error_message_with_context(
                &error,
            )))
        }
        422 => {
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            debug!("Validation error (422): {}", error.message);
            Err(FleetError::Validation(error_message_with_context(&error)))
        }
        402 => {
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            warn!("Premium required (402): {}", error.message);
            Err(FleetError::PremiumRequired {
                message: error.message,
                uuid: error.uuid,
            })
        }
        400 => {
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            debug!("Bad request error (400): {}", error.message);
            Err(FleetError::BadRequest(error_message_with_context(&error)))
        }

        _ => {
            let error = parse_error_response(response, status).await?;
            #[cfg(feature = "tracing")]
            warn!("API error ({}): {}", status, error.message);

            Err(FleetError::Api {
                status: status.as_u16(),
                message: error.message.clone(),
                uuid: error.uuid.clone(),
                response: Box::new(error),
            })
        }
    }
}

fn error_message_with_context(error: &ApiErrorResponse) -> String {
    let mut message = error.message.clone();
    if let Some(uuid) = &error.uuid {
        message.push_str(&format!(" [request UUID: {uuid}]"));
    }
    for detail in &error.errors {
        message.push_str(&format!("; {}: {}", detail.name, detail.reason));
    }
    message
}

async fn parse_error_response(
    response: reqwest::Response,
    status: reqwest::StatusCode,
) -> Result<ApiErrorResponse> {
    let bytes = read_limited_body(response, MAX_JSON_RESPONSE_SIZE).await?;
    let text = String::from_utf8_lossy(&bytes);
    if text.trim().is_empty() {
        return Ok(ApiErrorResponse {
            message: status
                .canonical_reason()
                .map_or_else(|| format!("HTTP {}", status.as_u16()), ToString::to_string),
            errors: Vec::new(),
            uuid: None,
        });
    }

    match serde_json::from_str::<ApiErrorResponse>(&text) {
        Ok(error) => Ok(error),
        Err(_) => Ok(ApiErrorResponse {
            message: text.into_owned(),
            errors: Vec::new(),
            uuid: None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_transport_errors_are_not_retried() {
        let retry_policy = RetryPolicy {
            max_retries: 2,
            base_delay: std::time::Duration::from_millis(10),
            max_delay: std::time::Duration::from_secs(1),
            respect_retry_after: false,
            use_jitter: false,
        };

        let attempt_count = Arc::new(Mutex::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result: Result<String> = handle_response_with_retry(&retry_policy, || {
            let count = attempt_count_clone.clone();
            async move {
                let mut attempts = count.lock().unwrap();
                *attempts += 1;

                Err(FleetError::RateLimit {
                    retry_after: None,
                    response: None,
                })
            }
        })
        .await;

        assert!(matches!(result, Err(FleetError::RateLimit { .. })));
        assert_eq!(*attempt_count.lock().unwrap(), 1);
    }

    #[test]
    fn test_retry_policy_exponential_backoff() {
        let mut policy = RetryPolicy::conservative();
        policy.use_jitter = false;

        assert_eq!(policy.calculate_delay(0), std::time::Duration::from_secs(1));
        assert_eq!(policy.calculate_delay(1), std::time::Duration::from_secs(2));
        assert_eq!(policy.calculate_delay(2), std::time::Duration::from_secs(4));
        assert_eq!(policy.calculate_delay(3), std::time::Duration::from_secs(8));
        // After reaching max_delay (30 secs), should cap
        assert_eq!(
            policy.calculate_delay(10),
            std::time::Duration::from_secs(30)
        );
    }

    #[test]
    fn test_retry_policy_parse_retry_after() {
        let policy = RetryPolicy::conservative();

        // Test parsing seconds format
        assert_eq!(
            policy.parse_retry_after("120"),
            Some(std::time::Duration::from_secs(30))
        );

        assert_eq!(
            policy.parse_retry_after("5"),
            Some(std::time::Duration::from_secs(5))
        );

        // Test non-numeric format (should return None and defer to exponential backoff)
        assert_eq!(policy.parse_retry_after("invalid"), None);
    }

    #[test]
    fn test_retry_policy_respects_disabled_retry_after() {
        let policy = RetryPolicy::none();
        // When respect_retry_after is false, should always return None
        assert_eq!(policy.parse_retry_after("120"), None);
    }

    #[test]
    fn test_conservative_retry_policy_defaults() {
        let policy = RetryPolicy::conservative();
        assert_eq!(policy.max_retries, 3);
        assert_eq!(policy.base_delay, std::time::Duration::from_secs(1));
        assert_eq!(policy.max_delay, std::time::Duration::from_secs(30));
        assert!(policy.respect_retry_after);
        assert!(policy.use_jitter);
    }

    #[test]
    fn test_aggressive_retry_policy_defaults() {
        let policy = RetryPolicy::aggressive();
        assert_eq!(policy.max_retries, 5);
        assert_eq!(policy.base_delay, std::time::Duration::from_millis(500));
        assert_eq!(policy.max_delay, std::time::Duration::from_secs(60));
        assert!(policy.respect_retry_after);
        assert!(policy.use_jitter);
    }

    #[test]
    fn test_default_retry_policy_is_conservative() {
        let default = RetryPolicy::default();
        let conservative = RetryPolicy::conservative();

        assert_eq!(default.max_retries, conservative.max_retries);
        assert_eq!(default.base_delay, conservative.base_delay);
        assert_eq!(default.max_delay, conservative.max_delay);
        assert_eq!(
            default.respect_retry_after,
            conservative.respect_retry_after
        );
        assert_eq!(default.use_jitter, conservative.use_jitter);
    }

    #[test]
    fn test_retry_policy_jitter_variation() {
        let mut policy = RetryPolicy::conservative();
        policy.use_jitter = true;

        // Run multiple times to check jitter adds variation (with high probability)
        let base_delay = std::time::Duration::from_secs(10);
        let delays: Vec<_> = (0..10)
            .map(|i| {
                policy.base_delay = base_delay;
                policy.calculate_delay(i)
            })
            .collect();

        // With jitter enabled, we should see variation (not all the same)
        // Due to randomness, this test might occasionally fail, but with 10 iterations
        // it's extremely unlikely all will be identical
        let unique_delays: std::collections::HashSet<_> = delays.into_iter().collect();
        assert!(
            unique_delays.len() > 1,
            "Jitter should create variation in delays"
        );
    }
}
