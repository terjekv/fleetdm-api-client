#![allow(deprecated)]
// Integration test entrypoint for live suite
#[path = "live/mod.rs"]
mod live;

/// Obtain an authenticated client. The live test target requires the `live-tests` feature.
#[macro_export]
macro_rules! live_client {
    () => {{
        match $crate::live::common::authenticated_client().await? {
            Some(client) => client,
            None => {
                return Err(fleetdm_api_client::FleetError::Config(
                    "live tests require FLEET_LIVE=1, FLEET_URL, FLEET_EMAIL, and FLEET_PASSWORD"
                        .into(),
                ));
            }
        }
    }};
}

/// Obtain the deterministic fixture seeded by `scripts/live-test.sh`.
#[macro_export]
macro_rules! live_fixture {
    () => {{
        match $crate::live::common::fixture()? {
            Some(fixture) => fixture,
            None => {
                return Err(fleetdm_api_client::FleetError::Config(
                    "live fixture manifest is missing; run tests through scripts/live-test.sh"
                        .into(),
                ));
            }
        }
    }};
}
