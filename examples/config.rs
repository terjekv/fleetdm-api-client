use fleetdm_api_client::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let url = env::var("FLEET_URL").expect("FLEET_URL must be set");
    let email = env::var("FLEET_EMAIL").expect("FLEET_EMAIL must be set");
    let password = env::var("FLEET_PASSWORD").expect("FLEET_PASSWORD must be set");

    // Authenticate
    let client = fleetdm_api_client::FleetClient::builder(&url)?
        .login(email, password)
        .await?
        .build();

    // Get configuration
    let config = client.config().get().await?;

    assert!(
        config.org_info().org_name() != "",
        "Org name should not be empty"
    );

    Ok(())
}
