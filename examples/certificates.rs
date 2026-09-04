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

    println!("Successfully authenticated to Fleet");
    println!();

    // List certificate authorities
    println!("===== Certificates =====");
    match client.certificates().list_ca().await {
        Ok(cas) => {
            println!(
                "Found {} certificate authorities",
                cas.certificate_authorities().len()
            );
            for ca in cas.certificate_authorities() {
                println!(
                    "  - CA #{}: {} (type: {})",
                    ca.id(),
                    ca.name(),
                    ca.ca_type()
                );
            }
        }
        Err(e) => {
            println!("Certificate authorities may not be configured: {}", e);
        }
    }
    println!();

    Ok(())
}
