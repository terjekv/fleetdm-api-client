use fleetdm_api_client::{FleetClient, models::ReportOrderKey};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    let reports = client
        .reports()
        .list()
        .order_key(ReportOrderKey::Name)
        .per_page(20)
        .send()
        .await?;

    for report in reports.reports() {
        println!("{}: {}", report.id(), report.name());
    }
    Ok(())
}
