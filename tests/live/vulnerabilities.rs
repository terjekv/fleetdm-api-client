//! Vulnerability endpoint tests

use crate::live_client;
use fleetdm_api_client::Result;
use fleetdm_api_client::{FleetClient, FleetError};

#[tokio::test]
async fn list_vulnerabilities() -> Result<()> {
    let client: FleetClient = live_client!();

    let vulnerabilities = client.vulnerabilities().list().per_page(50).send().await?;

    let vulns = vulnerabilities.vulnerabilities();
    assert!(vulns.len() <= 50, "should respect per_page limit");
    for vuln in vulns.iter().take(3) {
        assert!(
            !vuln.cve().is_empty(),
            "each vulnerability should have a CVE"
        );
    }

    Ok(())
}

#[tokio::test]
async fn list_vulnerabilities_with_exploit_filter() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client
        .vulnerabilities()
        .list()
        .exploit(true)
        .per_page(50)
        .send()
        .await;

    // Exploit filter requires Fleet Premium license
    match result {
        Ok(vulnerabilities) => {
            let vulns = vulnerabilities.vulnerabilities();
            assert!(vulns.len() <= 50, "should respect per_page limit");
        }
        Err(FleetError::PremiumRequired { .. }) => {
            // Premium license required - that's acceptable
            return Ok(());
        }
        Err(e) => return Err(e),
    }

    Ok(())
}
