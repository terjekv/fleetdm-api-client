//! Session endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;

#[tokio::test]
async fn delete_session() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a disposable session by logging in again
    let url = std::env::var("FLEET_URL").unwrap();
    let email = std::env::var("FLEET_EMAIL").unwrap();
    let password = std::env::var("FLEET_PASSWORD").unwrap();

    let disposable = FleetClient::builder(&url)?
        .login(&email, &password)
        .await?
        .build();

    // Get the current user's ID via /me
    let me = disposable.auth().me().await?;
    let user_id = me.user().id();

    // List the user's sessions via raw_api
    let sessions_json = client
        .raw_api()
        .ep_get_api_v1_fleet_users_by_id_sessions(user_id.to_string(), None)
        .await?;

    let sessions = sessions_json
        .body()
        .get("sessions")
        .and_then(|s| s.as_array())
        .expect("should have sessions array");

    assert!(
        sessions.len() >= 2,
        "should have at least 2 sessions (main + disposable)"
    );

    // Find the last session (our disposable one) and delete it
    let last_session_id = sessions
        .last()
        .and_then(|s| s.get("session_id"))
        .and_then(|id| id.as_u64())
        .expect("session should have session_id");

    let deleted = client.sessions().delete(last_session_id).await?;
    let _ = deleted.message();

    // Verify the session count decreased
    let after = client
        .raw_api()
        .ep_get_api_v1_fleet_users_by_id_sessions(user_id.to_string(), None)
        .await?;
    let after_sessions = after
        .body()
        .get("sessions")
        .and_then(|s| s.as_array())
        .expect("should have sessions array");

    assert!(
        after_sessions.len() < sessions.len(),
        "session count should decrease after deletion"
    );

    Ok(())
}
