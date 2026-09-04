# Live testing

Live tests validate the client against a real Fleet server. The normal workflow
uses Fleet's local preview environment and is disposable; external-server runs
require additional care.

## Coverage

The complete suite currently contains 116 asynchronous tests across 27 endpoint
modules. The strongest areas are authentication, hosts, certificates, setup
experience, software, and users.

The suite exercises:

- token and password authentication, public auth, SSO error paths, password
  changes, session behavior, and typed authentication failures;
- list, filter, sorting, pagination, detail, create, update, and delete behavior;
- deterministic report, label, policy, script, user, and optional Premium fleet
  fixtures;
- reports and live queries against a preview host;
- software, vulnerability, MDM, certificate, configuration, integration, and
  setup-experience response compatibility;
- Community/Premium and optional-integration error classification.

A passing test may deliberately stop after verifying that Fleet reports an
optional feature as unavailable. The default Community preview cannot exercise
successful Premium, configured SSO, SMTP/SES, APNs, ABM/VPP, SCIM, or real-device
MDM workflows. Generated raw route coverage is tested offline rather than by
calling all 241 routes live.

## Prerequisites

- Docker with a running daemon
- `curl`
- `tar`
- `mktemp`
- `shasum` or `sha256sum`
- Rust 1.88 or newer

## Disposable preview

Run the full suite:

```bash
scripts/live-test.sh
```

The script:

1. resolves the latest released `fleetctl`;
2. downloads its archive and published checksum;
3. verifies the archive before execution;
4. starts `fleetctl preview` on loopback;
5. authenticates through this Rust client;
6. seeds deterministic fixtures and records their exact IDs in a temporary
   manifest;
7. runs live tests serially;
8. restores the fixture host and deletes seeded resources, including after test
   failures;
9. stops the preview unless `--keep-running` was requested.

Useful variants:

```bash
# Small stable subset
scripts/live-test.sh --smoke

# Specific Fleet version
scripts/live-test.sh --tag=v4.87.0

# Current Fleet main preview
scripts/live-test.sh --tag=main

# One test or module name filter
scripts/live-test.sh --test-filter=live::hosts

# Reset preview state before starting
scripts/live-test.sh --reset

# Exercise Premium paths with an authorized test license
scripts/live-test.sh --license-key="your-test-license"
```

The full suite defaults to one test thread because tests share and mutate server
state. Increase `--test-threads` only after verifying the selected tests are
independent.

## External Fleet servers

The safe external path is the read-only smoke suite:

```bash
FLEET_URL="https://fleet.example.com" \
FLEET_EMAIL="admin@example.com" \
FLEET_PASSWORD="secret" \
  scripts/live-test.sh --external --smoke
```

The script rejects non-loopback HTTP URLs. It also refuses the mutating external
suite unless `--allow-destructive` is explicit:

```bash
FLEET_URL="https://fleet.example.com" \
FLEET_EMAIL="admin@example.com" \
FLEET_PASSWORD="secret" \
  scripts/live-test.sh --external --allow-destructive
```

That command can create, update, move, and delete Fleet resources. Use only a
dedicated test instance after reviewing the selected tests.

## Compatibility matrix

Run the pinned baseline and latest Fleet release:

```bash
scripts/live-compat-matrix.sh
```

The matrix defaults to smoke coverage. Set `FLEET_COMPAT_MODE=full` to run all
tests for each version. The scheduled GitHub Actions workflow uses smoke mode for
Fleet 4.87 and latest.

## Secrets and diagnostics

Credentials are passed through environment variables, not command-line
arguments. Temporary token and fixture manifests are deleted during cleanup.
Never commit preview logs, manifests, API tokens, Fleet license keys, or external
server credentials.

On failure, the script prints recent preview logs and still attempts fixture and
preview cleanup. A failure before tests begin is usually distinguishable from a
client failure by the phase marker in the output.
