//! Host endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::models::common::OrderDirection;
use fleetdm_api_client::models::host::{HostOrderKey, HostStatus};

#[tokio::test]
async fn list_hosts_smoke() -> Result<()> {
    let client: FleetClient = live_client!();

    let hosts = client.hosts().list().per_page(10).send().await?;
    assert!(hosts.hosts().len() <= 10, "should respect per_page limit");
    assert!(
        !hosts.hosts().is_empty(),
        "fleet preview should have enrolled hosts"
    );

    for host in hosts.hosts() {
        assert!(host.id() > 0, "each host should have an id");
        assert!(
            !host.hostname().is_empty(),
            "each host should have a hostname"
        );
        assert!(
            !host.platform().is_empty(),
            "each host should have a platform"
        );
    }

    Ok(())
}

#[tokio::test]
async fn host_status_filter() -> Result<()> {
    let client: FleetClient = live_client!();

    let all_hosts = client.hosts().list().per_page(1000).send().await?;
    let total_count = all_hosts.hosts().len();
    assert!(total_count > 0, "should have hosts");

    for status in [
        HostStatus::Online,
        HostStatus::Offline,
        HostStatus::New,
        HostStatus::Mia,
        HostStatus::Missing,
    ] {
        let result = client
            .hosts()
            .list()
            .status(status)
            .per_page(1000)
            .send()
            .await?;

        let count = result.hosts().len();
        assert!(
            count <= total_count,
            "filtered host count should not exceed total host count"
        );

        for host in result.hosts() {
            if status != HostStatus::New {
                assert_eq!(host.status(), status, "host status should match filter");
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn pagination_hosts() -> Result<()> {
    let client: FleetClient = live_client!();

    let page1 = client.hosts().list().page(0).per_page(5).send().await?;
    assert!(
        page1.hosts().len() <= 5,
        "first page should have at most 5 hosts"
    );
    assert!(
        !page1.hosts().is_empty(),
        "first page should have at least 1 host"
    );

    let page2 = client.hosts().list().page(1).per_page(5).send().await?;
    assert!(
        page2.hosts().len() <= 5,
        "second page should have at most 5 hosts"
    );

    if !page2.hosts().is_empty() {
        let page1_ids: Vec<u64> = page1.hosts().iter().map(|h| h.id()).collect();
        let page2_ids: Vec<u64> = page2.hosts().iter().map(|h| h.id()).collect();
        let overlap = page1_ids.iter().any(|id| page2_ids.contains(id));
        assert!(!overlap, "pages should not have overlapping hosts");
    }

    Ok(())
}

#[tokio::test]
async fn sorting_hosts() -> Result<()> {
    let client: FleetClient = live_client!();

    let asc = client
        .hosts()
        .list()
        .order_key(HostOrderKey::Hostname)
        .order_direction(OrderDirection::Asc)
        .per_page(50)
        .send()
        .await?;

    let desc = client
        .hosts()
        .list()
        .order_key(HostOrderKey::Hostname)
        .order_direction(OrderDirection::Desc)
        .per_page(50)
        .send()
        .await?;

    assert!(!asc.hosts().is_empty(), "should have hosts for sorting");
    assert!(!desc.hosts().is_empty(), "should have hosts for sorting");

    if asc.hosts().len() > 1 {
        let asc_names: Vec<String> = asc
            .hosts()
            .iter()
            .map(|h| h.hostname().to_lowercase())
            .collect();
        let mut sorted_asc = asc_names.clone();
        sorted_asc.sort();
        assert_eq!(asc_names, sorted_asc, "ascending results should be sorted");
    }

    if desc.hosts().len() > 1 {
        let desc_names: Vec<String> = desc
            .hosts()
            .iter()
            .map(|h| h.hostname().to_lowercase())
            .collect();
        let mut sorted_desc = desc_names.clone();
        sorted_desc.sort();
        sorted_desc.reverse();
        assert_eq!(
            desc_names, sorted_desc,
            "descending results should be reverse sorted"
        );
    }

    Ok(())
}

#[tokio::test]
async fn get_host_by_id() -> Result<()> {
    let client: FleetClient = live_client!();

    let hosts = client.hosts().list().per_page(1).send().await?;
    assert!(!hosts.hosts().is_empty(), "need at least one host");

    let host_id = hosts.hosts()[0].id();
    let detail = client.hosts().get(host_id).await?;

    assert_eq!(detail.host().id(), host_id);
    assert!(!detail.host().hostname().is_empty());
    assert!(!detail.host().uuid().is_empty());
    assert!(!detail.host().platform().is_empty());

    Ok(())
}

#[tokio::test]
async fn get_host_by_identifier() -> Result<()> {
    let client: FleetClient = live_client!();

    // Get a host to look up by identifier
    let hosts = client.hosts().list().per_page(1).send().await?;
    assert!(!hosts.hosts().is_empty(), "need at least one host");

    let expected_id = hosts.hosts()[0].id();
    let hostname = hosts.hosts()[0].hostname().to_string();

    // Look up by hostname
    let result = client.hosts().get_by_identifier(&hostname).await?;
    assert_eq!(result.host().id(), expected_id);
    assert_eq!(result.host().hostname(), hostname);

    Ok(())
}

#[tokio::test]
async fn host_count() -> Result<()> {
    let client: FleetClient = live_client!();

    let count_resp = client.hosts().count().send().await?;
    assert!(
        count_resp.count() > 0,
        "fleet preview should have enrolled hosts"
    );

    // Verify count matches list
    let all_hosts = client.hosts().list().per_page(1000).send().await?;
    assert_eq!(
        count_resp.count() as usize,
        all_hosts.hosts().len(),
        "count endpoint should match listed hosts"
    );

    Ok(())
}

#[tokio::test]
async fn host_count_with_filter() -> Result<()> {
    let client: FleetClient = live_client!();

    let total = client.hosts().count().send().await?;
    let online = client
        .hosts()
        .count()
        .status(HostStatus::Online)
        .send()
        .await?;

    assert!(
        online.count() <= total.count(),
        "online count should not exceed total"
    );

    Ok(())
}

#[tokio::test]
async fn host_summary() -> Result<()> {
    let client: FleetClient = live_client!();

    let summary = client.hosts().summary(None, None).await?;

    assert!(
        summary.totals_hosts_count() > 0,
        "fleet preview should have hosts in summary"
    );

    // Status counts should add up reasonably
    let status_sum = summary.online_count()
        + summary.offline_count()
        + summary.mia_count()
        + summary.new_count();
    assert!(
        status_sum >= summary.totals_hosts_count(),
        "status counts should cover all hosts (a host can be both online and new)"
    );

    // Platform breakdown should be present
    assert!(
        !summary.platforms().is_empty(),
        "should have platform breakdown"
    );

    let platform_total: u64 = summary.platforms().iter().map(|p| p.hosts_count()).sum();
    assert_eq!(
        platform_total,
        summary.totals_hosts_count(),
        "platform counts should sum to total"
    );

    for platform in summary.platforms() {
        assert!(
            !platform.platform().is_empty(),
            "platform name should be set"
        );
        assert!(
            platform.hosts_count() > 0,
            "each platform should have hosts"
        );
    }

    Ok(())
}

#[tokio::test]
async fn host_mdm_lock_unlock_wipe() -> Result<()> {
    let client: FleetClient = live_client!();

    // Get a host to test MDM actions on
    let hosts = client.hosts().list().per_page(1).send().await?;
    assert!(!hosts.hosts().is_empty(), "need at least one host");

    let host_id = hosts.hosts()[0].id();

    // Test lock command (this will fail if MDM is not enrolled or not supported)
    let lock_result = client.hosts().lock(host_id).await;
    match lock_result {
        Ok(response) => {
            assert_eq!(response.host_id(), host_id);
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // It's ok if lock command not available or host not MDM enrolled
        }
        Err(error) => return Err(error),
    }

    // Test unlock command (requires a PIN)
    let unlock_result = client
        .hosts()
        .unlock(
            host_id,
            fleetdm_api_client::models::host::UnlockHostRequest {
                pin: "000000".to_string(),
            },
        )
        .await;
    match unlock_result {
        Ok(response) => {
            assert_eq!(response.host_id(), host_id);
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // It's ok if unlock command not available or host not MDM enrolled
        }
        Err(error) => return Err(error),
    }

    // We DO NOT test wipe as it would erase the device

    Ok(())
}

#[tokio::test]
async fn host_refetch() -> Result<()> {
    let client: FleetClient = live_client!();

    let hosts = client.hosts().list().per_page(1).send().await?;
    assert!(!hosts.hosts().is_empty(), "need at least one host");

    let host_id = hosts.hosts()[0].id();

    // Refetch should succeed (just queues a refetch, doesn't wait)
    client.hosts().refetch(host_id).await?;

    Ok(())
}
