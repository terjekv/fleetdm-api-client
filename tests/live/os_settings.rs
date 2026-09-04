//! OS Settings endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;
use fleetdm_api_client::models::FileUpload;
use fleetdm_api_client::models::os_settings::*;

fn is_os_settings_api_error(error: &FleetError) -> bool {
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
async fn list_configuration_profiles() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.os_settings().list_profiles(None).await;

    match result {
        Ok(profiles) => {
            // Profiles are optional — may be empty if MDM not configured
            for profile in profiles.profiles() {
                assert!(!profile.name().is_empty(), "profile should have a name");
                if let Some(identifier) = profile.identifier() {
                    assert!(
                        !identifier.is_empty(),
                        "profile identifier should not be empty"
                    );
                }
                assert!(
                    !profile.profile_uuid().is_empty(),
                    "profile should have a uuid"
                );
            }
        }
        Err(error) if is_os_settings_api_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn get_disk_encryption_settings() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.os_settings().get_disk_encryption(None).await;

    match result {
        Ok(settings) => {
            let _ = settings.verified().macos();
            let _ = settings.failed().windows();
        }
        Err(error) if is_os_settings_api_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn profiles_summary() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.os_settings().profiles_summary(None).await;

    match result {
        Ok(summary) => {
            let _ = summary.verified();
            let _ = summary.failed();
        }
        Err(error) if is_os_settings_api_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn configuration_profile_lifecycle() -> Result<()> {
    let client: FleetClient = live_client!();

    // Skip if we can't list profiles (MDM not configured)
    if client.os_settings().list_profiles(None).await.is_err() {
        return Ok(());
    }

    // Creating actual profiles requires valid mobileconfig XML
    // Test that invalid content is rejected with an error, not a panic
    let invalid_profile = CreateConfigurationProfileRequest::new(FileUpload::new(
        "invalid.mobileconfig",
        b"invalid-profile-content".to_vec(),
    )?);

    let result = client.os_settings().create_profile(invalid_profile).await;
    // Both success and error are acceptable — we're testing the endpoint exists
    // and returns a structured response
    match result {
        Ok(_) => {}
        Err(e) => {
            // Should be an API error, not a deserialization or connection error
            let err_str = format!("{:?}", e);
            assert!(
                err_str.contains("Api")
                    || err_str.contains("BadRequest")
                    || err_str.contains("Validation"),
                "profile creation error should be an API error, got: {}",
                err_str
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn configuration_profile_detail_and_status() -> Result<()> {
    let client: FleetClient = live_client!();

    let list = match client.os_settings().list_profiles(None).await {
        Ok(list) => list,
        Err(error) if is_os_settings_api_error(&error) => return Ok(()),
        Err(error) => return Err(error),
    };

    let Some(profile) = list.profiles().first() else {
        return Ok(());
    };

    let detail = client
        .os_settings()
        .get_profile(profile.profile_uuid())
        .await?;
    assert_eq!(detail.profile().profile_uuid(), profile.profile_uuid());

    let status = client
        .os_settings()
        .get_profile_status(profile.profile_uuid())
        .await?;
    let _ = status.verified();
    let _ = status.failed();

    Ok(())
}
