# Fleet API Route Coverage

Last updated: 2026-09-04
Spec source: https://raw.githubusercontent.com/fleetdm/fleet/main/docs/REST%20API/rest-api.md
Vendored reference: `docs/fleet-rest-api-main.md`

## Status

- Spec routes (unique documented operations): `241`
- Generated `FleetClient::raw_api()` route wrappers: `241`
- Routes missing a generated wrapper: `0`

## Notes

- `FleetClient::raw_api()` (`src/endpoints/raw_api.rs`) has a generated wrapper for every route recognized by the pinned documentation parser.
- `raw_api` preserves response bytes (up to the 64 MiB in-memory safety limit) and headers and supports JSON, raw byte, multipart, and DELETE request bodies.
- This is syntactic route coverage, not a claim that each route's payload semantics have a curated typed model or live compatibility test.
- Ergonomic typed endpoint modules cover commonly used workflows and intentionally do not mirror every documented route.
- See [architecture](architecture.md) for the distinction between curated and generated APIs.
