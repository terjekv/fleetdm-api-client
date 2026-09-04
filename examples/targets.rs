use fleetdm_api_client::{Result, models::target::SearchTargetsRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("FLEET_URL").expect("FLEET_URL must be set");
    let email = env::var("FLEET_EMAIL").expect("FLEET_EMAIL must be set");
    let password = env::var("FLEET_PASSWORD").expect("FLEET_PASSWORD must be set");

    // Authenticate
    let client = fleetdm_api_client::FleetClient::builder(&url)?
        .login(email, password)
        .await?
        .build();

    println!("Successfully authenticated to Fleet");
    println!();

    // Search targets
    println!("===== Targets =====");
    let target_search = SearchTargetsRequest {
        query: "".to_string(),
        query_id: None,
        selected: None,
        include_observer: Some(true),
    };

    let targets = client.targets().search(&target_search).await?;
    println!("Total targets: {}", targets.targets_count());
    println!("  Online: {}", targets.targets_online());
    println!("  Offline: {}", targets.targets_offline());
    println!();

    Ok(())
}
