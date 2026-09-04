use fleetdm_api_client::FleetClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Managing Labels ===\n");

    // List all labels
    let labels = client.labels().list().per_page(10).send().await?;
    println!("Found {} labels", labels.labels().len());

    for label in labels.labels() {
        let host_count = label
            .host_count()
            .unwrap_or_else(|| label.count().unwrap_or(0));
        println!("  - {} ({} hosts)", label.name(), host_count);
    }

    println!("\n✅ Labels example completed successfully!");

    Ok(())
}
