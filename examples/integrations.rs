//! Example demonstrating Fleet integrations API
//!
//! This example shows how to use the integrations endpoint to interact with
//! APNs, ABM, VPP, SCIM, and Android Enterprise.
//!
//! Run with:
//! ```bash
//! cargo run --example integrations
//! ```

use fleetdm_api_client::{FleetClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Get configuration from environment variables
    let url = std::env::var("FLEET_URL").unwrap_or_else(|_| "https://fleet.example.com".into());
    let email = std::env::var("FLEET_EMAIL").expect("FLEET_EMAIL environment variable required");
    let password =
        std::env::var("FLEET_PASSWORD").expect("FLEET_PASSWORD environment variable required");

    // Create and authenticate client
    let client = FleetClient::builder(url)?
        .login(email, password)
        .await?
        .build();

    println!("=== Fleet Integrations Demo ===\n");

    // Get APNs certificate information
    println!("--- Apple Push Notification service (APNs) ---");
    match client.integrations().get_apns().await {
        Ok(apns) => {
            println!("  Common Name: {}", apns.common_name());
            println!("  Serial Number: {}", apns.serial_number());
            println!("  Issuer: {}", apns.issuer());
            println!("  Renewal Date: {}", apns.renew_date());
        }
        Err(e) => println!("  Error: {} (APNs may not be configured)", e),
    }
    println!();

    // List Apple Business Manager (ABM) tokens
    println!("--- Apple Business Manager (ABM) Tokens ---");
    match client.integrations().list_abm_tokens().await {
        Ok(response) => {
            let tokens = response.abm_tokens();
            if tokens.is_empty() {
                println!("  No ABM tokens configured");
            } else {
                for token in tokens {
                    println!("  Token ID: {}", token.id());
                    println!("  Organization: {}", token.org_name());
                    println!("  Apple ID: {}", token.apple_id());
                    println!("  MDM Server URL: {}", token.mdm_server_url());
                    println!("  Renewal Date: {}", token.renew_date());
                    println!("  Terms Expired: {}", token.terms_expired());
                    if let Some(team) = token.macos_team() {
                        println!("  macOS Team: {} (ID: {})", team.name(), team.id());
                    }
                    if let Some(team) = token.ios_team() {
                        println!("  iOS Team: {} (ID: {})", team.name(), team.id());
                    }
                    if let Some(team) = token.ipados_team() {
                        println!("  iPadOS Team: {} (ID: {})", team.name(), team.id());
                    }
                    println!();
                }
            }
        }
        Err(e) => println!("  Error: {} (Requires Fleet Premium)", e),
    }
    println!();

    // List Volume Purchasing Program (VPP) tokens
    println!("--- Volume Purchasing Program (VPP) Tokens ---");
    match client.integrations().list_vpp_tokens().await {
        Ok(response) => {
            let tokens = response.vpp_tokens();
            if tokens.is_empty() {
                println!("  No VPP tokens configured");
            } else {
                for token in tokens {
                    println!("  Token ID: {}", token.id());
                    println!("  Organization: {}", token.org_name());
                    println!("  Location: {}", token.location());
                    println!("  Renewal Date: {}", token.renew_date());
                    println!("  Teams:");
                    for team in token.teams() {
                        println!("    - {} (ID: {})", team.name(), team.id());
                    }
                    println!();
                }
            }
        }
        Err(e) => println!("  Error: {} (Requires Fleet Premium)", e),
    }
    println!();

    // Get SCIM details
    println!("--- SCIM Identity Provider Details ---");
    match client.integrations().get_scim_details().await {
        Ok(scim) => {
            let last_request = scim.last_request();
            println!("  Last Request:");
            println!("    Requested At: {}", last_request.requested_at());
            println!("    Status: {}", last_request.status());
            if !last_request.details().is_empty() {
                println!("    Details: {}", last_request.details());
            }
        }
        Err(e) => println!("  Error: {} (SCIM may not be configured)", e),
    }
    println!();

    // Get Android Enterprise information
    println!("--- Android Enterprise ---");
    match client.integrations().get_android_enterprise().await {
        Ok(android) => {
            println!("  Enterprise ID: {}", android.android_enterprise_id());
        }
        Err(e) => println!("  Error: {} (Android Enterprise may not be configured)", e),
    }
    println!();

    Ok(())
}
