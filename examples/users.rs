use fleetdm_api_client::FleetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Managing Users ===\n");

    // List all users
    let users = client.users().list().per_page(10).send().await?;
    println!("Found {} users", users.users().len());

    for user in users.users() {
        println!(
            "  - {} ({}) - Role: {:?}",
            user.name(),
            user.email(),
            user.global_role()
        );
    }

    println!("\n✅ Users example completed successfully!");

    Ok(())
}
