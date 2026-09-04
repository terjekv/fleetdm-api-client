//! Team endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;
use fleetdm_api_client::models::fleet::{CreateFleetRequest, UpdateFleetRequest};

#[tokio::test]
async fn list_teams() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.fleets().list().per_page(100).send().await;

    // Teams require Fleet Premium license
    match result {
        Ok(teams) => {
            assert!(teams.fleets().len() <= 100, "should respect per_page limit");
            for team in teams.fleets() {
                assert!(team.id() > 0, "each team should have a valid id");
                assert!(!team.name().is_empty(), "each team should have a name");
            }
        }
        Err(FleetError::PremiumRequired { .. }) => {
            // Premium license required - that's acceptable
            return Ok(());
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

#[tokio::test]
async fn create_team() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreateFleetRequest {
        name: format!("Test Team {}", chrono::Utc::now().timestamp()),
    };

    let created = client.fleets().create(create_req.clone()).await;

    // Teams require Fleet Premium license
    let created = match created {
        Ok(c) => c,
        Err(FleetError::PremiumRequired { .. }) => {
            // Premium license required - that's acceptable
            return Ok(());
        }
        Err(e) => return Err(e),
    };

    assert!(
        created.fleet().id() > 0,
        "created team should have a valid id"
    );
    assert_eq!(
        created.fleet().name(),
        create_req.name,
        "created team name should match"
    );

    // Clean up
    client.fleets().delete(created.fleet().id()).await?;

    Ok(())
}

#[tokio::test]
async fn read_team() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreateFleetRequest {
        name: format!("Read Team {}", chrono::Utc::now().timestamp()),
    };

    let created = client.fleets().create(create_req.clone()).await;

    let created = match created {
        Ok(c) => c,
        Err(FleetError::PremiumRequired { .. }) => return Ok(()),
        Err(e) => return Err(e),
    };

    let fetched = client.fleets().get(created.fleet().id()).await?;
    assert_eq!(
        fetched.fleet().id(),
        created.fleet().id(),
        "fetched team id should match"
    );
    assert_eq!(
        fetched.fleet().name(),
        create_req.name,
        "fetched team name should match"
    );

    // Clean up
    client.fleets().delete(created.fleet().id()).await?;

    Ok(())
}

#[tokio::test]
async fn update_team() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreateFleetRequest {
        name: format!("Update Team {}", chrono::Utc::now().timestamp()),
    };

    let created = client.fleets().create(create_req.clone()).await;

    let created = match created {
        Ok(c) => c,
        Err(FleetError::PremiumRequired { .. }) => return Ok(()),
        Err(e) => return Err(e),
    };

    let update_req = UpdateFleetRequest {
        name: Some(format!("Updated Team {}", chrono::Utc::now().timestamp())),
        ..Default::default()
    };

    let updated = client
        .fleets()
        .update(created.fleet().id(), update_req.clone())
        .await?;
    assert_eq!(
        updated.fleet().name(),
        update_req.name.clone().unwrap(),
        "updated team name should match"
    );

    // Clean up
    client.fleets().delete(created.fleet().id()).await?;

    Ok(())
}

#[tokio::test]
async fn delete_team() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreateFleetRequest {
        name: format!("Delete Team {}", chrono::Utc::now().timestamp()),
    };

    let created = client.fleets().create(create_req.clone()).await;

    let created = match created {
        Ok(c) => c,
        Err(FleetError::PremiumRequired { .. }) => return Ok(()),
        Err(e) => return Err(e),
    };

    client.fleets().delete(created.fleet().id()).await?;

    let get_result = client.fleets().get(created.fleet().id()).await;
    assert!(get_result.is_err(), "getting deleted team should fail");

    Ok(())
}
