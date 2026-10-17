use fleetdm_api_client::{FleetClient, models::host::Host};
use serde_json::{Value, json};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

fn host_json() -> Value {
    json!({
        "id": 42, "uuid": "00000000-0000-4000-8000-000000000042",
        "hostname": "host.example.test", "display_name": "Example host",
        "computer_name": "example", "platform": "darwin", "osquery_version": "5.0.0",
        "os_version": "macOS 15.2", "status": "offline", "uptime": 0,
        "memory": 17179869184_u64, "primary_ip": "192.0.2.42",
        "primary_mac": "02:00:00:00:00:42", "hardware_serial": "EXAMPLE-42",
        "created_at": "2025-01-01T00:00:00Z", "updated_at": "2025-02-01T00:00:00Z",
        "seen_time": "2025-02-02T00:00:00Z"
    })
}

fn inventory_fields() -> Value {
    json!({
        "build": "24C101", "platform_like": "darwin", "cpu_logical_cores": 8,
        "cpu_subtype": "ARM64E", "public_ip": "198.51.100.42",
        "hardware_vendor": "Example vendor", "hardware_version": "1.2",
        "hardware_marketing_name": "Example workstation", "gigs_total_disk_space": 246.5,
        "gigs_all_disk_space": 1024.25, "disk_encryption_enabled": false,
        "orbit_version": "1.0.0", "fleet_desktop_version": "1.0.0",
        "detail_updated_at": "2025-01-31T00:00:00Z"
    })
}

#[tokio::test]
async fn list_and_get_preserve_typed_inventory_fields() {
    let server = MockServer::start().await;
    let client = FleetClient::builder(server.uri())
        .unwrap()
        .with_token("test-token")
        .unwrap()
        .build();
    let mut payload = host_json();
    payload
        .as_object_mut()
        .unwrap()
        .extend(inventory_fields().as_object().unwrap().clone());
    for (route, body) in [
        ("/api/v1/fleet/hosts", json!({"hosts": [payload.clone()]})),
        ("/api/v1/fleet/hosts/42", json!({"host": payload})),
    ] {
        Mock::given(method("GET"))
            .and(path(route))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;
    }
    let list = client.hosts().list().send().await.unwrap();
    let detail = client.hosts().get(42).await.unwrap();
    for host in [&list.hosts()[0], detail.host()] {
        assert_eq!(host.build(), Some("24C101"));
        assert_eq!(host.platform_like(), Some("darwin"));
        assert_eq!(host.cpu_logical_cores(), Some(8));
        assert_eq!(host.cpu_subtype(), Some("ARM64E"));
        assert_eq!(host.public_ip(), Some("198.51.100.42"));
        assert_eq!(host.hardware_vendor(), Some("Example vendor"));
        assert_eq!(host.hardware_version(), Some("1.2"));
        assert_eq!(host.hardware_marketing_name(), Some("Example workstation"));
        assert_eq!(host.gigs_total_disk_space(), Some(246.5));
        assert_eq!(host.gigs_all_disk_space(), Some(1024.25));
        assert_eq!(host.disk_encryption_enabled(), Some(false));
        assert_eq!(host.orbit_version(), Some("1.0.0"));
        assert_eq!(host.fleet_desktop_version(), Some("1.0.0"));
        assert!(host.detail_updated_at().unwrap() < host.seen_time());
        let roundtrip = serde_json::to_value(host).unwrap();
        for (key, expected) in inventory_fields().as_object().unwrap() {
            assert_eq!(&roundtrip[key], expected, "field {key}");
        }
    }
}

#[test]
fn missing_and_null_inventory_remain_unknown() {
    for explicit_null in [false, true] {
        let mut payload = host_json();
        if explicit_null {
            for key in inventory_fields().as_object().unwrap().keys() {
                payload[key] = Value::Null;
            }
        }
        let host: Host = serde_json::from_value(payload).unwrap();
        assert_eq!(host.disk_encryption_enabled(), None);
        assert_eq!(host.detail_updated_at(), None);
        assert_eq!(host.gigs_total_disk_space(), None);
        assert_eq!(host.gigs_all_disk_space(), None);
        let serialized = serde_json::to_value(host).unwrap();
        for key in inventory_fields().as_object().unwrap().keys() {
            assert!(
                serialized.get(key).is_none(),
                "unknown field {key} was invented"
            );
        }
    }
}

#[test]
fn zero_capacity_is_distinct_from_unknown() {
    let mut payload = host_json();
    payload["gigs_total_disk_space"] = json!(0);
    payload["gigs_all_disk_space"] = json!(0.0);
    let host: Host = serde_json::from_value(payload).unwrap();
    assert_eq!(host.gigs_total_disk_space(), Some(0.0));
    assert_eq!(host.gigs_all_disk_space(), Some(0.0));
}

#[test]
fn malformed_inventory_types_are_rejected() {
    for (field, value) in [
        ("detail_updated_at", json!("not-a-date")),
        ("cpu_logical_cores", json!(-1)),
        ("gigs_total_disk_space", json!("unknown")),
        ("disk_encryption_enabled", json!("false")),
    ] {
        let mut payload = host_json();
        payload[field] = value;
        assert!(
            serde_json::from_value::<Host>(payload).is_err(),
            "accepted invalid {field}"
        );
    }
}
