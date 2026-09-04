use fleetdm_api_client::{FleetClient, models::ListSoftwareQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Listing Software Versions ===\n");

    // List all software versions
    let query = ListSoftwareQuery::new().per_page(10);
    let params = query.to_query_params();
    let params_ref: Vec<(&str, &str)> = params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let software = client.software().list_versions(Some(&params_ref)).await?;
    let items = software.software();
    println!("Found {} software items", items.len());

    for sw in items.iter().take(5) {
        let host_count = sw.hosts_count().unwrap_or(0);
        println!("  - {} v{} ({} hosts)", sw.name(), sw.version(), host_count);
    }

    // List vulnerable software versions
    println!("\nListing vulnerable software...");
    let vulnerable_query = ListSoftwareQuery::new().vulnerable(true).per_page(5);
    let vulnerable_params = vulnerable_query.to_query_params();
    let vulnerable_params_ref: Vec<(&str, &str)> = vulnerable_params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let vulnerable_sw = client
        .software()
        .list_versions(Some(&vulnerable_params_ref))
        .await?;
    let vuln_items = vulnerable_sw.software();
    println!("Found {} vulnerable software items", vuln_items.len());

    for sw in vuln_items.iter().take(5) {
        let host_count = sw.hosts_count().unwrap_or(0);
        println!("  - {} v{} ({} hosts)", sw.name(), sw.version(), host_count);
    }

    println!("\n✅ Software example completed successfully!");

    Ok(())
}
