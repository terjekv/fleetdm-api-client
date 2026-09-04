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

    // List vulnerabilities
    println!("===== Vulnerabilities =====");
    let vulns = client.vulnerabilities().list().per_page(5).send().await?;
    println!("Found {} vulnerabilities (showing 5)", vulns.count());

    let vuln_list = vulns.vulnerabilities();
    for vuln in vuln_list.iter().take(5) {
        println!(
            "  - {} (CVSS: {:.1}, Hosts: {})",
            vuln.cve(),
            vuln.cvss_score().unwrap_or(0.0),
            vuln.hosts_count()
        );
    }
    println!();

    // If we have any vulnerabilities, get details on the first one
    if let Some(first_vuln) = vulns.vulnerabilities().first() {
        println!("Getting details for {}...", first_vuln.cve());
        match client.vulnerabilities().get(first_vuln.cve(), None).await {
            Ok(Some(details)) => {
                println!(
                    "  Description: {}",
                    details.cve_description().unwrap_or("N/A")
                );
                let os_versions = details.os_versions();
                println!("  Affects {} OS versions", os_versions.len());
                let software = details.software();
                println!("  Affects {} software packages", software.len());
            }
            Ok(None) => println!("  Fleet has no affected software or OS versions"),
            Err(e) => {
                println!("  Could not get details: {}", e);
            }
        }
        println!();
    }

    Ok(())
}
