# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1] - 2026-09-04

### Added

- Typestate-based client construction with API-token and email/password
  authentication.
- Curated typed endpoints for hosts, users, fleets, teams, labels, reports,
  queries, policies, scripts, software, vulnerabilities, MDM workflows,
  configuration, integrations, and related Fleet operations.
- Generated `raw_api()` wrappers for all 241 routes recognized in the pinned
  Fleet REST API documentation.
- Typed error mapping for authentication, MFA, validation, not-found, Premium,
  rate-limit, connection, timeout, serialization, and response-size failures.
- Configurable HTTP 429 retries with exponential backoff, jitter, and
  `Retry-After` support.
- Streaming downloads for large software packages and metadata-preserving
  in-memory downloads for bounded files.
- Offline mock-server integration tests, route-generation coverage, examples,
  and a deterministic disposable Fleet preview suite.
- CI checks for Rust 1.88 and stable, formatting, Clippy, tests, documentation,
  generated-code drift, and dependency advisories.

### Changed

- Renamed the crates.io package from the underscore spelling to
  `fleetdm-api-client`; the Rust crate path remains `fleetdm_api_client`.
- Adopted Rust 2024 with a minimum supported Rust version of 1.88.
- Made edition- and role-dependent configuration sections optional when Fleet
  omits them.
- Kept legacy team/query terminology available where needed while exposing
  current fleet/report routes.

### Fixed

- Corrected request paths, HTTP methods, multipart fields, query encoding, and
  response models across the curated endpoint surface.
- Preserved Fleet server messages, field errors, UUIDs, and rate-limit context.
- Rebuilt multipart requests safely for explicit 429 retries.
- Escaped LDAP usernames before substituting them into DN and search-filter
  templates in the role-synchronization example.
- Made live tests deterministic across Community and Premium/configuration
  capability differences.

### Security

- Require HTTPS for non-loopback Fleet URLs and reject base URLs containing
  credentials, paths, queries, or fragments.
- Disable redirects in the built-in HTTP client to avoid forwarding bearer
  credentials or request bodies to another origin.
- Validate authentication tokens, upload filenames, multipart field names,
  identifiers, and mutually exclusive request state before network I/O.
- Redact sensitive values from `Debug` implementations.
- Limit typed JSON responses to 8 MiB and raw or in-memory file responses to
  64 MiB; software package downloads use streaming instead.
- Do not retry transport failures or non-429 statuses, avoiding accidental
  duplicate mutations after ambiguous failures.
- Removed the tracked local environment file and retired the unused Docker
  Compose setup; disposable live tests now use Fleet Preview exclusively.

[Unreleased]: https://github.com/terjekv/fleetdm-api-client/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/terjekv/fleetdm-api-client/releases/tag/v0.0.1
