#![allow(deprecated)]
//! Integration tests for FleetDM API Client
//!
//! These tests use wiremock to mock the Fleet API and test real-world scenarios
//! based on the official FleetDM API documentation at https://fleetdm.com/docs/rest-api/rest-api

use fleetdm_api_client::{FleetClient, FleetError, models::*};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;
use wiremock::matchers::{
    body_json, body_string_contains, header, header_regex, method, path, query_param,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ============================================================================
// AUTHENTICATION TESTS
// ============================================================================

#[tokio::test]
async fn test_login_success() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/login"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "user": {
                "id": 1,
                "name": "Jane Doe",
                "email": "janedoe@example.com",
                "enabled": true,
                "force_password_reset": false,
                "gravatar_url": "",
                "sso_enabled": false,
                "mfa_enabled": false,
                "global_role": "admin",
                "teams": []
            },
            "token": "test-token-12345"
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .login("janedoe@example.com", "password")
        .await
        .expect("should login successfully")
        .build();

    // Verify client was built successfully (authenticated state enforced by typestate)
    let _response = client.hosts().list().send().await;
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/login"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "message": "Authentication failed",
            "errors": [{"name": "base", "reason": "Authentication failed"}],
            "uuid": "error-uuid"
        })))
        .mount(&mock_server)
        .await;

    let result = FleetClient::builder(&server_url)
        .expect("should build client")
        .login("wrong@example.com", "wrong-password")
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_login_reports_mfa_as_a_typed_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/login"))
        .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
            "message": "Complete sign-in using the emailed magic link."
        })))
        .mount(&mock_server)
        .await;

    let result = FleetClient::builder(mock_server.uri())
        .unwrap()
        .login("mfa@example.com", "password")
        .await;
    assert!(matches!(result, Err(FleetError::MfaRequired(_))));
}

#[tokio::test]
async fn test_token_authentication() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    let _client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("my-api-token")
        .expect("valid token")
        .build();

    // Typestate pattern ensures this client is authenticated
}

// ============================================================================
// VERSION TESTS
// ============================================================================

#[tokio::test]
async fn test_version() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "version": "4.52.0",
            "branch": "main",
            "revision": "abc123",
            "go_version": "go1.21.0",
            "build_date": "2024-01-01T00:00:00Z",
            "build_user": "builder"
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client.version().get().await.expect("should get version");

    assert_eq!(response.version(), "4.52.0");
    assert!(!response.branch().is_empty());
}

// ============================================================================
// HOSTS TESTS
// ============================================================================

#[tokio::test]
async fn test_list_hosts() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/hosts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hosts": [
                {
                    "id": 1,
                    "uuid": "392547dc-0000-0000-a87a-d701ff75bc65",
                    "hostname": "macbook-pro.local",
                    "display_name": "macbook-pro.local",
                    "computer_name": "macbook-pro",
                    "platform": "darwin",
                    "osquery_version": "5.5.1",
                    "status": "online",
                    "uptime": 7200000,
                    "memory": 8589934592u64,
                    "primary_ip": "192.168.1.100",
                    "seen_time": "2020-11-05T06:03:39Z",
                    "created_at": "2020-11-05T05:09:44Z",
                    "updated_at": "2020-11-05T06:03:39Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .hosts()
        .list()
        .send()
        .await
        .expect("should list hosts");

    assert_eq!(response.hosts().len(), 1);
    assert_eq!(response.hosts()[0].hostname(), "macbook-pro.local");
    assert_eq!(response.hosts()[0].status(), HostStatus::Online);
}

#[tokio::test]
async fn test_list_hosts_with_status_filter() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/hosts"))
        .and(query_param("status", "online"))
        .and(query_param("fleet_id", "4"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hosts": [
                {
                    "id": 1,
                    "uuid": "online-host-uuid",
                    "hostname": "online-host.local",
                    "display_name": "online-host.local",
                    "computer_name": "online-host",
                    "platform": "darwin",
                    "osquery_version": "5.5.1",
                    "status": "online",
                    "uptime": 7200000,
                    "memory": 8589934592u64,
                    "primary_ip": "192.168.1.101",
                    "seen_time": "2020-11-05T06:03:39Z",
                    "created_at": "2020-11-05T05:09:44Z",
                    "updated_at": "2020-11-05T06:03:39Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .hosts()
        .list()
        .status(HostStatus::Online)
        .fleet_id(4)
        .send()
        .await
        .expect("should list online hosts");

    assert_eq!(response.hosts().len(), 1);
    assert_eq!(response.hosts()[0].status(), HostStatus::Online);
}

#[tokio::test]
async fn test_list_hosts_with_pagination() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/hosts"))
        .and(query_param("page", "0"))
        .and(query_param("per_page", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "hosts": (0..50).map(|i| serde_json::json!({
                "id": i + 1,
                "uuid": format!("host-uuid-{}", i),
                "hostname": format!("host-{}", i),
                "display_name": format!("host-{}", i),
                "computer_name": format!("host-{}", i),
                "platform": "darwin",
                "osquery_version": "5.5.1",
                "status": "online",
                "uptime": 7200000,
                "memory": 8589934592u64,
                "primary_ip": "192.168.1.101",
                "seen_time": "2020-11-05T06:03:39Z",
                "created_at": "2020-11-05T05:09:44Z",
                "updated_at": "2020-11-05T06:03:39Z"
            })).collect::<Vec<_>>()
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .hosts()
        .list()
        .page(0)
        .per_page(50)
        .send()
        .await
        .expect("should list hosts with pagination");

    assert_eq!(response.hosts().len(), 50);
}

#[tokio::test]
async fn test_get_host() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/hosts/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "host": {
                "id": 123,
                "uuid": "392547dc-0000-0000-a87a-d701ff75bc65",
                "hostname": "my-macbook.local",
                "display_name": "my-macbook.local",
                "computer_name": "my-macbook",
                "platform": "darwin",
                "osquery_version": "5.5.1",
                "status": "online",
                "uptime": 7200000,
                "memory": 8589934592u64,
                "primary_ip": "192.168.1.100",
                "seen_time": "2021-08-19T21:14:58Z",
                "created_at": "2021-08-19T02:02:22Z",
                "updated_at": "2021-08-19T21:14:58Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client.hosts().get(123).await.expect("should get host");

    assert_eq!(response.host().id(), 123);
    assert_eq!(response.host().hostname(), "my-macbook.local");
    assert_eq!(response.host().status(), HostStatus::Online);
}

// ============================================================================
// QUERIES TESTS
// ============================================================================

// ============================================================================
// POLICIES TESTS
// ============================================================================

#[tokio::test]
async fn test_list_policies() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/global/policies"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "policies": [
                {
                    "id": 1,
                    "name": "Gatekeeper enabled",
                    "query": "SELECT 1 FROM gatekeeper WHERE assessments_enabled = 1;",
                    "description": "Checks if gatekeeper is enabled",
                    "resolution": "Enable gatekeeper",
                    "critical": false,
                    "platform": "darwin",
                    "created_at": "2021-12-15T15:23:57Z",
                    "updated_at": "2021-12-15T15:23:57Z",
                    "author_id": null,
                    "author_name": "",
                    "author_email": "",
                    "team_id": null,
                    "passing_host_count": 0,
                    "failing_host_count": 0
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .policies()
        .list()
        .send()
        .await
        .expect("should list policies");

    assert_eq!(response.policies().len(), 1);
    assert_eq!(response.policies()[0].name(), "Gatekeeper enabled");
    assert!(!response.policies()[0].critical());
}

// ============================================================================
// TEAMS TESTS
// ============================================================================

// ============================================================================
// USERS TESTS
// ============================================================================

#[tokio::test]
async fn test_list_users() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "users": [
                {
                    "id": 1,
                    "name": "Jane Doe",
                    "email": "jane@example.com",
                    "global_role": "admin",
                    "enabled": true,
                    "force_password_reset": false,
                    "gravatar_url": "",
                    "sso_enabled": false,
                    "created_at": "2020-12-10T03:52:53Z",
                    "updated_at": "2020-12-10T03:52:53Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .users()
        .list()
        .send()
        .await
        .expect("should list users");

    assert_eq!(response.users().len(), 1);
    assert_eq!(response.users()[0].name(), "Jane Doe");
    assert_eq!(response.users()[0].email(), "jane@example.com");
}

// ============================================================================
// LABELS TESTS
// ============================================================================

#[tokio::test]
async fn test_list_labels() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/labels"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "labels": [
                {
                    "id": 6,
                    "name": "All Hosts",
                    "description": "All hosts which have enrolled in Fleet",
                    "query": "SELECT 1;",
                    "label_type": "builtin",
                    "host_count": 100,
                    "created_at": "2021-02-02T23:55:25Z",
                    "updated_at": "2021-02-02T23:55:25Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .labels()
        .list()
        .send()
        .await
        .expect("should list labels");

    assert_eq!(response.labels().len(), 1);
    assert_eq!(response.labels()[0].name(), "All Hosts");
    assert_eq!(response.labels()[0].label_type(), LabelType::Builtin);
    assert_eq!(response.labels()[0].host_count(), Some(100));
}

// ============================================================================
// ACTIVITIES TESTS
// ============================================================================

#[tokio::test]
async fn test_list_activities() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/activities"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "activities": [
                {
                    "id": 21,
                    "created_at": "2021-07-29T14:40:27Z",
                    "actor_full_name": "Jane Doe",
                    "actor_id": 1,
                    "actor_email": "jane@example.com",
                    "type": "created_team",
                    "details": {
                        "team_id": 2,
                        "team_name": "Apples"
                    }
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .activities()
        .list()
        .send()
        .await
        .expect("should list activities");

    assert_eq!(response.activities().len(), 1);
    assert_eq!(response.activities()[0].actor_full_name(), Some("Jane Doe"));
    assert_eq!(response.activities()[0].r#type(), "created_team");
}

// ============================================================================
// SOFTWARE TESTS
// ============================================================================

#[tokio::test]
async fn test_list_software() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/software/versions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "software": [
                {
                    "id": 1,
                    "name": "Firefox.app",
                    "version": "117.0",
                    "source": "apps",
                    "generated_cpe": "cpe:2.3:a:mozilla:firefox:117.0:*:*:*:*:macos:*:*",
                    "vulnerabilities": [],
                    "hosts_count": 5
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .software()
        .list()
        .send()
        .await
        .expect("should list software");

    let items = response.software();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name(), "Firefox.app");
}

#[tokio::test]
async fn test_list_fleets() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/fleets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "fleets": [
                {
                    "id": 1,
                    "name": "workstations",
                    "description": "Employee workstations",
                    "created_at": "2024-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .fleets()
        .list()
        .send()
        .await
        .expect("should list fleets");
    assert_eq!(response.fleets().len(), 1);
    assert_eq!(response.fleets()[0].name(), "workstations");
}

#[tokio::test]
async fn test_create_fleet() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/fleets"))
        .and(header("authorization", "Bearer test-token"))
        .and(body_json(serde_json::json!({"name": "new_fleet"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "team": {
                "id": 5,
                "name": "new_fleet",
                "description": "A new fleet",
                "created_at": "2024-01-01T00:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let request = CreateFleetRequest {
        name: "new_fleet".to_string(),
    };

    let response = client
        .fleets()
        .create(request)
        .await
        .expect("should create fleet");
    assert_eq!(response.fleet().id(), 5);
    assert_eq!(response.fleet().name(), "new_fleet");
}

#[tokio::test]
async fn test_list_reports() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/reports"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "reports": [
                {
                    "id": 42,
                    "name": "report1",
                    "query": "SELECT * FROM osquery_info",
                    "description": "report",
                    "fleet_id": null,
                    "created_at": "2024-01-01T00:00:00Z",
                    "updated_at": "2024-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let response = client
        .reports()
        .list()
        .send()
        .await
        .expect("should list reports");
    assert_eq!(response.reports().len(), 1);
    assert_eq!(response.reports()[0].name(), "report1");
}

#[test]
fn test_report_accepts_current_and_legacy_fleet_ids_together() {
    let response: ListReportsResponse = serde_json::from_value(serde_json::json!({
        "reports": [{
            "id": 42,
            "name": "compatibility",
            "query": "SELECT 1",
            "description": "",
            "fleet_id": 7,
            "team_id": 7
        }]
    }))
    .expect("reports may contain both fleet_id and team_id");

    assert_eq!(response.reports()[0].fleet_id(), Some(7));
}

#[test]
fn test_user_accepts_current_and_legacy_fleets_together() {
    let response: GetUserResponse = serde_json::from_value(serde_json::json!({
        "user": {
            "id": 9,
            "name": "Compatibility User",
            "email": "compat@example.test",
            "enabled": true,
            "force_password_reset": false,
            "gravatar_url": "",
            "sso_enabled": false,
            "global_role": null,
            "fleets": [{"id": 7, "name": "Current", "role": "observer"}],
            "teams": [{"id": 7, "name": "Legacy", "role": "observer"}],
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    }))
    .expect("users may contain both fleets and teams");

    assert_eq!(response.user().fleets().unwrap()[0].id(), 7);
    assert_eq!(response.user().teams().unwrap()[0].name(), "Current");
}

#[tokio::test]
async fn test_create_report() {
    let mock_server = MockServer::start().await;
    let server_url = mock_server.uri();

    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/reports"))
        .and(body_json(serde_json::json!({
            "name": "new_report",
            "description": "This is a new report.",
            "query": "SELECT * FROM osquery_info",
            "interval": 3600,
            "discard_data": false
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "report": {
                "id": 42,
                "name": "new_report",
                "query": "SELECT * FROM osquery_info",
                "description": "This is a new report.",
                "fleet_id": null,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-01T00:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(&server_url)
        .expect("should build client")
        .with_token("test-token")
        .expect("valid token")
        .build();

    let request = CreateReportRequest {
        name: "new_report".to_string(),
        description: Some("This is a new report.".to_string()),
        query: "SELECT * FROM osquery_info".to_string(),
        fleet_id: None,
        interval: Some(3600),
        platform: None,
        discard_data: Some(false),
        ..Default::default()
    };

    let response = client
        .reports()
        .create(request)
        .await
        .expect("should create report");
    assert_eq!(response.report().id(), 42);
    assert_eq!(response.report().name(), "new_report");
}

#[tokio::test]
async fn test_run_live_report_contract() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/reports/42/run"))
        .and(body_json(serde_json::json!({"host_ids": [1, 4]})))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "query_id": 42,
            "report_id": 42,
            "targeted_host_count": 2,
            "responded_host_count": 1,
            "results": [{"host_id": 1, "rows": [{"name": "macOS"}], "error": null}]
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("test-token")
        .expect("valid token")
        .build();
    let result = client
        .reports()
        .run_live(
            42,
            RunLiveReportRequest {
                host_ids: vec![1, 4],
            },
        )
        .await
        .unwrap();

    assert_eq!(result.report_id(), 42);
    assert_eq!(result.targeted_host_count(), 2);
    assert_eq!(result.results()[0].host_id(), 1);
}

#[tokio::test]
async fn test_untyped_success_response_bodies_are_ignored() {
    let mock_server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/api/v1/fleet/reports/id/42"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/api/v1/fleet/fleets/7"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"message": "fleet deleted"})),
        )
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("test-token")
        .expect("valid token")
        .build();
    client.reports().delete(42).await.unwrap();
    client.fleets().delete(7).await.unwrap();
}

#[tokio::test]
async fn test_targets_use_fleet_contract() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/targets"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "targets": {
                "hosts": [],
                "labels": [],
                "fleets": [{"id": 3, "name": "Workstations", "count": 4}],
                "teams": [{"id": 3, "name": "Workstations", "count": 4}]
            },
            "targets_count": 1,
            "targets_online": 0,
            "targets_offline": 0,
            "targets_missing_in_action": 0
        })))
        .mount(&mock_server)
        .await;
    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("test-token")
        .expect("valid token")
        .build();
    let response = client
        .targets()
        .search(&SearchTargetsRequest {
            query: String::new(),
            query_id: None,
            selected: None,
            include_observer: Some(true),
        })
        .await
        .unwrap();
    assert_eq!(response.targets().fleets()[0].id(), 3);
}

#[tokio::test]
async fn test_raw_api_preserves_binary_and_supports_delete_bodies() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/configuration_profiles/profile-1"))
        .and(query_param("alt", "media"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/octet-stream")
                .set_body_bytes(vec![0, 159, 146, 150]),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/api/v1/fleet/reports/My%20Report"))
        .and(body_json(serde_json::json!({"fleet_id": 9})))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("test-token")
        .expect("valid token")
        .build();
    let response = client
        .raw_api()
        .ep_get_api_v1_fleet_configuration_profiles_by_profile_uuid(
            "profile-1",
            Some(&[("alt", "media")]),
        )
        .await
        .unwrap();
    assert_eq!(response.bytes(), &[0, 159, 146, 150]);
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let body = ApiRequestBody::json(serde_json::json!({"fleet_id": 9}));
    client
        .raw_api()
        .ep_delete_api_v1_fleet_reports_by_name("My Report", None, Some(&body))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_raw_api_supports_multipart() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/scripts"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .and(body_string_contains("fleet_id"))
        .and(body_string_contains("test.sh"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "script_id": 10
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("test-token")
        .expect("valid token")
        .build();
    let body = ApiRequestBody::multipart(ApiMultipartBody::new().text("fleet_id", "1").file(
        "script",
        "test.sh",
        b"#!/bin/sh\n".to_vec(),
    ));
    let response = client
        .raw_api()
        .ep_post_api_v1_fleet_scripts(None, Some(&body))
        .await
        .unwrap();
    assert_eq!(
        response.json::<serde_json::Value>().unwrap()["script_id"],
        10
    );
}

#[tokio::test]
async fn test_retry_policy_retries_real_429_responses() {
    let mock_server = MockServer::start().await;
    let attempts = Arc::new(AtomicUsize::new(0));
    let responder_attempts = attempts.clone();
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/version"))
        .respond_with(move |_: &wiremock::Request| {
            if responder_attempts.fetch_add(1, Ordering::SeqCst) < 2 {
                ResponseTemplate::new(429).insert_header("retry-after", "0")
            } else {
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "version": "4.82.0",
                    "branch": "main",
                    "revision": "abc",
                    "go_version": "go1.24",
                    "build_date": "2026-01-01",
                    "build_user": "builder"
                }))
            }
        })
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_retry_policy(fleetdm_api_client::RetryPolicy {
            max_retries: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
            respect_retry_after: true,
            use_jitter: false,
        })
        .with_token("test-token")
        .expect("valid token")
        .build();

    assert_eq!(client.version().get().await.unwrap().version(), "4.82.0");
    assert_eq!(attempts.load(Ordering::SeqCst), 3);
}

#[tokio::test]
async fn public_auth_uses_no_bearer_token_and_preserves_the_sso_cookie() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/reset_password"))
        .and(body_json(serde_json::json!({
            "new_password": "new-password",
            "new_password_confirmation": "new-password",
            "password_reset_token": "reset-token"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/sso"))
        .and(body_json(serde_json::json!({"relay_url": "/dashboard"})))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header(
                    "set-cookie",
                    "__Host-FLEETSSOSESSIONID=session-value; Path=/; HttpOnly; Secure",
                )
                .set_body_json(serde_json::json!({"url": "https://idp.example.com/login"})),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/sso/callback"))
        .and(header("cookie", "__Host-FLEETSSOSESSIONID=session-value"))
        .and(body_json(
            serde_json::json!({"SAMLResponse": "encoded-saml"}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"token": "token"})),
        )
        .mount(&mock_server)
        .await;

    let builder = FleetClient::builder(mock_server.uri()).unwrap();
    builder
        .auth()
        .reset_password("new-password", "new-password", "reset-token")
        .await
        .unwrap();
    let initiation = builder.auth().initiate_sso("/dashboard").await.unwrap();
    assert_eq!(
        initiation.session_cookie(),
        Some("__Host-FLEETSSOSESSIONID=session-value")
    );
    builder
        .auth()
        .sso_callback("encoded-saml", initiation.session_cookie())
        .await
        .unwrap();

    for request in mock_server.received_requests().await.unwrap() {
        assert!(
            request.headers.get("authorization").is_none(),
            "public authentication must not send an Authorization header"
        );
    }
}

#[tokio::test]
async fn public_auth_validation_fails_before_network_io() {
    let mock_server = MockServer::start().await;
    let builder = FleetClient::builder(mock_server.uri()).unwrap();

    assert!(matches!(
        builder.auth().reset_password("one", "two", "token").await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        builder
            .auth()
            .initiate_sso("https://attacker.example")
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(mock_server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn os_settings_use_documented_multipart_json_and_binary_contracts() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/configuration_profiles"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .and(body_string_contains("profile.mobileconfig"))
        .and(body_string_contains("fleet_id"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"profile_uuid": "profile-uuid"})),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path(
            "/api/v1/fleet/configuration_profiles/profile-uuid/status",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "verified": 3,
            "verifying": 2,
            "failed": 1,
            "pending": 4
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/configuration_profiles/profile-uuid"))
        .and(query_param("alt", "media"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/x-apple-aspen-config")
                .insert_header(
                    "content-disposition",
                    "attachment; filename=profile.mobileconfig",
                )
                .set_body_bytes(b"profile-bytes"),
        )
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/configuration_profiles/batch"))
        .and(query_param("fleet_id", "9"))
        .and(query_param("dry_run", "true"))
        .and(body_json(serde_json::json!({
            "configuration_profiles": [{"profile": "e30="}]
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/disk_encryption"))
        .and(body_json(serde_json::json!({
            "fleet_id": 9,
            "enable_disk_encryption": true,
            "windows_require_bitlocker_pin": true
        })))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("token")
        .unwrap()
        .build();
    let mut create = CreateConfigurationProfileRequest::new(
        FileUpload::new("profile.mobileconfig", b"profile-data").unwrap(),
    );
    create.fleet_id = Some(9);
    assert_eq!(
        client
            .os_settings()
            .create_profile(create)
            .await
            .unwrap()
            .profile_uuid(),
        "profile-uuid"
    );
    let status = client
        .os_settings()
        .get_profile_status("profile-uuid")
        .await
        .unwrap();
    assert_eq!(status.verified(), 3);
    assert_eq!(status.pending(), 4);
    let download = client
        .os_settings()
        .download_profile("profile-uuid")
        .await
        .unwrap();
    assert_eq!(download.bytes(), b"profile-bytes");
    assert_eq!(
        download.content_type(),
        Some("application/x-apple-aspen-config")
    );

    let mut batch = BatchConfigurationProfilesRequest::new(vec![BatchConfigurationProfile::new(
        FileUpload::new("profile.json", b"{}").unwrap(),
    )]);
    batch.fleet_id = Some(9);
    batch.dry_run = true;
    client
        .os_settings()
        .create_profiles_batch(&batch)
        .await
        .unwrap();
    client
        .os_settings()
        .update_disk_encryption(UpdateDiskEncryptionRequest {
            fleet_id: Some(9),
            enable_disk_encryption: true,
            windows_require_bitlocker_pin: Some(true),
        })
        .await
        .unwrap();
}

#[tokio::test]
async fn setup_and_software_uploads_are_multipart_and_downloads_preserve_bytes() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/setup_experience/eula"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .and(body_string_contains("terms.pdf"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/v1/fleet/software/package"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .and(body_string_contains("installer.pkg"))
        .and(body_string_contains("self_service"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "software_package": {
                "fleet_id": 7,
                "title_id": 42,
                "name": "installer.pkg",
                "platform": "darwin",
                "version": "1.0",
                "hash_sha256": "abc"
            }
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/software/titles/42/package"))
        .and(query_param("fleet_id", "7"))
        .and(query_param("alt", "media"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/octet-stream")
                .set_body_bytes(vec![0, 159, 146, 150]),
        )
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_retry_policy(fleetdm_api_client::RetryPolicy::aggressive())
        .with_token("token")
        .unwrap()
        .build();
    client
        .setup_experience()
        .create_eula(FileUpload::new("terms.pdf", b"pdf-data").unwrap())
        .await
        .unwrap();
    let package = client
        .software()
        .upload_package(UploadSoftwarePackageRequest {
            software: FileUpload::new("installer.pkg", b"package-data").unwrap(),
            options: SoftwarePackageOptions {
                fleet_id: Some(7),
                self_service: Some(true),
                ..Default::default()
            },
        })
        .await
        .unwrap();
    assert_eq!(package.software_package().title_id(), 42);
    let mut download = client.software().download_package(42, 7).await.unwrap();
    assert_eq!(download.content_type(), Some("application/octet-stream"));
    let mut bytes = Vec::new();
    while let Some(chunk) = download.next_chunk().await.unwrap() {
        bytes.extend_from_slice(&chunk);
    }
    assert_eq!(bytes, [0, 159, 146, 150]);
}

#[tokio::test]
async fn vulnerability_contract_handles_filters_wrappers_and_no_content() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/vulnerabilities"))
        .and(query_param("order_key", "host_count"))
        .and(query_param("order_direction", "desc"))
        .and(query_param("query", "CVE-2026"))
        .and(query_param("after", "4"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "vulnerabilities": [],
            "count": 0
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/vulnerabilities/CVE-2026-1234"))
        .and(query_param("fleet_id", "5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "vulnerability": {
                "cve": "CVE-2026-1234",
                "created_at": "2026-01-02T03:04:05Z",
                "hosts_count": 1,
                "os_versions": [],
                "software": [{
                    "id": 9,
                    "software_title_id": 8,
                    "name": "Example",
                    "version": "1.0",
                    "source": "apps",
                    "hosts_count": 1
                }]
            }
        })))
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/vulnerabilities/CVE-2026-9999"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("token")
        .unwrap()
        .build();
    client
        .vulnerabilities()
        .list()
        .order_key(VulnerabilityOrderKey::HostCount)
        .order_direction(OrderDirection::Desc)
        .query("CVE-2026")
        .after("4")
        .send()
        .await
        .unwrap();
    let details = client
        .vulnerabilities()
        .get("CVE-2026-1234", Some(5))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(details.software()[0].software_id(), 9);
    assert!(
        client
            .vulnerabilities()
            .get("CVE-2026-9999", None)
            .await
            .unwrap()
            .is_none()
    );
    assert!(matches!(
        client.vulnerabilities().get("not-a-cve", None).await,
        Err(FleetError::Validation(_))
    ));
}

#[tokio::test]
async fn errors_retain_fleet_request_context_and_rate_limit_body() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/version"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "message": "invalid input",
            "uuid": "request-uuid",
            "errors": [{"name": "field", "reason": "must be valid"}]
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/version"))
        .respond_with(ResponseTemplate::new(429).set_body_json(serde_json::json!({
            "message": "slow down",
            "uuid": "rate-uuid",
            "errors": []
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("token")
        .unwrap()
        .build();
    let first = client.version().get().await.unwrap_err();
    match first {
        FleetError::BadRequest(message) => {
            assert!(message.contains("request-uuid"));
            assert!(message.contains("field: must be valid"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
    match client.version().get().await.unwrap_err() {
        FleetError::RateLimit {
            response: Some(response),
            ..
        } => {
            assert_eq!(response.message, "slow down");
            assert_eq!(response.uuid.as_deref(), Some("rate-uuid"));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn invalid_mutations_fail_before_network_io() {
    let mock_server = MockServer::start().await;
    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("token")
        .unwrap()
        .build();

    assert!(matches!(
        client
            .fleets()
            .update(1, UpdateFleetRequest::default())
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .teams()
            .update(
                1,
                UpdateTeamRequest {
                    name: None,
                    description: None,
                    agent_options: None,
                },
            )
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .labels()
            .update(
                1,
                UpdateLabelRequest {
                    name: None,
                    description: None,
                },
            )
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .users()
            .update(
                1,
                UpdateUserRequest {
                    name: None,
                    email: None,
                    enabled: None,
                    global_role: None,
                    teams: None,
                },
            )
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .policies()
            .update(
                1,
                UpdatePolicyRequest {
                    name: None,
                    query: None,
                    description: None,
                    resolution: None,
                    team_id: None,
                    platform: None,
                    critical: None,
                    conditional_access_enabled: None,
                },
            )
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .reports()
            .update(1, UpdateReportRequest::default())
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .queries()
            .create(CreateQueryRequest {
                name: " ".into(),
                query: String::new(),
                description: None,
                team_id: None,
                interval: None,
                platform: None,
                min_osquery_version: None,
                observer_can_run: None,
                discard_data: None,
            })
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .invitations()
            .update(
                1,
                UpdateInvitationRequest {
                    email: None,
                    name: None,
                    sso_enabled: None,
                    global_role: None,
                    teams: None,
                },
            )
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client.config().update(&serde_json::Value::Null).await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .config()
            .modify_enroll_secrets(&ModifyEnrollSecretsRequest { secrets: vec![] })
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .certificates()
            .create_ca(CreateCertificateAuthorityRequest {
                digicert: None,
                ndes_scep_proxy: None,
                custom_scep_proxy: None,
                custom_est_proxy: None,
                hydrant: None,
                smallstep: None,
            })
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .software()
            .patch_package(
                1,
                UpdateSoftwarePackageRequest {
                    fleet_id: 1,
                    software: None,
                    options: SoftwarePackageOptions::default(),
                },
            )
            .await,
        Err(FleetError::Validation(_))
    ));
    assert!(matches!(
        client
            .scripts()
            .create(CreateScriptRequest::new("../unsafe.sh", "echo hello"))
            .await,
        Err(FleetError::Validation(_))
    ));

    assert!(mock_server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn updating_a_script_uses_a_rebuildable_multipart_request() {
    let mock_server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/api/v1/fleet/scripts/7"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .and(body_string_contains("replacement.sh"))
        .and(body_string_contains("echo replacement"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 7,
            "name": "replacement.sh",
            "team_id": null,
            "created_at": "2025-01-01T00:00:00Z",
            "updated_at": "2025-01-02T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_retry_policy(fleetdm_api_client::RetryPolicy::aggressive())
        .with_token("token")
        .unwrap()
        .build();
    let response = client
        .scripts()
        .update(
            7,
            FileUpload::new("replacement.sh", b"echo replacement").unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.script().id(), 7);
}

#[tokio::test]
async fn default_client_does_not_follow_authenticated_redirects() {
    let destination = MockServer::start().await;
    let origin = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/version"))
        .respond_with(
            ResponseTemplate::new(307)
                .insert_header("location", format!("{}/captured", destination.uri())),
        )
        .mount(&origin)
        .await;
    Mock::given(method("GET"))
        .and(path("/captured"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&destination)
        .await;

    let client = FleetClient::builder(origin.uri())
        .unwrap()
        .with_token("token")
        .unwrap()
        .build();
    assert!(client.version().get().await.is_err());
    assert!(destination.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn oversized_json_responses_are_rejected_before_buffering() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/fleet/version"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![b'x'; 8 * 1024 * 1024 + 1]))
        .mount(&mock_server)
        .await;

    let client = FleetClient::builder(mock_server.uri())
        .unwrap()
        .with_token("token")
        .unwrap()
        .build();
    assert!(matches!(
        client.version().get().await,
        Err(FleetError::ResponseTooLarge { limit: 8_388_608 })
    ));
}
