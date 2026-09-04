//! Invitation endpoint tests

use crate::live_client;
use fleetdm_api_client::models::invitation::*;
use fleetdm_api_client::{FleetClient, FleetError, Result};

#[tokio::test]
async fn list_invitations() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.invitations().list().send().await?;
    // Fresh instance may have zero invitations — that's fine
    let _ = result.invites();

    Ok(())
}

#[tokio::test]
async fn invitation_lifecycle() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();
    let email = format!("invite-test-{}@example.com", ts);

    // Create — requires SMTP/SES to be configured
    let create_req = CreateInvitationRequest {
        email: email.clone(),
        name: "Test Invite".to_string(),
        sso_enabled: false,
        global_role: Some("observer".to_string()),
        teams: None,
    };
    let create_result = client.invitations().create(create_req).await;
    let created = match create_result {
        Ok(resp) => resp,
        Err(error)
            if crate::live::common::is_optional_feature_error(&error)
                || matches!(
                    &error,
                    FleetError::Api {
                        status: 500,
                        message,
                        ..
                    } if message.contains("requires that SMTP or SES (email) is configured")
                ) =>
        {
            // SMTP/SES not configured — skip the rest of the lifecycle test
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let invite_id = created.invite().id();
    assert!(invite_id > 0, "invite should have an id");
    assert_eq!(created.invite().email(), email);
    assert_eq!(created.invite().name(), "Test Invite");
    assert_eq!(created.invite().global_role(), Some("observer"));

    // Get by ID
    let fetched = client.invitations().get(invite_id).await?;
    assert_eq!(fetched.invite().id(), invite_id);
    assert_eq!(fetched.invite().email(), email);
    assert_eq!(fetched.invite().name(), "Test Invite");

    // List — should contain our invite
    let list = client.invitations().list().send().await?;
    assert!(
        list.invites().iter().any(|i| i.id() == invite_id),
        "list should contain the created invite"
    );

    // Update
    let update_req = UpdateInvitationRequest {
        email: None,
        name: Some("Updated Invite".to_string()),
        sso_enabled: None,
        global_role: None,
        teams: None,
    };
    let updated = client.invitations().update(invite_id, update_req).await?;
    assert_eq!(updated.invite().name(), "Updated Invite");
    assert_eq!(updated.invite().email(), email);

    // Delete
    client.invitations().delete(invite_id).await?;

    // Verify deleted
    let list_after = client.invitations().list().send().await?;
    assert!(
        !list_after.invites().iter().any(|i| i.id() == invite_id),
        "deleted invite should not appear in list"
    );

    Ok(())
}
