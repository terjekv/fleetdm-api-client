//! Carve endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;

#[tokio::test]
async fn list_carves() -> Result<()> {
    let client: FleetClient = live_client!();

    let carves = client.carves().list().await?;

    // Carves may be empty or null, but the call should succeed
    if let Some(carve_list) = carves.carves() {
        for carve in carve_list.iter().take(3) {
            assert!(carve.id() > 0, "each carve should have a valid id");
        }
    }

    Ok(())
}
