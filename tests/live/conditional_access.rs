//! Conditional-access integration download tests.

use crate::live_client;
use fleetdm_api_client::{FleetClient, FleetError, Result};

fn is_unconfigured(error: &FleetError) -> bool {
    matches!(
        error,
        FleetError::PremiumRequired { .. }
            | FleetError::NotFound(_)
            | FleetError::BadRequest(_)
            | FleetError::Validation(_)
            | FleetError::Api { .. }
    )
}

#[tokio::test]
async fn conditional_access_download_endpoints() -> Result<()> {
    let client: FleetClient = live_client!();

    match client.conditional_access().okta_signing_certificate().await {
        Ok(certificate) => assert!(!certificate.is_empty()),
        Err(error) if is_unconfigured(&error) => {}
        Err(error) => return Err(error),
    }

    match client.conditional_access().okta_apple_profile().await {
        Ok(profile) => assert!(!profile.is_empty()),
        Err(error) if is_unconfigured(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}
