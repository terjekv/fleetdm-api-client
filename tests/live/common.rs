//! Common utilities for live integration tests

use fleetdm_api_client::{FleetClient, Result, RetryPolicy};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::env;
use std::fs;
use tokio::sync::Mutex as AsyncMutex;

// Cache the auth token (not the client) to avoid runtime-crossing issues
static AUTH_TOKEN: OnceCell<AsyncMutex<Option<String>>> = OnceCell::new();
static LIVE_FIXTURE: OnceCell<LiveFixture> = OnceCell::new();

/// Errors that indicate an optional Fleet capability or configuration is unavailable.
/// Transport, authentication, deserialization, and server failures are deliberately excluded.
pub fn is_optional_feature_error(error: &fleetdm_api_client::FleetError) -> bool {
    matches!(
        error,
        fleetdm_api_client::FleetError::PremiumRequired { .. }
            | fleetdm_api_client::FleetError::NotFound(_)
            | fleetdm_api_client::FleetError::BadRequest(_)
            | fleetdm_api_client::FleetError::Validation(_)
            | fleetdm_api_client::FleetError::Api {
                status: 403 | 405 | 409 | 501,
                ..
            }
    )
}

#[derive(Deserialize)]
pub struct LiveFixture {
    token: String,
    pub run_id: String,
    pub host_id: u64,
    #[serde(rename = "original_fleet_id")]
    pub _original_fleet_id: u64,
    pub fleet_id: Option<u64>,
    pub report_id: u64,
    pub label_id: u64,
    pub policy_id: u64,
    pub script_id: u64,
    pub user_id: u64,
}

#[derive(Deserialize)]
struct SharedToken {
    token: String,
}

fn shared_token() -> Result<Option<String>> {
    if let Some(fixture) = fixture()? {
        return Ok(Some(fixture.token.clone()));
    }
    let Ok(path) = env::var("FLEET_TOKEN_FILE") else {
        return Ok(None);
    };
    let bytes = fs::read(&path).map_err(|error| {
        fleetdm_api_client::FleetError::Config(format!(
            "could not read live token manifest {path}: {error}"
        ))
    })?;
    let token: SharedToken = serde_json::from_slice(&bytes).map_err(|error| {
        fleetdm_api_client::FleetError::Config(format!(
            "invalid live token manifest {path}: {error}"
        ))
    })?;
    Ok(Some(token.token))
}

pub fn fixture() -> Result<Option<&'static LiveFixture>> {
    let Ok(path) = env::var("FLEET_FIXTURE_FILE") else {
        return Ok(None);
    };
    LIVE_FIXTURE
        .get_or_try_init(|| {
            let bytes = fs::read(&path).map_err(|error| {
                fleetdm_api_client::FleetError::Config(format!(
                    "could not read live fixture manifest {path}: {error}"
                ))
            })?;
            serde_json::from_slice(&bytes).map_err(|error| {
                fleetdm_api_client::FleetError::Config(format!(
                    "invalid live fixture manifest {path}: {error}"
                ))
            })
        })
        .map(Some)
}

pub fn live_config() -> Option<(String, String, String)> {
    if env::var("FLEET_LIVE").ok().as_deref() != Some("1") {
        return None;
    }

    let url = env::var("FLEET_URL").ok()?;
    let email = env::var("FLEET_EMAIL").ok()?;
    let password = env::var("FLEET_PASSWORD").ok()?;
    Some((url, email, password))
}

pub async fn authenticated_client() -> Result<Option<FleetClient>> {
    let Some((url, email, password)) = live_config() else {
        return Ok(None);
    };

    // Get or create token (one login, cached for all tests)
    let token_cell = AUTH_TOKEN.get_or_init(|| AsyncMutex::new(None));
    let token = {
        let mut guard = token_cell.lock().await;
        if guard.is_none() {
            *guard = if let Some(token) = shared_token()? {
                Some(token)
            } else {
                let authenticated_builder = FleetClient::builder(&url)?
                    .with_retry_policy(RetryPolicy::aggressive())
                    .login(email, password)
                    .await?;
                Some(authenticated_builder.build().token().to_string())
            };
        }
        guard.clone().unwrap()
    };

    // Build a fresh client per test using the cached token
    let client = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .with_token(token)?
        .build();

    Ok(Some(client))
}
