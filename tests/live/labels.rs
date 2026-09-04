//! Label endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::models::label::{CreateLabelRequest, LabelType, UpdateLabelRequest};

#[tokio::test]
async fn list_labels() -> Result<()> {
    let client: FleetClient = live_client!();

    let labels = client.labels().list().send().await?;
    assert!(
        !labels.labels().is_empty(),
        "should have at least built-in labels"
    );

    Ok(())
}

#[tokio::test]
async fn labels_detailed() -> Result<()> {
    let client: FleetClient = live_client!();

    let labels = client.labels().list().send().await?;
    assert!(
        !labels.labels().is_empty(),
        "should have at least some labels"
    );

    let built_in: Vec<_> = labels
        .labels()
        .iter()
        .filter(|l| l.label_type() == LabelType::Builtin)
        .collect();

    assert!(
        !built_in.is_empty(),
        "fleet preview should have built-in labels"
    );

    for label in labels.labels() {
        assert!(label.id() > 0, "label should have valid id");
        assert!(!label.name().is_empty(), "label should have name");
    }

    Ok(())
}

#[tokio::test]
async fn label_lifecycle() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();
    let name = format!("Test Label {}", ts);

    // Create
    let create_req = CreateLabelRequest {
        name: name.clone(),
        description: "Integration test label".to_string(),
        query: "SELECT 1".to_string(),
        platform: Some("darwin".to_string()),
    };
    let created = client.labels().create(create_req).await?;
    let label_id = created.label().id();
    assert!(label_id > 0);
    assert_eq!(created.label().name(), name);
    assert_eq!(created.label().description(), "Integration test label");
    assert_eq!(created.label().query(), "SELECT 1");
    assert_eq!(created.label().label_type(), LabelType::Regular);

    // Get
    let fetched = client.labels().get(label_id).await?;
    assert_eq!(fetched.label().id(), label_id);
    assert_eq!(fetched.label().name(), name);

    // Update
    let update_req = UpdateLabelRequest {
        name: Some(format!("Updated Label {}", ts)),
        description: Some("Updated description".to_string()),
    };
    let updated = client.labels().update(label_id, update_req).await?;
    assert_eq!(updated.label().name(), format!("Updated Label {}", ts));
    assert_eq!(updated.label().description(), "Updated description");

    // List should contain our label
    let all = client.labels().list().send().await?;
    assert!(
        all.labels().iter().any(|l| l.id() == label_id),
        "list should contain the created label"
    );

    // Delete — this tests the corrected /labels/id/{id} path
    let deleted = client.labels().delete(label_id).await?;
    let _ = deleted.message();

    // Verify deleted
    let get_after = client.labels().get(label_id).await;
    assert!(get_after.is_err(), "deleted label should not be fetchable");

    Ok(())
}
