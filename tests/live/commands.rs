//! MDM commands endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;

#[tokio::test]
async fn list_commands() -> Result<()> {
    let client: FleetClient = live_client!();

    let result = client.commands().list().per_page(10).send().await;
    match result {
        Ok(commands) => {
            assert!(
                commands.commands().len() <= 10,
                "should respect per_page limit"
            );
            for cmd in commands.commands() {
                assert!(!cmd.command_uuid().is_empty(), "command should have uuid");
                assert!(
                    !cmd.request_type().is_empty(),
                    "command should have request_type"
                );
                assert!(!cmd.status().is_empty(), "command should have status");
                assert!(cmd.host_id() > 0, "command should have host_id");
            }
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // MDM not configured — acceptable
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn list_commands_with_filters() -> Result<()> {
    let client: FleetClient = live_client!();

    // Filter by a nonexistent host — should return empty or error if MDM not configured
    let result = client
        .commands()
        .list()
        .host_identifier("nonexistent-host-identifier-xyz")
        .per_page(10)
        .send()
        .await;

    match result {
        Ok(filtered) => {
            assert!(
                filtered.commands().is_empty(),
                "filtering by bogus host_identifier should return no commands"
            );
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // MDM not configured — acceptable
        }
        Err(error) => return Err(error),
    }

    // Filter by request_type
    let result = client
        .commands()
        .list()
        .request_type("DeviceLock")
        .per_page(10)
        .send()
        .await;

    match result {
        Ok(by_type) => {
            for cmd in by_type.commands() {
                assert_eq!(
                    cmd.request_type(),
                    "DeviceLock",
                    "filtered commands should match request_type"
                );
            }
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // MDM not configured — acceptable
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

#[tokio::test]
async fn run_command_requires_mdm() -> Result<()> {
    let client: FleetClient = live_client!();

    // Get a host to run a command on
    let hosts = client.hosts().list().per_page(1).send().await?;
    if hosts.hosts().is_empty() {
        return Ok(());
    }

    let host_id = hosts.hosts()[0].id();

    // Running a command on a non-MDM-enrolled host should fail
    let result = client.commands().run("RestartDevice", vec![host_id]).await;

    match result {
        Ok(response) => {
            // If it somehow succeeded, verify the response shape
            assert!(!response.command_uuid().is_empty());
            assert!(!response.request_type().is_empty());

            // Check results
            let results = client.commands().results(response.command_uuid()).await?;
            let _ = results.results();
        }
        Err(error) if crate::live::common::is_optional_feature_error(&error) => {
            // Expected — hosts are not MDM-enrolled in preview
        }
        Err(error) => return Err(error),
    }

    Ok(())
}
