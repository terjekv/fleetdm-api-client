//! Tests for premium feature detection
//!
//! These tests verify that the client properly detects and reports
//! when Fleet Premium features are accessed without the required license.

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;
use fleetdm_api_client::models::fleet::CreateFleetRequest;

/// Test that team creation returns PremiumRequired error on free Fleet
#[tokio::test]
async fn team_creation_requires_premium() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreateFleetRequest {
        name: format!("Premium Test Team {}", chrono::Utc::now().timestamp()),
    };

    let result = client.fleets().create(create_req.clone()).await;

    match result {
        Ok(_) => {
            // If it succeeds, we have a Premium license - that's fine
            Ok(())
        }
        Err(FleetError::PremiumRequired { message, uuid }) => {
            // This is the expected error on free Fleet
            assert!(
                message.contains("Premium") || message.contains("license"),
                "Premium error message should mention premium or license: {}",
                message
            );
            // UUID should be present for tracking
            assert!(uuid.is_some(), "Premium error should include UUID");
            Ok(())
        }
        Err(e) => {
            // Any other error is unexpected
            panic!("Expected PremiumRequired error, got: {:?}", e);
        }
    }
}

/// Test that team listing returns PremiumRequired error on free Fleet
#[tokio::test]
async fn team_listing_requires_premium() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.fleets().list().per_page(10).send().await;

    match result {
        Ok(_) => {
            // If it succeeds, we have a Premium license - that's fine
            Ok(())
        }
        Err(FleetError::PremiumRequired { message, .. }) => {
            // This is the expected error on free Fleet
            assert!(
                message.contains("Premium") || message.contains("license"),
                "Premium error message should mention premium or license: {}",
                message
            );
            Ok(())
        }
        Err(e) => {
            panic!("Expected PremiumRequired error, got: {:?}", e);
        }
    }
}

/// Test that vulnerability exploit filter returns PremiumRequired error on free Fleet
#[tokio::test]
async fn vulnerability_exploit_filter_requires_premium() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client
        .vulnerabilities()
        .list()
        .exploit(true)
        .per_page(10)
        .send()
        .await;

    match result {
        Ok(_) => {
            // If it succeeds, we have a Premium license - that's fine
            Ok(())
        }
        Err(FleetError::PremiumRequired { message, .. }) => {
            // This is the expected error on free Fleet
            assert!(
                message.contains("Premium") || message.contains("license"),
                "Premium error message should mention premium or license: {}",
                message
            );
            Ok(())
        }
        Err(e) => {
            panic!("Expected PremiumRequired error, got: {:?}", e);
        }
    }
}

/// Test that PremiumRequired error can be distinguished from other errors
#[tokio::test]
async fn premium_error_is_distinct() -> Result<()> {
    let client: FleetClient = live_client!();

    // Try to get a non-existent team (this should fail with NotFound or PremiumRequired)
    let result = client.fleets().get(999999999).await;

    match result {
        Ok(_) => {
            // Highly unlikely a team with ID 999999999 exists
            panic!("Unexpectedly found team with ID 999999999");
        }
        Err(FleetError::PremiumRequired { .. }) => {
            // Premium error means we can't even try
            Ok(())
        }
        Err(FleetError::NotFound(_)) => {
            // NotFound is acceptable if we have Premium
            Ok(())
        }
        Err(FleetError::Api { status: 404, .. }) => {
            // Some implementations might return API error with 404
            Ok(())
        }
        Err(e) => {
            panic!("Expected NotFound or PremiumRequired, got: {:?}", e);
        }
    }
}

/// Test that 402 status code is correctly mapped to PremiumRequired
#[tokio::test]
async fn status_402_maps_to_premium_error() -> Result<()> {
    let client: FleetClient = live_client!();

    // Test teams list endpoint
    let result = client.fleets().list().send().await;

    if let Err(e) = result {
        match e {
            FleetError::PremiumRequired { .. } => {
                // Good - 402 was properly mapped
            }
            FleetError::Api { status: 402, .. } => {
                panic!(
                    "Teams list returned 402 but wasn't mapped to PremiumRequired: {:?}",
                    e
                );
            }
            _ => {
                // Other errors are acceptable (might have Premium license)
            }
        }
    }

    // Test team create endpoint
    let create_req = CreateFleetRequest {
        name: "Test".to_string(),
    };

    let result = client.fleets().create(create_req.clone()).await;

    if let Err(e) = result {
        match e {
            FleetError::PremiumRequired { .. } => {
                // Good - 402 was properly mapped
            }
            FleetError::Api { status: 402, .. } => {
                panic!(
                    "Team create returned 402 but wasn't mapped to PremiumRequired: {:?}",
                    e
                );
            }
            _ => {
                // Other errors are acceptable
            }
        }
    }

    Ok(())
}
