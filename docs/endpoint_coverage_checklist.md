# Fleet API Coverage Checklist (Spec vs Client Code)

Last updated: 2026-09-04
Spec source: https://raw.githubusercontent.com/fleetdm/fleet/main/docs/REST%20API/rest-api.md
Vendored reference: `docs/fleet-rest-api-main.md`
Code scanned: `src/endpoints/raw_api.rs`

## Summary

- Spec routes (unique documented operations): `241`
- Generated `FleetClient::raw_api()` route wrappers: `241`
- Missing spec routes from generated transport coverage: `0`

These counts measure route extraction and wrapper generation only. They do not establish request/response semantics or live Fleet-version compatibility.

## What Changed In This Refresh

- Refreshed the vendored Fleet REST API markdown from upstream `main`.
- Regenerated `src/endpoints/raw_api.rs` from the refreshed route list.
- Picked up the upstream route shift from team/query terminology to fleet/report terminology where present in the current docs.
- Added byte-preserving transport coverage for newly documented routes such as
  report endpoints, fleet endpoints, API-only user creation, REST API permission
  listing, managed account password, recovery lock password, host certificate
  resend, and software web apps.

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
