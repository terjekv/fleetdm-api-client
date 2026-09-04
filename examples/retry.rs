use fleetdm_api_client::{FleetClient, RetryPolicy};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get environment variables
    let url = env::var("FLEET_URL").expect("FLEET_URL not set");
    let token = env::var("FLEET_API_TOKEN").expect("FLEET_API_TOKEN not set");

    // Example 1: Client with no retry policy (default)
    // Rate limit errors will be returned immediately to the caller
    println!("Example 1: No retry policy (fails immediately on rate limit)");
    let client_no_retry = FleetClient::builder(&url)?.with_token(&token)?.build();

    match client_no_retry.hosts().list().send().await {
        Ok(hosts) => println!("✓ Found {} hosts", hosts.hosts().len()),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Example 2: Client with conservative retry policy (good for production)
    // - 3 retry attempts
    // - 1 second base delay
    // - 30 second max delay
    // - Respects Retry-After headers
    // - Uses jitter to prevent thundering herd
    println!("\nExample 2: Conservative retry policy (production-ready)");
    let client_conservative = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::conservative())
        .with_token(&token)?
        .build();

    match client_conservative.hosts().list().send().await {
        Ok(hosts) => println!(
            "✓ Found {} hosts (with automatic retries)",
            hosts.hosts().len()
        ),
        Err(e) => println!("✗ Error after retries: {}", e),
    }

    // Example 3: Client with aggressive retry policy (good for testing)
    // - 5 retry attempts
    // - 500ms base delay
    // - 60 second max delay
    // - Respects Retry-After headers
    // - Uses jitter
    println!("\nExample 3: Aggressive retry policy (testing/high-availability)");
    let client_aggressive = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .with_token(&token)?
        .build();

    match client_aggressive.reports().list().send().await {
        Ok(reports) => println!(
            "✓ Found {} reports (with aggressive retries)",
            reports.reports().len()
        ),
        Err(e) => println!("✗ Error after aggressive retries: {}", e),
    }

    // Example 4: Custom retry policy
    println!("\nExample 4: Custom retry policy");
    let custom_policy = RetryPolicy {
        max_retries: 2,
        base_delay: std::time::Duration::from_secs(2),
        max_delay: std::time::Duration::from_secs(15),
        respect_retry_after: true,
        use_jitter: false, // Disable jitter for predictable delays
    };

    let client_custom = FleetClient::builder(&url)?
        .with_retry_policy(custom_policy)
        .with_token(&token)?
        .build();

    match client_custom.labels().list().send().await {
        Ok(labels) => println!(
            "✓ Found {} labels (with custom retry)",
            labels.labels().len()
        ),
        Err(e) => println!("✗ Error with custom retry: {}", e),
    }

    // Example 5: Per-request retry override
    // Note: Currently only implemented for some endpoints (e.g., labels)
    // Other endpoints will be updated in future versions
    println!("\nExample 5: Per-request retry override (labels endpoint)");
    let client = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::conservative()) // Default policy
        .with_token(&token)?
        .build();

    // Override with aggressive retry for this specific request
    match client
        .labels()
        .list()
        .with_retry(RetryPolicy::aggressive())
        .send()
        .await
    {
        Ok(labels) => println!(
            "✓ Found {} labels (with per-request override)",
            labels.labels().len()
        ),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Explicitly disable retry for this request
    match client.labels().list().no_retry().send().await {
        Ok(labels) => println!("✓ Found {} labels (without retry)", labels.labels().len()),
        Err(e) => println!("✗ Error without retry: {}", e),
    }

    println!("\n✅ All examples completed!");

    Ok(())
}
