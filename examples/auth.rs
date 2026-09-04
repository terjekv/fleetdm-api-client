use fleetdm_api_client::{FleetClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Get credentials from environment
    let url = std::env::var("FLEET_URL").expect("FLEET_URL not set");
    let email = std::env::var("FLEET_EMAIL").expect("FLEET_EMAIL not set");
    let password = std::env::var("FLEET_PASSWORD").expect("FLEET_PASSWORD not set");

    println!("Connecting to Fleet at: {}", url);

    // Login to get a client
    let client = FleetClient::builder(&url)?
        .login(&email, &password)
        .await?
        .build();

    println!("✓ Successfully logged in");

    // Get current user information
    let me = client.auth().me().await?;
    println!("\n=== Current User ===");
    println!("ID: {}", me.user().id());
    println!("Email: {}", me.user().email());
    println!("Name: {}", me.user().name());
    println!("Enabled: {}", me.user().enabled());
    println!("SSO Enabled: {}", me.user().sso_enabled());
    println!("Global Role: {:?}", me.user().global_role());
    println!("API Only: {}", me.user().api_only());

    if !me.available_teams().is_empty() {
        println!("\nAvailable Teams:");
        for team in me.available_teams() {
            println!("  - {} (ID: {})", team.name, team.id);
        }
    }

    if !me.user().teams().is_empty() {
        println!("\nUser Teams:");
        for team in me.user().teams() {
            println!("  - {} (ID: {}, Role: {})", team.name, team.id, team.role);
        }
    }

    // Check SSO configuration
    let public_builder = FleetClient::builder(&url)?;
    let sso_config = public_builder.auth().sso_config().await?;
    if let Some(settings) = sso_config.settings {
        println!("\n=== SSO Configuration ===");
        println!("IDP Name: {}", settings.idp_name);
        println!("SSO Enabled: {}", settings.sso_enabled);
        if let Some(image_url) = settings.idp_image_url {
            println!("IDP Image URL: {}", image_url);
        }
    } else {
        println!("\n=== SSO Configuration ===");
        println!("SSO is not configured");
    }

    // Note: Uncomment these to test other auth operations
    // WARNING: These will affect your session/account!

    // Change password (requires old password)
    // client.auth().change_password("old_pass", "new_pass").await?;

    // Logout
    // client.auth().logout().await?;
    // println!("\n✓ Successfully logged out");

    Ok(())
}
