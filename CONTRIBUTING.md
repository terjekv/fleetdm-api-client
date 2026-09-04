# Contributing

Thank you for helping improve `fleetdm-api-client`.

## Development setup

Install Rust 1.88 or newer and clone the repository. The normal test suite is
offline and does not require a Fleet server:

```bash
cargo test --all-targets --locked
```

Read [AGENTS.md](AGENTS.md) before changing code. It defines the repository
architecture, compatibility boundaries, validation rules, and required checks.

## Making changes

- Keep changes focused and preserve unrelated worktree changes.
- Add or update focused tests for behavior and public-contract changes.
- Put API paths in `src/paths.rs`, models in `src/models/`, and curated endpoint
  behavior in `src/endpoints/`.
- Route requests through `FleetClient` and the shared `src/http/` transport.
- Add rustdoc for public APIs and update examples or user documentation when
  behavior changes.
- Record user-visible changes under `Unreleased` in [CHANGELOG.md](CHANGELOG.md).

Do not edit `src/endpoints/raw_api.rs` manually. It is generated from the pinned
Fleet documentation:

```bash
scripts/update-spec-api.py --check
```

If the check reports drift, update the generator or intentionally refresh the
vendored specification and regenerate the file. Review generated diffs before
committing them.

## Required checks

Run these before submitting a change:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --all-targets --locked
cargo test --doc --locked
scripts/update-spec-api.py --check
```

Run `cargo audit` when dependencies or the lockfile change.

## Live compatibility tests

The default live workflow creates a disposable local Fleet preview:

```bash
scripts/live-test.sh
```

The full suite mutates Fleet resources. Never point it at an external instance
without reading [docs/live-testing.md](docs/live-testing.md) and explicitly
opting into destructive behavior. Do not commit credentials, API tokens,
license keys, fixture manifests, or preview logs.

## Pull requests

Describe the externally observable behavior, compatibility impact, and checks
run. Call out anything that could not be verified, especially Fleet-version,
Premium, MDM, or external-integration behavior.
