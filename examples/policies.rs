use fleetdm_api_client::FleetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Managing Policies ===\n");

    // List all policies
    let policies = client.policies().list().per_page(10).send().await?;
    println!("Found {} policies", policies.policies().len());

    for policy in policies.policies().iter().take(5) {
        println!(
            "  - {} (ID: {}, Critical: {})",
            policy.name(),
            policy.id(),
            policy.critical()
        );
    }

    println!("\n✅ Policies example completed successfully!");

    Ok(())
}
