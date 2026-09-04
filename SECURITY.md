# Security policy

## Supported versions

Before `1.0`, security fixes are provided for the most recent published release
and the default branch. Older `0.0.x` releases may require upgrading rather than
receiving a backport.

## Reporting a vulnerability

Do not disclose a suspected vulnerability in a public issue. Use GitHub's
private vulnerability reporting for this repository when available. Include:

- the affected version or commit;
- impact and realistic attack scenario;
- reproduction steps or a minimal proof of concept;
- any suggested mitigation;
- whether the issue is already public or under an embargo.

Maintainers should acknowledge a complete report within seven days. Disclosure
timing will be coordinated after the impact and fix are understood.

Never include real Fleet credentials, API tokens, private keys, enrollment
secrets, or customer data in a report. Use synthetic values and redact HTTP
captures.

## Secure-use model

The built-in client applies the following protections:

- non-loopback Fleet URLs must use HTTPS;
- base URLs cannot contain credentials, paths, queries, or fragments;
- redirects are disabled by default;
- authentication is required before constructing `FleetClient`;
- tokens and common secret-bearing models are redacted from `Debug`;
- user-controlled path and query values are encoded by shared helpers;
- known invalid requests and unsafe multipart names fail before network I/O;
- typed JSON bodies are limited to 8 MiB and raw or in-memory file bodies to
  64 MiB;
- large software packages are streamed;
- retries are opt-in and limited to HTTP 429 responses.

These protections have boundaries:

- A custom `reqwest::Client` supplied through `http_client()` replaces the
  built-in TLS, timeout, and redirect configuration. The caller must configure
  it safely.
- Secrets still exist in process memory and may be returned through explicit
  accessors. Redacted `Debug` output is not memory zeroization.
- Fleet error bodies can contain server-provided context. Treat them as
  potentially sensitive before logging or forwarding them.
- The generated raw API provides transport safety but cannot validate every
  route-specific semantic invariant.
- TLS security ultimately depends on the selected trust roots, the Fleet
  server, and the `reqwest`/Rustls dependency stack.

Use least-privilege Fleet tokens, rotate credentials regularly, restrict log
access, and prefer the curated typed endpoints where available.

## Release security

Crate releases use crates.io trusted publishing from the tag-only GitHub Actions
workflow. The workflow requests a short-lived OIDC credential and does not use a
stored crates.io token. Publication requires a strict version tag on the exact
current `main` commit, matching release metadata, and the complete CI suite.

Only the publication job can request an OIDC token, while GitHub release writes
are isolated in a later job. External actions are pinned to immutable commit
hashes and monitored by Dependabot. See [docs/releasing.md](docs/releasing.md)
for the trusted-publisher binding and operational procedure.
