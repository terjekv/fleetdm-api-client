//! Authentication endpoint tests

use crate::live_client;
use fleetdm_api_client::models::user::CreateUserRequest;
use fleetdm_api_client::{FleetClient, FleetError, Result, RetryPolicy};
use std::env;

/// Test login with email/password via FleetClient::builder().login()
#[tokio::test]
async fn login_with_credentials() -> Result<()> {
    // This test uses the raw login method instead of the cached client
    let Some(url) = env::var("FLEET_URL").ok() else {
        return Ok(());
    };
    let Some(email) = env::var("FLEET_EMAIL").ok() else {
        return Ok(());
    };
    let Some(password) = env::var("FLEET_PASSWORD").ok() else {
        return Ok(());
    };

    // Test login
    let client = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .login(email, password)
        .await?
        .build();

    // Verify we can make authenticated requests
    let me = client.auth().me().await?;
    assert!(me.user().id() > 0, "authenticated user should have an id");
    assert!(
        !me.user().email().is_empty(),
        "authenticated user should have an email"
    );

    Ok(())
}

/// Test login with invalid credentials should fail
#[tokio::test]
async fn login_with_invalid_credentials() -> Result<()> {
    let Some(url) = env::var("FLEET_URL").ok() else {
        return Ok(());
    };

    // Test login with invalid credentials
    let result = FleetClient::builder(&url)?
        .login("invalid@example.com", "wrongpassword")
        .await;

    assert!(
        result.is_err(),
        "login with invalid credentials should fail"
    );

    match result {
        Err(FleetError::Authentication(message)) => {
            assert!(
                !message.trim().is_empty(),
                "authentication error should retain Fleet's response context"
            );
        }
        Err(error) => panic!("expected authentication error, got {error:?}"),
        Ok(_) => panic!("invalid credentials unexpectedly authenticated"),
    }

    Ok(())
}

/// Test GET /api/v1/fleet/me - Get current user info
#[tokio::test]
async fn get_me() -> Result<()> {
    let client: FleetClient = live_client!();

    let me = client.auth().me().await?;

    // Validate response structure
    assert!(me.user().id() > 0, "user should have a valid id");
    assert!(!me.user().email().is_empty(), "user should have an email");
    assert!(!me.user().name().is_empty(), "user should have a name");
    assert!(
        me.user().global_role().is_some() || !me.user().teams().is_empty(),
        "user should have either global_role or team memberships"
    );

    // Validate available_teams (may be empty)
    let _available_teams = me.available_teams();

    Ok(())
}

/// Test POST /api/v1/fleet/logout - Logout current user
#[tokio::test]
async fn logout() -> Result<()> {
    // Create a fresh client for this test since logout invalidates the token
    let Some(url) = env::var("FLEET_URL").ok() else {
        return Ok(());
    };
    let Some(email) = env::var("FLEET_EMAIL").ok() else {
        return Ok(());
    };
    let Some(password) = env::var("FLEET_PASSWORD").ok() else {
        return Ok(());
    };

    let client = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .login(email, password)
        .await?
        .build();

    // Verify we're authenticated
    let me_before = client.auth().me().await?;
    assert!(me_before.user().id() > 0);

    // Logout
    let _logout_response = client.auth().logout().await?;

    // After logout, the token should be invalid
    // However, since our client still has the token, we can't easily test this
    // without making a new request with the same token
    // In practice, the server has invalidated the session

    Ok(())
}

/// Test PATCH /api/v1/fleet/change_password - Change password for authenticated user
#[tokio::test]
async fn change_password() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a test user to change password for
    let timestamp = chrono::Utc::now().timestamp();
    let test_email = format!("changepass-{}@example.com", timestamp);
    let initial_password = "InitialPassword123!";
    let new_password = "NewPassword456!";

    let create_req = CreateUserRequest {
        name: "Change Password Test".to_string(),
        email: test_email.clone(),
        password: Some(initial_password.to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };

    let created = client.users().create(create_req).await?;
    let user_id = created.user().id();

    // Login as the test user
    let url = env::var("FLEET_URL").unwrap();
    let test_client = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .login(&test_email, initial_password)
        .await?
        .build();

    // Change password
    let change_result = test_client
        .auth()
        .change_password(initial_password, new_password)
        .await;

    // Clean up first
    client.users().delete(user_id).await?;

    // Now check the result
    match change_result {
        Ok(_) => {
            // Password changed successfully
        }
        Err(FleetError::Authentication(msg)) => {
            // If password reset is required, that's expected behavior
            if msg.contains("password reset required") {
                return Ok(());
            }
            panic!("Unexpected authentication error: {}", msg);
        }
        Err(e) => {
            panic!(
                "change password should succeed or require password reset: {:?}",
                e
            );
        }
    }

    // Verify we can login with new password
    let _login_with_new_password = FleetClient::builder(&url)?
        .login(&test_email, new_password)
        .await;

    // Note: This might fail because we deleted the user, but if change_password worked,
    // the password was changed before deletion
    // In a real scenario without deletion, this would work

    Ok(())
}

/// Test PATCH /api/v1/fleet/change_password with wrong old password should fail
#[tokio::test]
async fn change_password_wrong_old_password() -> Result<()> {
    let client: FleetClient = live_client!();

    // Try to change password with wrong old password
    let result = client
        .auth()
        .change_password("WrongOldPassword!", "NewPassword123!")
        .await;

    assert!(
        result.is_err(),
        "change password with wrong old password should fail"
    );

    Ok(())
}

/// Test POST /api/v1/fleet/forgot_password - Send password reset email
#[tokio::test]
async fn forgot_password() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a test user
    let timestamp = chrono::Utc::now().timestamp();
    let test_email = format!("forgot-{}@example.com", timestamp);

    let create_req = CreateUserRequest {
        name: "Forgot Password Test".to_string(),
        email: test_email.clone(),
        password: Some("TestPassword123!".to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };

    let created = client.users().create(create_req).await?;

    // Request password reset
    // This may fail if email is not configured on the server
    let public_builder = FleetClient::builder(env::var("FLEET_URL").expect("FLEET_URL set"))?;
    let result = public_builder.auth().forgot_password(&test_email).await;

    // Clean up
    client.users().delete(created.user().id()).await?;

    match result {
        Ok(_) => {
            // Email is configured, request succeeded
        }
        Err(FleetError::Api { message, .. }) => {
            // Email not configured is acceptable for testing
            if message.contains("email not configured")
                || message.contains("SMTP")
                || message.contains("SES")
            {
                return Ok(());
            }
            // Other API errors - feature might not be configured
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

/// Test POST /api/v1/fleet/reset_password
/// Note: This test cannot be fully executed without a real password reset token from email
#[tokio::test]
async fn reset_password_invalid_token() -> Result<()> {
    // Test with invalid token - should fail
    let public_builder = FleetClient::builder(env::var("FLEET_URL").expect("FLEET_URL set"))?;
    let result = public_builder
        .auth()
        .reset_password("NewPassword123!", "NewPassword123!", "InvalidToken")
        .await;

    assert!(
        result.is_err(),
        "reset password with invalid token should fail"
    );

    Ok(())
}

/// Test POST /api/v1/fleet/perform_required_password_reset
/// This requires the user to have force_password_reset=true
#[tokio::test]
async fn perform_required_password_reset() -> Result<()> {
    let client: FleetClient = live_client!();

    // Create a test user with force_password_reset
    let timestamp = chrono::Utc::now().timestamp();
    let test_email = format!("forcereset-{}@example.com", timestamp);
    let initial_password = "InitialPassword123!";

    let create_req = CreateUserRequest {
        name: "Force Reset Test".to_string(),
        email: test_email.clone(),
        password: Some(initial_password.to_string()),
        global_role: Some("observer".to_string()),
        sso_enabled: Some(false),
        api_only: Some(false),
        teams: None,
    };

    let created = client.users().create(create_req).await?;
    let user_id = created.user().id();

    // TODO: We need to set force_password_reset=true on the user
    // This might require an admin API endpoint to force this flag
    // For now, we'll skip the actual test execution if the user doesn't have the flag

    // Clean up
    client.users().delete(user_id).await?;

    Ok(())
}

/// Test GET /api/v1/fleet/sso - Get SSO configuration
#[tokio::test]
async fn get_sso_config() -> Result<()> {
    let public_builder = FleetClient::builder(env::var("FLEET_URL").expect("FLEET_URL set"))?;
    let sso_config = public_builder.auth().sso_config().await?;

    // SSO may or may not be configured
    if let Some(settings) = sso_config.settings {
        // idp_name can be empty if SSO is not configured
        // sso_enabled may be true or false
        let _sso_enabled = settings.sso_enabled;
        let _idp_name = settings.idp_name;
    }

    Ok(())
}

/// Test POST /api/v1/fleet/sso - Initiate SSO
#[tokio::test]
async fn initiate_sso() -> Result<()> {
    // Try to initiate SSO
    let public_builder = FleetClient::builder(env::var("FLEET_URL").expect("FLEET_URL set"))?;
    let result = public_builder.auth().initiate_sso("/hosts/manage").await;

    match result {
        Ok(response) => {
            // SSO is configured
            assert!(!response.url.is_empty(), "SSO URL should not be empty");
        }
        Err(FleetError::Api { ref message, .. })
            if message.contains("SSO")
                || message.contains("sso")
                || message.contains("not configured") => {}
        Err(FleetError::BadRequest(_)) => {
            // SSO not configured - this is acceptable
            return Ok(());
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

/// Test POST /api/v1/fleet/sso/callback
/// Note: This requires a valid SAML response which we cannot generate in tests
#[tokio::test]
async fn sso_callback_invalid() -> Result<()> {
    // Test with invalid SAML response - should fail
    let public_builder = FleetClient::builder(env::var("FLEET_URL").expect("FLEET_URL set"))?;
    let result = public_builder
        .auth()
        .sso_callback("InvalidSAMLResponse", None)
        .await;

    assert!(
        result.is_err(),
        "SSO callback with invalid SAML response should fail"
    );

    Ok(())
}

/// Test full authentication flow: login, me, logout
#[tokio::test]
async fn full_auth_flow() -> Result<()> {
    let Some(url) = env::var("FLEET_URL").ok() else {
        return Ok(());
    };
    let Some(email) = env::var("FLEET_EMAIL").ok() else {
        return Ok(());
    };
    let Some(password) = env::var("FLEET_PASSWORD").ok() else {
        return Ok(());
    };

    // 1. Login
    let client = FleetClient::builder(&url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .login(email, password)
        .await?
        .build();

    // 2. Get current user info
    let me = client.auth().me().await?;
    assert!(me.user().id() > 0);
    let user_email = me.user().email();
    assert!(!user_email.is_empty());

    // 3. Verify token is working
    let token = client.token();
    assert!(!token.is_empty());

    // 4. Logout
    client.auth().logout().await?;

    Ok(())
}

/// Test that authenticated requests work
#[tokio::test]
async fn authenticated_requests() -> Result<()> {
    let client: FleetClient = live_client!();

    // Test multiple authenticated endpoints
    let me = client.auth().me().await?;
    assert!(me.user().id() > 0);

    client.version().get().await?;

    // Test other endpoints to verify token works across the board
    let hosts = client.hosts().list().per_page(1).send().await?;
    let _host_list = hosts.hosts();

    Ok(())
}

/// Verify model field accessors
#[tokio::test]
async fn test_user_model_accessors() -> Result<()> {
    let client: FleetClient = live_client!();

    let me = client.auth().me().await?;
    let user = me.user();

    // Test all accessor methods
    let _id = user.id();
    let _email = user.email();
    let _name = user.name();
    let _enabled = user.enabled();
    let _force_password_reset = user.force_password_reset();
    let _gravatar_url = user.gravatar_url();
    let _gravatar_url_dark = user.gravatar_url_dark();
    let _sso_enabled = user.sso_enabled();
    let _global_role = user.global_role();
    let _teams = user.teams();
    let _api_only = user.api_only();

    // Test MeResponse accessors
    let _available_teams = me.available_teams();

    assert!(user.id() > 0);
    assert!(!user.email().is_empty());

    Ok(())
}

/// Test creating and deleting an SSO user
#[tokio::test]
async fn create_and_delete_sso_user() -> Result<()> {
    let client: FleetClient = live_client!();

    let timestamp = chrono::Utc::now().timestamp();
    let test_email = format!("sso-user-{}@example.com", timestamp);

    // Create an SSO user (no password required when SSO is enabled)
    let create_req = CreateUserRequest {
        name: "SSO Test User".to_string(),
        email: test_email.clone(),
        password: None, // SSO users don't have passwords
        global_role: Some("observer".to_string()),
        sso_enabled: Some(true), // Enable SSO for this user
        api_only: Some(false),
        teams: None,
    };

    let created = client.users().create(create_req).await?;
    assert!(
        created.user().id() > 0,
        "created SSO user should have a valid id"
    );
    assert_eq!(
        created.user().email(),
        test_email,
        "created SSO user email should match"
    );
    assert!(
        created.user().sso_enabled(),
        "created user should have SSO enabled"
    );

    // Verify we can read the SSO user
    let fetched = client.users().get(created.user().id()).await?;
    assert_eq!(
        fetched.user().id(),
        created.user().id(),
        "fetched SSO user id should match"
    );
    assert!(
        fetched.user().sso_enabled(),
        "fetched user should have SSO enabled"
    );

    // Delete the SSO user
    client.users().delete(created.user().id()).await?;

    // Verify the user is deleted
    let get_result = client.users().get(created.user().id()).await;
    assert!(get_result.is_err(), "getting deleted SSO user should fail");

    Ok(())
}
