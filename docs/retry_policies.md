# Retry policies

The client can retry HTTP 429 rate-limit responses. It does not retry transport
errors or other HTTP statuses, so non-idempotent operations are not duplicated
after an ambiguous connection failure.

## Client-wide configuration

```rust
use fleetdm_api_client::{FleetClient, RetryPolicy};

fn build_client() -> fleetdm_api_client::Result<FleetClient> {
    Ok(FleetClient::builder("https://fleet.example.com")?
        .with_retry_policy(RetryPolicy::conservative())
        .with_token("token")?
        .build())
}
```

The policy applies through the shared transport to typed endpoints and generated
raw route wrappers. Multipart callers rebuild their request for each 429 retry;
an otherwise non-cloneable request is safely sent once.

Available presets are:

- `RetryPolicy::conservative()`: 3 retries, a 1-second base delay, and a
  30-second delay cap.
- `RetryPolicy::aggressive()`: 5 retries, a 500-millisecond base delay, and a
  60-second delay cap.
- `RetryPolicy::none()`: no retries.

Custom policies configure the retry count, exponential backoff, delay cap,
`Retry-After` handling, and jitter. `Retry-After` supports both delta-seconds and
HTTP-date values. Invalid or past dates fall back to the calculated delay.

No retry policy is installed by default.

## Per-request override

The label-list builder currently supports a focused override:

```rust
async fn list_labels(
    client: &FleetClient,
) -> fleetdm_api_client::Result<()> {
    client
        .labels()
        .list()
        .with_retry(RetryPolicy::aggressive())
        .send()
        .await?;

    client.labels().list().no_retry().send().await?;
    Ok(())
}
```

Other typed endpoints use the client-wide policy.

## Exhausted retries

After the configured retry count is exhausted, the client returns
`FleetError::RateLimit`. It retains the `Retry-After` value and Fleet's parsed
error body when present.

See [`examples/retry.rs`](../examples/retry.rs) for a complete example. Live
tests remain opt-in and should be run through `scripts/live-test.sh`; they are not
required to verify retry behavior because the offline suite exercises real mock
429 responses deterministically.
