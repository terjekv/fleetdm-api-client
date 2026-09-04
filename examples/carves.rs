use fleetdm_api_client::Result;
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

    // List carves
    println!("===== File Carving =====");
    match client.carves().list().await {
        Ok(carves) => {
            if let Some(carve_list) = carves.carves() {
                println!("Found {} carves", carve_list.len());
                for carve in carve_list.iter().take(3) {
                    println!(
                        "  - Carve {} (host_id={}, blocks={}, expired={})",
                        carve.id(),
                        carve.host_id(),
                        carve.block_count(),
                        carve.expired().unwrap_or(false)
                    );
                }
            } else {
                println!("No carves found");
            }
        }
        Err(e) => {
            println!("File carving may not be enabled: {}", e);
        }
    }
    println!();

    Ok(())
}
