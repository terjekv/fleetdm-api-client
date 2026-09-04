//! User endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::models::user::{CreateUserRequest, UpdateUserRequest};

#[tokio::test]
async fn list_users() -> Result<()> {
    let client: FleetClient = live_client!();

    // List all users — should have at least the admin
    let users = client.users().list().per_page(100).send().await?;
    assert!(
        !users.users().is_empty(),
        "should have at least the admin user"
    );

    let admin = users
        .users()
        .iter()
        .find(|u| u.global_role() == Some("admin"));
    assert!(admin.is_some(), "should have an admin user");

    Ok(())
}

#[tokio::test]
async fn list_users_with_query_filter() -> Result<()> {
    let client: FleetClient = live_client!();

    let ts = chrono::Utc::now().timestamp();
    let unique_name = format!("FilterTestUser{}", ts);

    // Create a user with a unique name
    let created = client
        .users()
        .create(CreateUserRequest {
            name: unique_name.clone(),
            email: format!("filter-{}@example.com", ts),
            password: Some("SecurePassword123!".to_string()),
            global_role: Some("observer".to_string()),
            sso_enabled: Some(false),
            api_only: Some(false),
            teams: None,
        })
        .await?;

    // Search by the unique name
    let filtered = client.users().list().query(&unique_name).send().await?;

    assert!(
        filtered.users().iter().any(|u| u.name() == unique_name),
        "query filter should find the user by name"
    );

    // Cleanup
    client.users().delete(created.user().id()).await?;

    Ok(())
}

#[tokio::test]
async fn create_user() -> Result<()> {
    let client: FleetClient = live_client!();

    let create_req = CreateUserRequest {
        name: "Test User".to_string(),
        email: format!("test-{}@example.com", chrono::Utc::now().timestamp()),
        password: Some("SecurePassword123!".to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };

    let created = client.users().create(create_req.clone()).await?;
    assert!(
        created.user().id() > 0,
        "created user should have a valid id"
    );
    assert_eq!(
        created.user().name(),
        create_req.name,
        "created user name should match"
    );
    assert_eq!(
        created.user().email(),
        create_req.email,
        "created user email should match"
    );

    // Clean up
    client.users().delete(created.user().id()).await?;

    Ok(())
}

#[tokio::test]
async fn read_user() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a user to read
    let create_req = CreateUserRequest {
        name: "Read Test User".to_string(),
        email: format!("read-{}@example.com", chrono::Utc::now().timestamp()),
        password: Some("SecurePassword123!".to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };
    let created = client.users().create(create_req.clone()).await?;

    let fetched = client.users().get(created.user().id()).await?;
    assert_eq!(
        fetched.user().id(),
        created.user().id(),
        "fetched user id should match"
    );
    assert_eq!(
        fetched.user().name(),
        create_req.name,
        "fetched user name should match"
    );

    // Clean up
    client.users().delete(created.user().id()).await?;

    Ok(())
}

#[tokio::test]
async fn update_user() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a user to update
    let create_req = CreateUserRequest {
        name: "Update Test User".to_string(),
        email: format!("update-{}@example.com", chrono::Utc::now().timestamp()),
        password: Some("SecurePassword123!".to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };
    let created = client.users().create(create_req.clone()).await?;

    let update_req = UpdateUserRequest {
        name: Some("Updated Test User".to_string()),
        email: None,
        enabled: Some(true),
        global_role: None,
        teams: None,
    };

    let updated = client
        .users()
        .update(created.user().id(), update_req.clone())
        .await?;
    assert_eq!(
        updated.user().name(),
        update_req.name.unwrap(),
        "updated user name should match"
    );

    // Clean up
    client.users().delete(created.user().id()).await?;

    Ok(())
}

#[tokio::test]
async fn delete_user() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a user to delete
    let create_req = CreateUserRequest {
        name: "Delete Test User".to_string(),
        email: format!("delete-{}@example.com", chrono::Utc::now().timestamp()),
        password: Some("SecurePassword123!".to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };
    let created = client.users().create(create_req.clone()).await?;

    client.users().delete(created.user().id()).await?;

    let get_result = client.users().get(created.user().id()).await;
    assert!(get_result.is_err(), "getting deleted user should fail");

    Ok(())
}
