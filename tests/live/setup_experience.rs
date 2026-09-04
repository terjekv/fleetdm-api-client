//! Setup experience endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;

fn is_feature_or_missing_error(error: &FleetError) -> bool {
    matches!(
        error,
        FleetError::PremiumRequired { .. }
            | FleetError::NotFound(_)
            | FleetError::BadRequest(_)
            | FleetError::Validation(_)
    )
}

#[tokio::test]
pub async fn setup_experience_get_enrollment_secrets() -> Result<()> {
    let client: FleetClient = live_client!();

    let secrets_response = client.config().get_enroll_secrets().await?;
    if let Some(secrets) = secrets_response.secrets() {
        assert!(
            !secrets.is_empty(),
            "should have at least one enroll secret"
        );
        for secret in secrets {
            assert!(
                !secret.secret().is_empty(),
                "each secret should have a value"
            );
        }
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_config() -> Result<()> {
    let client: FleetClient = live_client!();

    let config = client.config().get().await?;
    let json = serde_json::to_value(config)?;

    // Config should always be a JSON object with known top-level keys
    assert!(json.is_object(), "config should be a JSON object");
    assert!(
        json.get("org_info").is_some(),
        "config should contain org_info"
    );
    assert!(
        json.get("server_settings").is_some(),
        "config should contain server_settings"
    );

    // MDM section should exist (even if empty/unconfigured)
    if let Some(mdm) = json.get("mdm") {
        assert!(mdm.is_object(), "mdm config should be an object");
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_update_config() -> Result<()> {
    let client: FleetClient = live_client!();

    // Read current config
    let before = client.config().get().await?;
    let before_json = serde_json::to_value(&before)?;

    // Update a safe setting
    let setup_config = serde_json::json!({
        "mdm": {
            "macos_setup": {
                "enable_end_user_authentication": false
            }
        }
    });

    let updated = client.config().update(&setup_config).await?;
    let updated_json = serde_json::to_value(updated)?;

    // Org info should be preserved after update
    assert_eq!(
        before_json.get("org_info"),
        updated_json.get("org_info"),
        "update should not change unrelated config"
    );

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_bootstrap_summary() -> Result<()> {
    let client: FleetClient = live_client!();

    let config = client.config().get().await?;
    let json = serde_json::to_value(config)?;

    // MDM section should be structured
    if let Some(mdm) = json.get("mdm") {
        // macos_migration is an object (even if empty)
        if let Some(migration) = mdm.get("macos_migration") {
            assert!(migration.is_object(), "macos_migration should be an object");
        }
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_config_profiles() -> Result<()> {
    let client: FleetClient = live_client!();

    let config = client.config().get().await?;
    let json = serde_json::to_value(config)?;

    if let Some(mdm) = json.get("mdm") {
        // These settings keys should exist as objects (even if empty)
        if let Some(macos_settings) = mdm.get("macos_settings") {
            assert!(
                macos_settings.is_object(),
                "macos_settings should be an object"
            );
        }
        if let Some(windows_settings) = mdm.get("windows_settings") {
            assert!(
                windows_settings.is_object(),
                "windows_settings should be an object"
            );
        }
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_eula_metadata() -> Result<()> {
    let client: FleetClient = live_client!();

    match client.setup_experience().eula_metadata().await {
        Ok(metadata) => {
            assert!(
                !metadata.token().is_empty(),
                "EULA metadata should include a token"
            );
        }
        Err(error) if is_feature_or_missing_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_software_endpoint() -> Result<()> {
    let client: FleetClient = live_client!();

    match client.setup_experience().get_software(None).await {
        Ok(response) => {
            for title in response.software_titles() {
                assert!(
                    title.id() > 0,
                    "setup experience software title should have an id"
                );
                assert!(
                    !title.name().is_empty(),
                    "setup experience software title should have a name"
                );
            }
        }
        Err(error) if is_feature_or_missing_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_script_endpoint() -> Result<()> {
    let client: FleetClient = live_client!();

    match client.setup_experience().get_script(None).await {
        Ok(script) => {
            assert!(script.id() > 0, "setup experience script should have an id");
            assert!(
                !script.name().is_empty(),
                "setup experience script should have a name"
            );
        }
        Err(error) if is_feature_or_missing_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_custom_enrollment_profile() -> Result<()> {
    let client: FleetClient = live_client!();

    match client
        .setup_experience()
        .enrollment_profile_automatic(None)
        .await
    {
        Ok(profile) => {
            assert!(
                !profile.enrollment_profile().is_empty(),
                "enrollment_profile should be an object"
            );
        }
        Err(error) if is_feature_or_missing_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
pub async fn setup_experience_manual_and_ota_profiles() -> Result<()> {
    let client: FleetClient = live_client!();

    match client
        .setup_experience()
        .enrollment_profile_manual(None)
        .await
    {
        Ok(profile) => {
            assert!(
                !profile.bytes().is_empty(),
                "manual enrollment profile download should not be empty"
            );
        }
        Err(error) if is_feature_or_missing_error(&error) => {}
        Err(error) => return Err(error),
    }

    match client.setup_experience().enrollment_profile_ota(None).await {
        Ok(profile) => {
            assert!(
                !profile.bytes().is_empty(),
                "OTA enrollment profile download should not be empty"
            );
        }
        Err(error) if is_feature_or_missing_error(&error) => {}
        Err(error) => return Err(error),
    }

    Ok(())
}
