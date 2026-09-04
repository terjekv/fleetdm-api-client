# FleetDM API Client

An unofficial, asynchronous Rust client for the
[Fleet](https://fleetdm.com/) REST API.

The crate combines ergonomic typed endpoints for common workflows with a
generated authenticated transport for every route recognized in the vendored
Fleet API documentation. It favors secure defaults, explicit validation, and
preserving useful Fleet error context.

This is an early `0.0.x` release. Follow the
[changelog](CHANGELOG.md) when upgrading, because the public API may evolve
before `1.0`.

## Highlights

- Authentication is enforced at compile time with a typestate builder.
- Remote Fleet servers require HTTPS; plain HTTP is accepted only on loopback.
- The default HTTP client does not follow redirects with bearer credentials.
- Curated endpoints return typed request, response, and domain models.
- Generated `raw_api()` wrappers cover all 241 routes in the pinned Fleet spec.
- Query values and user-controlled path segments are percent-encoded centrally.
- Inputs are validated before network I/O where the client can detect invalid
  state locally.
- JSON and in-memory binary responses are bounded; large software packages are
  streamed.
- HTTP 429 retry behavior is opt-in and supports exponential backoff,
  `Retry-After`, and jitter.
- Sensitive models avoid exposing tokens, passwords, private keys, scripts, and
  file contents through `Debug`.

## Requirements

- Rust 1.88 or newer
- Tokio-compatible async runtime
- A supported Fleet server and API token, or Fleet login credentials

The managed live suite currently exercises a disposable Fleet preview. A
scheduled smoke matrix covers Fleet 4.87 and the latest release. Fleet edition,
license, role, and integration configuration can affect endpoint availability.

## Installation

The package name uses hyphens on crates.io and the conventional underscore form
in Rust code:

```toml
[dependencies]
fleetdm-api-client = "0.0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

For development from an unreleased commit, use a Git dependency or a local path
dependency instead.

## Quick start

```rust
use fleetdm_api_client::{FleetClient, models::HostStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FleetClient::builder("https://fleet.example.com")?
        .with_token("your-api-token")?
        .build();

    let response = client
        .hosts()
        .list()
        .status(HostStatus::Online)
        .per_page(50)
        .send()
        .await?;

    for host in response.hosts() {
        println!("{}: {}", host.hostname(), host.status());
    }

    Ok(())
}
```

`FleetClient::build()` exists only for an authenticated builder. This does not
compile:

```compile_fail
use fleetdm_api_client::FleetClient;

let client = FleetClient::builder("https://fleet.example.com")
    .unwrap()
    .build();
```

## Authentication

API tokens are recommended for automation:

```rust
let client = FleetClient::builder("https://fleet.example.com")?
    .with_token("your-api-token")?
    .build();
```

Fleet email/password login is also supported:

```rust
let client = FleetClient::builder("https://fleet.example.com")?
    .login("admin@example.com", "password")
    .await?
    .build();
```

Public password-reset and SSO setup operations are available through
`FleetClient::builder(...).auth()`. Authenticated user and session operations
are available through `client.auth()`.

MFA-required login is reported as `FleetError::MfaRequired`; the client does not
attempt to automate an interactive MFA challenge.

## API surfaces

### Curated typed API

`FleetClient` provides typed accessors for:

- activities and audit history;
- authentication, invitations, sessions, and users;
- hosts, labels, fleets, and legacy team-compatible routes;
- reports, legacy query-compatible routes, policies, targets, and translation;
- scripts, software, vulnerabilities, and operating-system versions;
- MDM commands, OS settings, setup experience, certificates, and conditional
  access;
- Fleet configuration, integrations, file carves, and version information.

Typed endpoints are the preferred surface. They validate known invariants and
decode Fleet responses into models with narrow accessors.

### Generated raw API

Use `raw_api()` when Fleet documents a route that does not yet have a curated
wrapper:

```rust
let response = client
    .raw_api()
    .ep_get_api_v1_fleet_version(None)
    .await?;

println!("status: {}", response.status());
println!("{}", response.body());
```

Generated wrappers:

- are authenticated through the same `FleetClient` transport;
- encode path and query values through shared helpers;
- support JSON, byte, multipart, and DELETE request bodies where applicable;
- retain successful response status, headers, and up to 64 MiB of response
  bytes;
- use generated method names derived from HTTP method and route.

Route coverage is syntactic. A generated wrapper does not imply a stable typed
payload contract or a dedicated live compatibility test. See
[API coverage](docs/missing_endpoints.md).

## Common workflows

### Reports and policies

```rust
use fleetdm_api_client::models::{CreatePolicyRequest, RunLiveReportRequest};

let policy = client.policies().create(CreatePolicyRequest {
    name: "Full disk encryption".into(),
    query: "SELECT 1 FROM disk_encryption WHERE encrypted = 1".into(),
    description: "Require encrypted storage".into(),
    resolution: Some("Enable FileVault or BitLocker".into()),
    team_id: None,
    platform: Some("darwin,windows".into()),
    critical: Some(true),
    conditional_access_enabled: Some(true),
}).await?;

println!("created policy {}", policy.policy().id());

let result = client.reports().run_live(42, RunLiveReportRequest {
    host_ids: vec![1, 2, 3],
}).await?;
println!("{} hosts responded", result.responded_host_count());
```

### Scripts and uploads

```rust
use fleetdm_api_client::models::{CreateScriptRequest, FileUpload};

let created = client.scripts().create(CreateScriptRequest::new(
    "collect-info.sh",
    "#!/bin/sh\nuname -a\n",
)).await?;

let replacement = FileUpload::new(
    "collect-info.sh",
    b"#!/bin/sh\nuname -a\nid\n".to_vec(),
)?;
client.scripts().update(created.script_id(), replacement).await?;
```

Upload filenames must be safe basenames and upload payloads must be non-empty.
Endpoint-specific size, extension, and mutually exclusive targeting rules are
validated before sending.

### Streaming software packages

Fleet permits multi-gigabyte installers, so package downloads are not buffered
into a single allocation:

```rust
let mut package = client.software().download_package(42, 7).await?;
let mut downloaded = 0_u64;

while let Some(chunk) = package.next_chunk().await? {
    downloaded += chunk.len() as u64;
    // Persist or process `chunk` before requesting the next one.
}

println!("downloaded {downloaded} bytes");
```

Other typed JSON responses are limited to 8 MiB. Raw responses and in-memory
file responses are limited to 64 MiB. Exceeding a limit returns
`FleetError::ResponseTooLarge`.

## Retries

No retry policy is installed by default. To retry HTTP 429 responses:

```rust
use fleetdm_api_client::{FleetClient, RetryPolicy};

let client = FleetClient::builder("https://fleet.example.com")?
    .with_retry_policy(RetryPolicy::conservative())
    .with_token("your-api-token")?
    .build();
```

Transport failures and statuses other than 429 are not retried. This avoids
duplicating non-idempotent operations after ambiguous connection failures. See
[retry policies](docs/retry_policies.md) for presets and per-request behavior.

## Errors

All public operations use `FleetError` and the crate `Result` alias:

```rust
use fleetdm_api_client::FleetError;

match client.hosts().get(999).await {
    Ok(host) => println!("found {}", host.host().hostname()),
    Err(FleetError::NotFound(message)) => eprintln!("not found: {message}"),
    Err(FleetError::Authentication(message)) => eprintln!("auth: {message}"),
    Err(FleetError::RateLimit { retry_after, .. }) => {
        eprintln!("rate limited; Retry-After={retry_after:?}");
    }
    Err(FleetError::ResponseTooLarge { limit }) => {
        eprintln!("response exceeded {limit} bytes");
    }
    Err(error) => eprintln!("Fleet request failed: {error}"),
}
```

Structured Fleet API errors retain server messages, field errors, UUIDs, and
status where available. See [security](SECURITY.md) before logging arbitrary
server error content in a sensitive environment.

## Documentation

- [Documentation index](docs/README.md)
- [Architecture and extension guide](docs/architecture.md)
- [API route coverage](docs/missing_endpoints.md)
- [Retry policies](docs/retry_policies.md)
- [Live testing](docs/live-testing.md)
- [Release process](docs/releasing.md)
- [LDAP role synchronization example](docs/ldap-role-sync.md)
- [Security policy and secure-use notes](SECURITY.md)
- [Changelog](CHANGELOG.md)
- [Runnable examples](examples/)

## Development

Run the normal release checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --locked
cargo test --doc --locked
scripts/update-spec-api.py --check
cargo audit
cargo deny check advisories bans licenses sources
```

Run the disposable Fleet compatibility suite when Docker is available:

```bash
scripts/live-test.sh
```

The full live suite currently contains 116 tests across 27 endpoint modules.
See [live testing](docs/live-testing.md) before targeting an external Fleet
server; the full suite creates, updates, and deletes resources.

Contributions should follow [CONTRIBUTING.md](CONTRIBUTING.md) and the repository
guidelines in [AGENTS.md](AGENTS.md).

## License

Licensed under the [MIT License](LICENSE).

Fleet and FleetDM are trademarks of Fleet Device Management, Inc. This project
is not affiliated with or endorsed by Fleet Device Management, Inc.
