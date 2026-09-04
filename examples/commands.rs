use fleetdm_api_client::{FleetClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Get credentials from environment
    let url = std::env::var("FLEET_URL").expect("FLEET_URL not set");
    let token = std::env::var("FLEET_TOKEN").unwrap_or_else(|_| {
        let _email = std::env::var("FLEET_EMAIL").expect("FLEET_EMAIL not set");
        let _password = std::env::var("FLEET_PASSWORD").expect("FLEET_PASSWORD not set");

        // If no token, we need to login first
        println!("Note: For commands example, set FLEET_TOKEN environment variable");
        println!("Attempting to login with email/password...");

        // This won't work in a sync context, so we'll panic with a helpful message
        panic!("Please set FLEET_TOKEN environment variable or run auth example first");
    });

    let client = FleetClient::builder(&url)?.with_token(&token)?.build();

    let commands = client.commands().list().per_page(10).send().await;

    assert!(commands.is_ok(), "Failed to list commands");
    Ok(())
}
