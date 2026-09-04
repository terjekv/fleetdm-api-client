use fleetdm_api_client::FleetClient;
use fleetdm_api_client::models::OrderDirection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Authenticate
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token-here")?
        .build();

    println!("=== Audit Log (Recent Activities) ===\n");

    // List recent activities
    let activities = client
        .activities()
        .list()
        .per_page(10)
        .order_direction(OrderDirection::Desc)
        .send()
        .await?;

    println!("Found {} recent activities", activities.activities().len());

    for activity in activities.activities().iter().take(10) {
        let actor = activity.actor_email().unwrap_or("System");
        println!(
            "  - {} performed: {} at {}",
            actor,
            activity.r#type(),
            activity.created_at()
        );
    }

    println!("\n✅ Activities example completed successfully!");

    Ok(())
}
