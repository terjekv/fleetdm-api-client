# Repository Guidelines

## Scope And Working Style

- These instructions apply to the repository root and all subdirectories.
- Read the relevant implementation, tests, and documentation before changing them.
- Keep changes scoped to the requested outcome. Do not reformat, refactor, upgrade
  dependencies, or alter public APIs unless the task requires it.
- Preserve user-authored and unrelated worktree changes. Never reset or overwrite
  them to obtain a clean tree.
- Prefer the smallest clear, testable solution over speculative abstraction or
  configurability.
- Verify material assumptions against repository code, the vendored Fleet API
  documentation, or focused tests. State remaining uncertainty instead of guessing.

## Repository Map

- `src/client.rs` owns client construction, authentication typestate, and endpoint
  accessors.
- `src/http/` owns shared request sending, response decoding, error mapping, query
  encoding, and retry integration.
- `src/endpoints/` contains the curated, ergonomic Fleet API surface.
- `src/models/` contains request, response, and domain wire types.
- `src/paths.rs` is the central location for API path constants and builders.
- `src/endpoints/raw_api.rs` is generated from
  `docs/fleet-rest-api-main.md` by `scripts/update-spec-api.py`.
- `tests/` contains offline integration and coverage tests. `tests/live/` contains
  opt-in tests against a real Fleet instance.
- `examples/` are part of the user-facing API and must continue to compile.

## Verification

- Run `cargo fmt --all -- --check` for Rust changes.
- Run `cargo clippy --all-targets --all-features -- -D warnings` before considering
  code changes complete.
- Run `cargo test --all-targets` for the normal offline suite and example builds.
- Run `cargo test --doc` when public documentation or examples change.
- Run `scripts/update-spec-api.py --check` when the vendored API specification,
  generated raw API, route coverage, or generator changes.
- Prefer a focused test while iterating, then run the applicable full checks above.
- If a check cannot run, report exactly what was not run and why.

## Architecture And API Boundaries

- Preserve authentication as a compile-time invariant. The unauthenticated builder
  must not construct a usable `FleetClient`; keep authenticated-only behavior on
  the authenticated typestate.
- Route requests through `FleetClient` and the shared helpers in `src/http/` so
  authentication, retries, response decoding, and error handling remain consistent.
  Do not duplicate transport logic inside individual endpoint modules.
- Use `FleetError` and the crate `Result` alias as the public error surface. Prefer
  actionable, specific errors and retain useful server response context.
- Keep paths in `src/paths.rs`. Encode user-controlled path segments and query
  values with the shared helpers; never construct unescaped request URLs by string
  interpolation.
- Put curated operations in the matching `src/endpoints/<area>.rs` module and wire
  them through `src/endpoints/mod.rs` and a `FleetClient` accessor.
- Put wire and domain types in the matching `src/models/<area>.rs` module and expose
  them consistently through `src/models/mod.rs`.
- Model validated or mutually exclusive state explicitly. Prefer newtypes, enums,
  validating constructors, and typestate when they prevent meaningful invalid
  states; use a simpler type when they do not.
- Keep response representation private where the existing model style does so, and
  provide narrow accessors. Request fields may remain public when callers are meant
  to construct the value directly.
- Treat public methods, exported types, serialized field names, and feature flags as
  compatibility commitments. Preserve legacy aliases and deprecations unless a
  breaking change is explicitly requested.
- Use typed responses for curated endpoints. Reserve `serde_json::Value` and the raw
  API for genuinely open-ended or not-yet-curated payloads.

## Fleet API And Generated Code

- Treat `docs/fleet-rest-api-main.md` as the pinned local source for documented
  Fleet routes. When behavior varies by Fleet version, preserve compatibility where
  practical and test the variants explicitly.
- Do not hand-edit `src/endpoints/raw_api.rs`. Update the vendored specification or
  `scripts/update-spec-api.py`, regenerate, and review the resulting diff.
- Use `scripts/update-spec-api.py --fetch` only when intentionally refreshing from
  Fleet's upstream documentation. It performs network access and can create a large
  generated diff.
- A new curated endpoint normally requires the path definition, model types,
  endpoint implementation, module exports, client accessor, focused tests, and
  user-facing documentation when applicable.
- Keep serde names faithful to Fleet's wire format. Use explicit `rename`, aliases,
  defaults, and flattened compatibility fields only when supported by observed or
  documented payloads.

## Rust Standards

- Follow idiomatic Rust and the repository's existing edition and minimum toolchain
  declared in `Cargo.toml`.
- Use small functions, explicit contracts, early returns, and flat control flow.
  Avoid cleverness, hidden side effects, unnecessary cloning, and premature helpers.
- Keep invariants close to the data they protect. Validate requests before sending
  them and fail before performing network I/O when input is invalid.
- Use exhaustive enums for closed Fleet value sets. If Fleet may add values without
  notice, choose a forward-compatible representation deliberately and test unknown
  values.
- Do not swallow errors or add `allow` attributes merely to make builds pass. Remove
  dead code or address the underlying warning.
- Add rustdoc for public APIs, including route and behavior notes when they help a
  caller use the API correctly.

## Tests

- Add or update tests whenever behavior, serialization, URL construction, retry
  logic, error mapping, or a public contract changes.
- Use offline deterministic tests by default. Exercise HTTP behavior with the
  existing mock-server patterns rather than depending on a live Fleet server.
- Keep each test focused on one behavior. Include success, invalid-input, error, and
  compatibility cases where relevant.
- Assert the externally meaningful contract: HTTP method and path, encoded query or
  body, deserialized result, and error variant. Avoid tests coupled only to internal
  implementation details.
- Maintain `tests/coverage.rs` and the generator check so the raw API remains aligned
  with the vendored specification.

## Live Tests And Secrets

- Live tests are opt-in and may create, update, or delete Fleet resources. Do not run
  them against an external Fleet instance unless the user explicitly authorizes it.
- Use `scripts/live-test.sh` for the managed live workflow. Its default preview is
  disposable; `--external --smoke` is the read-only external path, while mutating an
  external instance requires the explicit `--allow-destructive` flag.
- Never commit Fleet credentials, API tokens, LDAP passwords, license keys, `.env`
  contents, token manifests, fixture manifests, or logs containing secrets.
- Avoid logging authentication headers, passwords, tokens, or sensitive response
  bodies. Examples must use obvious placeholders rather than plausible credentials.

## Documentation And Change Discipline

- Update `README.md`, rustdoc, examples, and files under `docs/` when public behavior
  or supported routes change. Keep examples compileable and consistent with the
  current API.
- Keep stable guidance separate from generated upstream documentation and temporary
  investigation notes.
- Before handing off, review the diff for accidental generated churn, secrets,
  unrelated edits, compatibility breaks, and missing validation.
- Summarize what changed and list the checks actually run. Call out remaining risks
  or unverified live behavior explicitly.
