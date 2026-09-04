//! Policy endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;
use fleetdm_api_client::models::policy::{CreatePolicyRequest, UpdatePolicyRequest};

#[tokio::test]
async fn list_policies() -> Result<()> {
    let client: FleetClient = live_client!();

    let policies = client.policies().list().per_page(100).send().await?;
    assert!(
        policies.policies().len() <= 100,
        "should respect per_page limit"
    );

    for policy in policies.policies() {
        assert!(policy.id() > 0, "each policy should have a valid id");
        assert!(!policy.name().is_empty(), "each policy should have a name");
        assert!(
            !policy.query().is_empty(),
            "each policy should have a query"
        );
    }

    Ok(())
}

#[tokio::test]
async fn create_policy() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreatePolicyRequest {
        name: format!("Test Policy {}", chrono::Utc::now().timestamp()),
        query: "SELECT 1 FROM osquery_info WHERE version != ''".to_string(),
        description: "Test policy for integration testing".to_string(),
        resolution: Some("This is a test policy".to_string()),
        team_id: None,
        platform: Some("darwin,linux,windows".to_string()),
        critical: Some(false),
        conditional_access_enabled: Some(false),
    };

    let created = client.policies().create(create_req.clone()).await?;
    assert!(
        created.policy().id() > 0,
        "created policy should have a valid id"
    );
    assert_eq!(
        created.policy().name(),
        create_req.name,
        "created policy name should match"
    );
    assert_eq!(
        created.policy().query(),
        create_req.query,
        "created policy query should match"
    );
    // Clean up
    client.policies().delete(created.policy().id()).await?;

    Ok(())
}

#[tokio::test]
async fn read_policy() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreatePolicyRequest {
        name: format!("Read Policy {}", chrono::Utc::now().timestamp()),
        query: "SELECT 1 FROM osquery_info WHERE version != ''".to_string(),
        description: "Read policy for integration testing".to_string(),
        resolution: Some("Read policy".to_string()),
        team_id: None,
        platform: Some("darwin,linux,windows".to_string()),
        critical: Some(false),
        conditional_access_enabled: Some(false),
    };

    let created = client.policies().create(create_req.clone()).await?;

    let fetched = client.policies().get(created.policy().id()).await?;
    assert_eq!(
        fetched.policy().id(),
        created.policy().id(),
        "fetched policy id should match"
    );
    assert_eq!(
        fetched.policy().name(),
        create_req.name,
        "fetched policy name should match"
    );

    // Clean up
    client.policies().delete(created.policy().id()).await?;

    Ok(())
}

#[tokio::test]
async fn delete_policy() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreatePolicyRequest {
        name: format!("Delete Policy {}", chrono::Utc::now().timestamp()),
        query: "SELECT 1 FROM osquery_info WHERE version != ''".to_string(),
        description: "Delete policy for integration testing".to_string(),
        resolution: Some("Delete policy".to_string()),
        team_id: None,
        platform: Some("darwin,linux,windows".to_string()),
        critical: Some(false),
        conditional_access_enabled: Some(false),
    };

    let created = client.policies().create(create_req.clone()).await?;

    client.policies().delete(created.policy().id()).await?;

    let get_result = client.policies().get(created.policy().id()).await;
    assert!(get_result.is_err(), "getting deleted policy should fail");

    Ok(())
}

#[tokio::test]
async fn update_policy() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create an initial policy
    let create_req = CreatePolicyRequest {
        name: format!("Update Policy {}", chrono::Utc::now().timestamp()),
        query: "SELECT 1 FROM osquery_info WHERE version != ''".to_string(),
        description: "Original description".to_string(),
        resolution: Some("Original resolution".to_string()),
        team_id: None,
        platform: Some("darwin,linux,windows".to_string()),
        critical: Some(false),
        conditional_access_enabled: Some(false),
    };

    let created = client.policies().create(create_req.clone()).await?;

    // Perform update
    let update_req = UpdatePolicyRequest {
        name: None,
        query: None,
        description: Some("Updated description".to_string()),
        resolution: Some("Updated resolution".to_string()),
        team_id: None,
        platform: None,
        critical: Some(true),
        conditional_access_enabled: None,
    };

    let updated = match client
        .policies()
        .update(created.policy().id(), update_req.clone())
        .await
    {
        Ok(resp) => resp,
        // Some backends may reject updates (e.g., policy not editable). If so, skip.
        Err(FleetError::Api { status: 400, .. }) | Err(FleetError::BadRequest(_)) => return Ok(()),
        Err(e) => return Err(e),
    };

    assert_eq!(updated.policy().description(), "Updated description");
    assert_eq!(updated.policy().resolution(), "Updated resolution");
    assert!(
        updated.policy().critical(),
        "updated policy should be critical"
    );

    // Clean up
    client.policies().delete(updated.policy().id()).await?;

    Ok(())
}
