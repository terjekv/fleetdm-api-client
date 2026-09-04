//! Deterministic fixture contract shared by the live compatibility suite.

use crate::{live_client, live_fixture};
use fleetdm_api_client::Result;

#[tokio::test]
async fn seeded_fixture_contract() -> Result<()> {
    let client = live_client!();
    let fixture = live_fixture!();

    assert!(fixture.run_id.starts_with("api-client-"));

    let host = client.hosts().get(fixture.host_id).await?;
    assert_eq!(host.host().id(), fixture.host_id);

    let report = client.reports().get(fixture.report_id).await?;
    assert_eq!(report.report().id(), fixture.report_id);
    assert!(report.report().name().starts_with(&fixture.run_id));

    let label = client.labels().get(fixture.label_id).await?;
    assert_eq!(label.label().id(), fixture.label_id);
    assert!(label.label().name().starts_with(&fixture.run_id));

    let policy = client.policies().get(fixture.policy_id).await?;
    assert_eq!(policy.policy().id(), fixture.policy_id);
    assert!(policy.policy().name().starts_with(&fixture.run_id));

    let script = client.scripts().get(fixture.script_id).await?;
    assert_eq!(script.script().id(), fixture.script_id);
    assert!(script.script().name().starts_with(&fixture.run_id));

    let user = client.users().get(fixture.user_id).await?;
    assert_eq!(user.user().id(), fixture.user_id);
    assert!(user.user().name().contains(&fixture.run_id));

    if let Some(fleet_id) = fixture.fleet_id {
        let fleet = client.fleets().get(fleet_id).await?;
        assert_eq!(fleet.fleet().id(), fleet_id);
        assert!(fleet.fleet().name().starts_with(&fixture.run_id));

        let hosts = client
            .hosts()
            .list()
            .fleet_id(fleet_id)
            .per_page(100)
            .send()
            .await?;
        assert!(
            hosts
                .hosts()
                .iter()
                .any(|host| host.id() == fixture.host_id),
            "fixture host should be assigned to the fixture fleet"
        );
    }

    Ok(())
}
