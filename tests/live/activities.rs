//! Activity endpoint tests

use crate::live_client;

use fleetdm_api_client::models::activity::ActivityOrderKey;
use fleetdm_api_client::models::common::OrderDirection;
use fleetdm_api_client::{FleetClient, Result};

#[tokio::test]
async fn list_activities() -> Result<()> {
    let client: FleetClient = live_client!();

    let activities = client.activities().list().per_page(10).send().await?;
    assert!(
        activities.activities().len() <= 10,
        "should respect per_page limit"
    );

    for activity in activities.activities() {
        assert!(
            !activity.r#type().is_empty(),
            "each activity should have a type"
        );
    }

    Ok(())
}

#[tokio::test]
async fn activity_ordering() -> Result<()> {
    let client: FleetClient = live_client!();

    // Test ordering by created_at (the documented and supported order key)
    let result = client
        .activities()
        .list()
        .per_page(5)
        .order_key(ActivityOrderKey::CreatedAt)
        .order_direction(OrderDirection::Desc)
        .send()
        .await?;

    assert!(
        result.activities().len() <= 5,
        "should respect per_page limit"
    );

    if result.activities().len() >= 2 {
        for i in 0..result.activities().len() - 1 {
            let current = &result.activities()[i];
            let next = &result.activities()[i + 1];
            assert!(
                current.created_at() >= next.created_at(),
                "activities should be sorted by created_at desc"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn sorting_with_pagination() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client
        .activities()
        .list()
        .per_page(3)
        .page(0)
        .order_key(ActivityOrderKey::CreatedAt)
        .order_direction(OrderDirection::Desc)
        .send()
        .await?;

    assert!(
        result.activities().len() <= 3,
        "should respect per_page limit"
    );

    if result.activities().len() > 1 {
        for i in 0..result.activities().len() - 1 {
            let current_time = result.activities()[i].created_at();
            let next_time = result.activities()[i + 1].created_at();
            assert!(
                current_time >= next_time,
                "activities should be sorted by created_at desc"
            );
        }
    }

    Ok(())
}
