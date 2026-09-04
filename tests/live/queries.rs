//! Report endpoint tests

use crate::live_client;
use fleetdm_api_client::models::report::{CreateReportRequest, UpdateReportRequest};
use fleetdm_api_client::{FleetClient, FleetError, Result};

#[tokio::test]
async fn list_reports_smoke() -> Result<()> {
    let client: FleetClient = live_client!();

    let reports = client.reports().list().per_page(10).send().await?;
    assert!(
        reports.reports().len() <= 10,
        "should respect per_page limit"
    );

    for report in reports.reports() {
        assert!(report.id() > 0, "each report should have an id");
        assert!(!report.name().is_empty(), "each report should have a name");
    }

    Ok(())
}

#[tokio::test]
async fn report_lifecycle() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();
    let name = format!("Test Report {}", ts);

    let created = client
        .reports()
        .create(CreateReportRequest {
            name: name.clone(),
            query: "SELECT * FROM uptime".to_string(),
            description: Some("Integration test report".to_string()),
            fleet_id: None,
            interval: Some(3600),
            platform: Some("darwin,linux".to_string()),
            discard_data: Some(false),
            ..Default::default()
        })
        .await?;
    let report_id = created.report().id();
    assert!(report_id > 0);
    assert_eq!(created.report().name(), name);
    assert_eq!(created.report().query(), "SELECT * FROM uptime");
    assert_eq!(created.report().description(), "Integration test report");
    assert_eq!(created.report().interval(), Some(3600));
    assert_eq!(created.report().platform(), Some("darwin,linux"));

    let fetched = client.reports().get(report_id).await?;
    assert_eq!(fetched.report().id(), report_id);
    assert_eq!(fetched.report().name(), name);

    let updated = client
        .reports()
        .update(
            report_id,
            UpdateReportRequest {
                name: Some(format!("Updated Report {}", ts)),
                query: Some("SELECT * FROM os_version".to_string()),
                description: Some("Updated description".to_string()),
                interval: Some(7200),
                platform: None,
                ..Default::default()
            },
        )
        .await?;
    assert_eq!(updated.report().name(), format!("Updated Report {}", ts));
    assert_eq!(updated.report().query(), "SELECT * FROM os_version");
    assert_eq!(updated.report().interval(), Some(7200));

    let all = client.reports().list().per_page(1000).send().await?;
    assert!(
        all.reports().iter().any(|q| q.id() == report_id),
        "list should contain the created report"
    );

    client.reports().delete(report_id).await?;

    let get_after = client.reports().get(report_id).await;
    assert!(get_after.is_err(), "deleted report should not be fetchable");

    Ok(())
}

#[tokio::test]
async fn report_batch_delete() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();

    let report_1 = client
        .reports()
        .create(CreateReportRequest {
            name: format!("Batch Del 1 {}", ts),
            query: "SELECT 1".to_string(),
            description: None,
            fleet_id: None,
            interval: None,
            platform: None,
            ..Default::default()
        })
        .await?;
    let report_2 = client
        .reports()
        .create(CreateReportRequest {
            name: format!("Batch Del 2 {}", ts),
            query: "SELECT 2".to_string(),
            description: None,
            fleet_id: None,
            interval: None,
            platform: None,
            ..Default::default()
        })
        .await?;

    let id1 = report_1.report().id();
    let id2 = report_2.report().id();

    client.reports().batch_delete(vec![id1, id2]).await?;

    assert!(client.reports().get(id1).await.is_err());
    assert!(client.reports().get(id2).await.is_err());

    Ok(())
}

#[tokio::test]
async fn report_data() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();
    let created = client
        .reports()
        .create(CreateReportRequest {
            name: format!("Report Data Test {}", ts),
            query: "SELECT * FROM uptime".to_string(),
            description: None,
            fleet_id: None,
            interval: Some(0),
            platform: None,
            discard_data: Some(false),
            ..Default::default()
        })
        .await?;
    let report_id = created.report().id();

    let data_result = client.reports().report_data(report_id).await;
    match data_result {
        Ok(data) => {
            let _ = data.body();
        }
        Err(FleetError::NotFound(_)) => {
            // Some Fleet versions return 404 until report data exists.
        }
        Err(error) => return Err(error),
    }

    client.reports().delete(report_id).await?;

    Ok(())
}
