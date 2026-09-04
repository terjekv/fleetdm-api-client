//! Integrations endpoint tests
//! Tests actual Fleet integration endpoints for APNs, ABM, VPP, SCIM, and Android Enterprise

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::FleetError;
use fleetdm_api_client::Result;

#[tokio::test]
pub async fn integrations_get_apns() -> Result<()> {
    let client: FleetClient = live_client!();

    // GET /api/v1/fleet/apns
    let result = client.integrations().get_apns().await;

    // If successful, verify the response structure
    match result {
        Ok(apns) => {
            assert!(!apns.common_name().is_empty());
            assert!(!apns.serial_number().is_empty());
        }
        Err(err) => {
            // Interestingly the APN endpoint returns 400 if we're not set up, and not 402
            // as we might expect for a premium feature.
            if matches!(err, FleetError::BadRequest(_)) {
                return Ok(());
            }
            panic!("Unexpected error getting APNs info: {}", err);
        }
    }

    Ok(())
}

#[tokio::test]
pub async fn integrations_list_abm_tokens() -> Result<()> {
    let client: FleetClient = live_client!();

    // GET /api/v1/fleet/abm_tokens (Fleet Premium)
    // May return 400 if we require a premium license
    let result = client.integrations().list_abm_tokens().await;

    // If successful, verify the response structure
    match result {
        Ok(abm_response) => {
            // Should have abm_tokens array (may be empty)
            let _ = abm_response.abm_tokens();
        }
        Err(err) => {
            if matches!(err, FleetError::PremiumRequired { .. }) {
                return Ok(());
            }
            panic!("Unexpected error listing ABM tokens: {}", err);
        }
    }

    Ok(())
}

#[tokio::test]
pub async fn integrations_list_vpp_tokens() -> Result<()> {
    let client: FleetClient = live_client!();

    // GET /api/v1/fleet/vpp_tokens (Fleet Premium)
    // May return 404 or require premium license
    let result = client.integrations().list_vpp_tokens().await;

    // If successful, verify the response structure
    if let Ok(vpp_response) = result {
        // Should have vpp_tokens array (may be empty)
        let _ = vpp_response.vpp_tokens();
    }

    Ok(())
}

#[tokio::test]
pub async fn integrations_get_scim_details() -> Result<()> {
    let client: FleetClient = live_client!();

    // GET /api/v1/fleet/scim/details
    // May return 400 if SCIM is not configured
    let result = client.integrations().get_scim_details().await;

    // If successful, verify the response structure
    match result {
        Ok(scim) => {
            let last_request = scim.last_request();
            assert!(!last_request.status().is_empty());
            assert!(!last_request.requested_at().is_empty());
        }
        Err(err) => {
            if matches!(err, FleetError::PremiumRequired { .. }) {
                return Ok(());
            }
            panic!("Unexpected error getting SCIM details: {}", err);
        }
    }

    Ok(())
}

#[tokio::test]
pub async fn integrations_get_android_enterprise() -> Result<()> {
    let client: FleetClient = live_client!();

    // GET /api/v1/fleet/android_enterprise
    let result = client.integrations().get_android_enterprise().await;

    let android = match result {
        Err(err) => {
            if matches!(err, FleetError::NotFound(_)) {
                return Ok(());
            }
            panic!(
                "Unexpected error getting Android Enterprise details: {}",
                err
            );
        }
        Ok(android) => android,
    };

    assert!(!android.android_enterprise_id().is_empty());

    Ok(())
}
