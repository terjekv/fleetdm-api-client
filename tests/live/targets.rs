//! Target endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::models::target::SearchTargetsRequest;

#[tokio::test]
async fn search_targets() -> Result<()> {
    let client: FleetClient = live_client!();

    let search_req = SearchTargetsRequest {
        query: "".to_string(),
        query_id: None,
        selected: None,
        include_observer: Some(true),
    };

    let result = client.targets().search(&search_req).await?;

    // Targets search returns hosts, labels, and fleets.
    assert!(
        !result.targets().hosts().is_empty()
            || !result.targets().labels().is_empty()
            || !result.targets().fleets().is_empty(),
        "should have at least one target type"
    );

    Ok(())
}
