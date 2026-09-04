use fleetdm_api_client::FleetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Managing Scripts ===\n");

    // List all scripts
    let scripts = client.scripts().list().per_page(10).send().await?;
    let items = scripts.scripts();
    println!("Found {} scripts", items.len());

    for script in items.iter().take(5) {
        println!("  - {} (ID: {})", script.name(), script.id());
    }

    println!("\n✅ Scripts example completed successfully!");

    Ok(())
}
