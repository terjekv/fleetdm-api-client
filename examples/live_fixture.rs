use fleetdm_api_client::error::FleetError;
use fleetdm_api_client::models::fleet::CreateFleetRequest;
use fleetdm_api_client::models::label::CreateLabelRequest;
use fleetdm_api_client::models::policy::CreatePolicyRequest;
use fleetdm_api_client::models::report::CreateReportRequest;
use fleetdm_api_client::models::script::CreateScriptRequest;
use fleetdm_api_client::models::user::CreateUserRequest;
use fleetdm_api_client::{FleetClient, RetryPolicy};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Clone, Serialize, Deserialize)]
struct LiveFixture {
    token: String,
    run_id: String,
    host_id: u64,
    original_fleet_id: u64,
    fleet_id: Option<u64>,
    report_id: u64,
    label_id: u64,
    policy_id: u64,
    script_id: u64,
    user_id: u64,
}

struct PendingFixture {
    token: String,
    run_id: String,
    host_id: u64,
    original_fleet_id: u64,
    fleet_id: Option<u64>,
    report_id: Option<u64>,
    label_id: Option<u64>,
    policy_id: Option<u64>,
    script_id: Option<u64>,
    user_id: Option<u64>,
}

impl PendingFixture {
    fn complete(self) -> Result<LiveFixture, Box<dyn std::error::Error>> {
        Ok(LiveFixture {
            token: self.token,
            run_id: self.run_id,
            host_id: self.host_id,
            original_fleet_id: self.original_fleet_id,
            fleet_id: self.fleet_id,
            report_id: self.report_id.ok_or("report fixture was not created")?,
            label_id: self.label_id.ok_or("label fixture was not created")?,
            policy_id: self.policy_id.ok_or("policy fixture was not created")?,
            script_id: self.script_id.ok_or("script fixture was not created")?,
            user_id: self.user_id.ok_or("user fixture was not created")?,
        })
    }
}

async fn client() -> Result<FleetClient, Box<dyn std::error::Error>> {
    let url = env::var("FLEET_URL")?;
    let email = env::var("FLEET_EMAIL")?;
    let password = env::var("FLEET_PASSWORD")?;
    Ok(FleetClient::builder(url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .login(email, password)
        .await?
        .build())
}

async fn setup(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = client().await?;
    let hosts = client.hosts().list().per_page(1).send().await?;
    let host = hosts
        .hosts()
        .first()
        .ok_or("Fleet preview did not contain a host to use as a fixture")?;
    let run_id = format!(
        "api-client-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis()
    );
    let mut fixture = PendingFixture {
        token: client.token().to_string(),
        run_id: run_id.clone(),
        host_id: host.id(),
        original_fleet_id: host.team_id().unwrap_or(0),
        fleet_id: None,
        report_id: None,
        label_id: None,
        policy_id: None,
        script_id: None,
        user_id: None,
    };

    let result: Result<(), Box<dyn std::error::Error>> = async {
        fixture.report_id = Some(
            client
                .reports()
                .create(CreateReportRequest {
                    name: format!("{run_id}-report"),
                    query: "SELECT * FROM osquery_info".to_string(),
                    description: Some("Deterministic API client live fixture".to_string()),
                    interval: Some(0),
                    discard_data: Some(false),
                    ..Default::default()
                })
                .await?
                .report()
                .id(),
        );

        fixture.label_id = Some(
            client
                .labels()
                .create(CreateLabelRequest {
                    name: format!("{run_id}-label"),
                    description: "Deterministic API client live fixture".to_string(),
                    query: "SELECT 1".to_string(),
                    platform: None,
                })
                .await?
                .label()
                .id(),
        );

        fixture.policy_id = Some(
            client
                .policies()
                .create(CreatePolicyRequest {
                    name: format!("{run_id}-policy"),
                    query: "SELECT 1 FROM osquery_info WHERE version != ''".to_string(),
                    description: "Deterministic API client live fixture".to_string(),
                    resolution: Some("No action required; this is a test fixture".to_string()),
                    team_id: None,
                    platform: None,
                    critical: Some(false),
                    conditional_access_enabled: Some(false),
                })
                .await?
                .policy()
                .id(),
        );

        fixture.script_id = Some(
            client
                .scripts()
                .create(CreateScriptRequest::new(
                    format!("{run_id}.sh"),
                    "#!/bin/sh\necho fleet-api-client-fixture\n",
                ))
                .await?
                .script_id(),
        );

        fixture.user_id = Some(
            client
                .users()
                .create(CreateUserRequest {
                    name: format!("Live Fixture {run_id}"),
                    email: format!("{run_id}@example.test"),
                    password: Some(format!("Fleet-{run_id}-A1!")),
                    global_role: Some("observer".to_string()),
                    sso_enabled: Some(false),
                    api_only: Some(false),
                    teams: None,
                })
                .await?
                .user()
                .id(),
        );

        match client
            .fleets()
            .create(CreateFleetRequest {
                name: format!("{run_id}-fleet"),
            })
            .await
        {
            Ok(created) => {
                let fleet_id = created.fleet().id();
                fixture.fleet_id = Some(fleet_id);
                client
                    .hosts()
                    .transfer(fleet_id, vec![fixture.host_id])
                    .await?;
            }
            Err(FleetError::PremiumRequired { .. }) => {}
            Err(error) => return Err(error.into()),
        }

        Ok(())
    }
    .await;

    if let Err(error) = result {
        cleanup_pending(&client, &fixture).await;
        return Err(error);
    }

    let fixture = fixture.complete()?;
    fs::write(path, serde_json::to_vec_pretty(&fixture)?)?;
    println!(
        "seeded live fixture {} (fleet: {})",
        fixture.run_id,
        fixture.fleet_id.map_or_else(
            || "unavailable without Premium".to_string(),
            |id| id.to_string()
        )
    );
    Ok(())
}

async fn cleanup_pending(client: &FleetClient, fixture: &PendingFixture) {
    if fixture.fleet_id.is_some() {
        cleanup_result(
            "restore fixture host",
            client
                .hosts()
                .transfer(fixture.original_fleet_id, vec![fixture.host_id])
                .await,
        );
    }
    if let Some(id) = fixture.fleet_id {
        cleanup_result("delete fixture fleet", client.fleets().delete(id).await);
    }
    if let Some(id) = fixture.user_id {
        cleanup_result("delete fixture user", client.users().delete(id).await);
    }
    if let Some(id) = fixture.script_id {
        cleanup_result("delete fixture script", client.scripts().delete(id).await);
    }
    if let Some(id) = fixture.policy_id {
        cleanup_result("delete fixture policy", client.policies().delete(id).await);
    }
    if let Some(id) = fixture.report_id {
        cleanup_result("delete fixture report", client.reports().delete(id).await);
    }
    if let Some(id) = fixture.label_id {
        cleanup_result("delete fixture label", client.labels().delete(id).await);
    }
}

fn cleanup_result<T>(operation: &str, result: fleetdm_api_client::Result<T>) {
    if let Err(error) = result {
        eprintln!("warning: could not {operation}: {error}");
    }
}

async fn cleanup(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let fixture: LiveFixture = serde_json::from_slice(&fs::read(path)?)?;
    let url = env::var("FLEET_URL")?;
    let client = FleetClient::builder(url)?
        .with_retry_policy(RetryPolicy::aggressive())
        .with_token(fixture.token.clone())?
        .build();
    let pending = PendingFixture {
        token: fixture.token.clone(),
        run_id: fixture.run_id.clone(),
        host_id: fixture.host_id,
        original_fleet_id: fixture.original_fleet_id,
        fleet_id: fixture.fleet_id,
        report_id: Some(fixture.report_id),
        label_id: Some(fixture.label_id),
        policy_id: Some(fixture.policy_id),
        script_id: Some(fixture.script_id),
        user_id: Some(fixture.user_id),
    };
    cleanup_pending(&client, &pending).await;
    println!("cleaned live fixture {}", fixture.run_id);
    Ok(())
}

async fn authenticate(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = client().await?;
    fs::write(
        path,
        serde_json::to_vec(&serde_json::json!({ "token": client.token() }))?,
    )?;
    println!("wrote shared live-test token");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let command = args
        .next()
        .ok_or("expected setup, cleanup, or authenticate command")?;
    let path = args.next().ok_or("expected fixture manifest path")?;
    if args.next().is_some() {
        return Err("unexpected extra arguments".into());
    }
    match command.as_str() {
        "setup" => setup(Path::new(&path)).await,
        "cleanup" => cleanup(Path::new(&path)).await,
        "authenticate" => authenticate(Path::new(&path)).await,
        _ => Err(format!("unknown command: {command}").into()),
    }
}
