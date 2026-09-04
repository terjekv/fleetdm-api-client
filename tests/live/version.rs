//! Version endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;

#[tokio::test]
async fn version_smoke() -> Result<()> {
    let client: FleetClient = live_client!();

    let version = client.version().get().await?;
    assert!(!version.version().is_empty(), "version should not be empty");

    Ok(())
}
