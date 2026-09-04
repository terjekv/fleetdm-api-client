//! Config endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;

#[tokio::test]
async fn get_config() -> Result<()> {
    let client: FleetClient = live_client!();

    let config = client.config().get().await?;
    assert!(
        !config.org_info().org_name().is_empty(),
        "org name should not be empty"
    );

    Ok(())
}

#[tokio::test]
async fn get_enroll_secrets() -> Result<()> {
    let client: FleetClient = live_client!();

    let secrets = client.config().get_enroll_secrets().await?;

    // Enroll secrets may be empty or null, but the call should succeed
    // Just verify we got a valid response structure
    if let Some(secret_list) = secrets.secrets() {
        // If secrets exist, they should be valid
        for secret in secret_list.iter().take(1) {
            assert!(
                !secret.secret().is_empty(),
                "each secret should have a value"
            );
        }
    }

    Ok(())
}
