//! Team policy endpoint tests (requires Fleet Premium)

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;
use fleetdm_api_client::models::fleet::CreateFleetRequest;
use fleetdm_api_client::models::policy::{CreatePolicyRequest, UpdatePolicyRequest};

#[tokio::test]
async fn team_policy_lifecycle() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();

    // Create a team first (requires premium)
    let team_req = CreateFleetRequest {
        name: format!("Policy Test Team {}", ts),
    };
    let team = match client.fleets().create(team_req).await {
        Ok(t) => t,
        Err(FleetError::PremiumRequired { .. }) => return Ok(()),
        Err(e) => return Err(e),
    };
    let team_id = team.fleet().id();

    // Create a team policy
    let policy_req = CreatePolicyRequest {
        name: format!("Team Policy {}", ts),
        query: "SELECT 1 FROM os_version WHERE major >= 14".to_string(),
        description: "Test team policy".to_string(),
        resolution: Some("Update your OS".to_string()),
        team_id: None, // team_id is in the URL path, not the body
        platform: Some("darwin".to_string()),
        critical: Some(false),
        conditional_access_enabled: Some(false),
    };
    let created = client.policies().create_team(team_id, policy_req).await?;
    let policy_id = created.policy().id();
    assert!(policy_id > 0);
    assert_eq!(created.policy().name(), format!("Team Policy {}", ts));

    // Get team policy
    let fetched = client.policies().get_team(team_id, policy_id).await?;
    assert_eq!(fetched.policy().id(), policy_id);

    // List team policies
    let list = client.policies().list_team(team_id).send().await?;
    assert!(
        list.policies().iter().any(|p| p.id() == policy_id),
        "team policy list should contain our policy"
    );

    // Update team policy
    let update_req = UpdatePolicyRequest {
        name: Some(format!("Updated Team Policy {}", ts)),
        query: None,
        description: None,
        resolution: None,
        team_id: None,
        platform: None,
        critical: Some(true),
        conditional_access_enabled: None,
    };
    let updated = client
        .policies()
        .update_team(team_id, policy_id, update_req)
        .await?;
    assert_eq!(
        updated.policy().name(),
        format!("Updated Team Policy {}", ts)
    );
    assert!(updated.policy().critical());

    // Delete team policy
    client
        .policies()
        .delete_team(team_id, vec![policy_id])
        .await?;

    // Verify deleted
    let list_after = client.policies().list_team(team_id).send().await?;
    assert!(
        !list_after.policies().iter().any(|p| p.id() == policy_id),
        "deleted policy should not appear in list"
    );

    // Cleanup: delete team
    client.fleets().delete(team_id).await?;

    Ok(())
}

#[tokio::test]
async fn team_policy_list_with_merge_inherited() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();

    // Create a team
    let team = match client
        .fleets()
        .create(CreateFleetRequest {
            name: format!("Inherit Test {}", ts),
        })
        .await
    {
        Ok(t) => t,
        Err(FleetError::PremiumRequired { .. }) => return Ok(()),
        Err(e) => return Err(e),
    };
    let team_id = team.fleet().id();

    // List with merge_inherited=true — should include global policies
    let merged = client
        .policies()
        .list_team(team_id)
        .merge_inherited(true)
        .send()
        .await?;
    let _ = merged.policies();

    // List without merge — should only show team-specific policies
    let team_only = client.policies().list_team(team_id).send().await?;
    assert!(
        team_only.policies().len() <= merged.policies().len(),
        "team-only policies should not exceed merged count"
    );

    // Cleanup
    client.fleets().delete(team_id).await?;

    Ok(())
}
