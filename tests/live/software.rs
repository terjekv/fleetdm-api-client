//! Software endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::error::FleetError;

#[tokio::test]
async fn list_software() -> Result<()> {
    let client: FleetClient = live_client!();

    let software = client.software().list().per_page(50).send().await?;

    let sw_list = software.software();
    assert!(sw_list.len() <= 50, "should respect per_page limit");
    for sw in sw_list {
        assert!(sw.id() > 0, "each software item should have an id");
        assert!(
            !sw.name().is_empty(),
            "each software item should have a name"
        );
    }

    Ok(())
}

#[tokio::test]
async fn software_titles_and_detail() -> Result<()> {
    let client: FleetClient = live_client!();

    let titles = client
        .software()
        .list_titles(Some(&[("page", "0"), ("per_page", "10")]))
        .await?;
    let software_titles = titles.software_titles();
    for title in software_titles {
        assert!(title.id() > 0, "software title should have a valid id");
        assert!(
            !title.name().is_empty(),
            "software title should have a name"
        );
    }

    let Some(first_title) = software_titles.first() else {
        return Ok(());
    };
    let title_id = first_title.id();
    let title = client.software().get_title(title_id, None).await?;
    let detail = title.software_title();
    assert_eq!(
        detail.id(),
        title_id,
        "detail response should match requested title"
    );
    assert!(
        !detail.name().is_empty(),
        "software title detail should have a name"
    );

    Ok(())
}

#[tokio::test]
async fn software_versions_and_detail() -> Result<()> {
    let client: FleetClient = live_client!();

    let versions = client
        .software()
        .list_versions(Some(&[("page", "0"), ("per_page", "10")]))
        .await?;
    let software = versions.software();
    for version in software {
        assert!(version.id() > 0, "software version should have a valid id");
        assert!(
            !version.name().is_empty(),
            "software version should have a name"
        );
        assert!(
            !version.version().is_empty(),
            "software version should have a version string"
        );
    }

    let Some(first_version) = software.first() else {
        return Ok(());
    };
    let version_id = first_version.id();
    let version = client.software().get_version(version_id, None).await?;
    let detail = version.software();
    assert_eq!(
        detail.id(),
        version_id,
        "detail response should match requested software version"
    );

    Ok(())
}

#[tokio::test]
async fn os_versions_and_detail() -> Result<()> {
    let client: FleetClient = live_client!();

    let versions = client
        .software()
        .list_os_versions(Some(&[("page", "0"), ("per_page", "10")]))
        .await?;
    let os_versions = versions.os_versions();
    for version in os_versions {
        assert!(
            version.os_version_id() > 0,
            "OS version should have a valid id"
        );
        assert!(!version.name().is_empty(), "OS version should have a name");
    }

    let Some(first_version) = os_versions.first() else {
        return Ok(());
    };
    let os_version_id = first_version.os_version_id();
    let version = client
        .software()
        .get_os_version(os_version_id, None)
        .await?;
    let detail = version.os_version();
    assert_eq!(
        detail.os_version_id(),
        os_version_id,
        "detail response should match requested OS version"
    );

    Ok(())
}

#[tokio::test]
async fn fleet_maintained_apps() -> Result<()> {
    let client: FleetClient = live_client!();

    let response = match client
        .software()
        .list_fleet_maintained_apps(Some(&[("page", "0"), ("per_page", "10")]))
        .await
    {
        Ok(response) => response,
        Err(FleetError::PremiumRequired { .. }) => return Ok(()),
        Err(error) => return Err(error),
    };
    let apps = response.fleet_maintained_apps();
    assert!(
        !apps.is_empty(),
        "Fleet should expose at least one maintained app"
    );

    for app in apps {
        assert!(app.id() > 0, "fleet-maintained app should have a valid id");
        assert!(
            !app.slug().is_empty(),
            "fleet-maintained app should have a slug"
        );
    }

    let app_id = apps[0].id();
    let detail = client
        .software()
        .get_fleet_maintained_app(app_id, None)
        .await?;
    let app = detail.fleet_maintained_app();
    assert_eq!(
        app.id(),
        app_id,
        "detail response should match requested app"
    );

    Ok(())
}

#[tokio::test]
async fn software_vulnerable_filter() -> Result<()> {
    let client: FleetClient = live_client!();

    let all_software = client.software().list().per_page(1000).send().await?;
    let all_count = all_software.software().len();

    let vulnerable = client
        .software()
        .list()
        .vulnerable(true)
        .per_page(1000)
        .send()
        .await?;
    let vuln_count = vulnerable.software().len();

    assert!(
        vuln_count <= all_count,
        "vulnerable software count should be <= total software count"
    );

    let vuln_software = vulnerable.software();
    if !vuln_software.is_empty() {
        for item in vuln_software {
            assert!(
                !item.vulnerabilities().is_empty(),
                "vulnerable software should have vulnerabilities listed"
            );
        }
    }

    Ok(())
}
