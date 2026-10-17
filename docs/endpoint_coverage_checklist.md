# Fleet API Coverage Checklist (Spec vs Client Code)

Last updated: 2026-10-01
Spec source: https://raw.githubusercontent.com/fleetdm/fleet/main/docs/REST%20API/rest-api.md
Vendored reference: `docs/fleet-rest-api-main.md`
Code scanned: `src/endpoints/raw_api.rs`

## Summary

- Spec routes (unique documented operations): `266`
- Generated `FleetClient::raw_api()` route wrappers: `266`
- Missing spec routes from generated transport coverage: `0`

These counts measure route extraction and wrapper generation only. They do not establish request/response semantics or live Fleet-version compatibility.

## What Changed In This Refresh

- Refreshed the vendored Fleet REST API markdown from upstream `main`.
- Regenerated `src/endpoints/raw_api.rs` from the refreshed route list.
- Added 25 generated wrappers; all 241 previously recognized routes remain.
- The additions below are relative to the 2026-09-04 snapshot. All paths share
  the `/api/v1/fleet` prefix.

| Area | Added operations |
| --- | --- |
| Apple asset declarations | `GET /assets`, `POST /assets`, `GET /assets/:asset_uuid`, `DELETE /assets/:asset_uuid` |
| Custom host vitals | `GET /custom_host_vitals`, `POST /custom_host_vitals`, `PATCH /custom_host_vitals/:id`, `DELETE /custom_host_vitals/:id`, `PUT /hosts/:host_id/custom_host_vitals/:id`, `PUT /spec/custom_host_vitals` |
| Policy automation | `GET /policies/:id/automation_activities`, `POST /policies/:policy_id/reset` |
| Microsoft Graph credentials | `GET /microsoft_graph_credentials`, `PUT /microsoft_graph_credentials` |
| Host management | `DELETE /hosts/:id/commands/:command_uuid`, `POST /hosts/:id/apns_ping`, `POST /hosts/:id/name_template/resend`, `POST /hosts/release_ab`, `POST /host_name_template` |
| Live queries | `POST /queries/run`, `POST /queries/run_by_identifiers` |
| Scripts | `GET /scripts/results/:execution_id` |
| Configuration profiles | `PATCH /configuration_profiles/:profile_uuid` |
| Self-service software | `POST /device/:token/software/install/:software_title_id` |
| Android Enterprise | `DELETE /android_enterprise` |

The upstream snapshot also changes documented fields and behavior on existing
routes. This refresh updates the source documentation and generated transport;
it does not establish full semantic parity for the curated typed endpoints or
compatibility with every released Fleet version.

## Generated Route Surface

For a low-level wrapper around any route recognized in the pinned documentation, use:

- `client.raw_api()` from `src/endpoints/raw_api.rs`

`raw_api` methods return `ApiResponse` values containing status, headers, and up
to 64 MiB of exact response bytes. They support JSON, byte, and multipart
`ApiRequestBody` payloads.

## Validation

Run before release:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets`
- `cargo test --doc`
- `scripts/update-spec-api.py --check`
- `cargo audit`
- `scripts/live-test.sh` for the disposable local Fleet compatibility suite

External live testing requires explicit authorization because the full suite
mutates Fleet resources. See [live testing](live-testing.md).
