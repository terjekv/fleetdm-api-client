use fleetdm_api_client::{
    FleetClient,
    models::{CreateReportRequest, ReportOrderKey},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Managing Reports ===\n");

    let reports = client
        .reports()
        .list()
        .order_key(ReportOrderKey::Name)
        .per_page(10)
        .send()
        .await?;

    println!("Found {} reports", reports.reports().len());

    let request = CreateReportRequest {
        name: "Example Report".to_string(),
        query: "SELECT name, version FROM os_version".to_string(),
        description: Some("Example report created by fleetdm_api_client".to_string()),
        fleet_id: None,
        interval: None,
        platform: Some("darwin,linux,windows".to_string()),
        discard_data: Some(false),
        ..Default::default()
    };

    let created = client.reports().create(request).await?;
    println!("Created report: {}", created.report().name());

    println!("\nReports example completed successfully!");
    Ok(())
}
