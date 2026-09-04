//! Common utilities for live integration tests

use fleetdm_api_client::{FleetClient, Result};
use std::env;

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
    match live_config() {
        Some((url, email, password)) => {
            let client = FleetClient::builder(&url)?
                .login(email, password)
                .await?
                .build();
            Ok(Some(client))
        }
        None => Ok(None),
    }
}
