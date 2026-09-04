use fleetdm_api_client::FleetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Managing Fleets ===\n");

    // List all fleets
    let fleets = client.fleets().list().per_page(10).send().await?;
    println!("Found {} fleets", fleets.fleets().len());

    for fleet in fleets.fleets() {
        println!("  - {} (ID: {})", fleet.name(), fleet.id());
    }

    println!("\n✅ Fleets example completed successfully!");

    Ok(())
}
