use fleetdm_api_client::FleetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using an API token (recommended)
    let _client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("✅ Client created with API token");

    // Example 2: Using email/password authentication
    let client = FleetClient::builder("https://fleet.example.com")?
        .login("admin@example.com", "your-password")
        .await?
        .build();

    println!("✅ Client created with email/password");

    // Example 3: The following would NOT compile (authentication required):
    // let client = FleetClient::builder("https://fleet.example.com")?.build();
    //                                                                 ^^^^^
    // Error: no method named `build` found for struct `FleetClientBuilder<Unauthenticated>`

    // Demonstrate a simple API call
    match client.hosts().list().per_page(1).send().await {
        Ok(hosts) => {
            println!("✅ Successfully connected to FleetDM!");
            println!("   Found {} hosts", hosts.hosts().len());
        }
        Err(e) => {
            println!("❌ Failed to connect: {}", e);
        }
    }

    Ok(())
}
