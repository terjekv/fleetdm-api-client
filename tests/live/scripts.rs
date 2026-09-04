//! Script endpoint tests

use crate::live_client;
use fleetdm_api_client::FleetClient;
use fleetdm_api_client::Result;
use fleetdm_api_client::models::script::CreateScriptRequest;

#[tokio::test]
async fn list_scripts() -> Result<()> {
    let client: FleetClient = live_client!();

    let scripts = client.scripts().list().per_page(100).send().await?;

    let script_list = scripts.scripts();
    assert!(script_list.len() <= 100, "should respect per_page limit");
    for script in script_list {
        assert!(script.id() > 0, "each script should have a valid id");
        assert!(!script.name().is_empty(), "each script should have a name");
    }

    Ok(())
}

#[tokio::test]
async fn create_script() -> Result<()> {
    let client: FleetClient = live_client!();

    let script_name = format!("test-script-{}.sh", chrono::Utc::now().timestamp());
    let create_req = CreateScriptRequest {
        name: script_name.clone(),
        script_contents: "#!/bin/bash\necho 'Hello from Fleet test script'\n".to_string(),
        team_id: None,
    };

    let created = client.scripts().create(create_req.clone()).await?;
    assert!(
        created.script_id() > 0,
        "created script should have a valid id"
    );

    // Clean up
    client.scripts().delete(created.script_id()).await?;

    Ok(())
}

#[tokio::test]
async fn read_script() -> Result<()> {
    let client: FleetClient = live_client!();

    let script_name = format!("read-script-{}.sh", chrono::Utc::now().timestamp());
    let create_req = CreateScriptRequest {
        name: script_name.clone(),
        script_contents: "#!/bin/bash\necho 'Hello from Fleet test script'\n".to_string(),
        team_id: None,
    };

    let created = client.scripts().create(create_req.clone()).await?;

    let fetched = client.scripts().get(created.script_id()).await?;
    assert_eq!(
        fetched.script().id(),
        created.script_id(),
        "fetched script id should match"
    );
    assert_eq!(
        fetched.script().name(),
        script_name,
        "fetched script name should match"
    );

    // Clean up
    client.scripts().delete(created.script_id()).await?;

    Ok(())
}

#[tokio::test]
async fn delete_script() -> Result<()> {
    let client: FleetClient = live_client!();

    let script_name = format!("delete-script-{}.sh", chrono::Utc::now().timestamp());
    let create_req = CreateScriptRequest {
        name: script_name.clone(),
        script_contents: "#!/bin/bash\necho 'Hello from Fleet test script'\n".to_string(),
        team_id: None,
    };

    let created = client.scripts().create(create_req.clone()).await?;

    client.scripts().delete(created.script_id()).await?;

    let get_result = client.scripts().get(created.script_id()).await;
    assert!(get_result.is_err(), "getting deleted script should fail");

    Ok(())
}
