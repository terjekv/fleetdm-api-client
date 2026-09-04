#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

log() { printf "[live-test] %s\n" "$*"; }
err() { printf "[live-test][error] %s\n" "$*" >&2; }
warn() { printf "[live-test][warn] %s\n" "$*" >&2; }

usage() {
  cat <<'EOF'
Usage: scripts/live-test.sh [options]

Starts an official Fleet preview environment by default, waits for it to accept
API logins, then runs the live integration suite against it.

Options:
  --keep-running           Leave the preview environment running after tests
  --reset                  Reset the preview environment before starting
  --external               Use an already running Fleet server; do not start/stop preview
  --allow-destructive      Allow fixture seeding and mutating tests with --external
  --smoke                  Run a small stable subset of live tests
  --license-key=KEY        Pass a Fleet Premium trial/license key to preview
  --tag=TAG                Start a specific Fleet version tag in preview
  --fleet-url=URL          Fleet base URL (default: http://localhost:1337)
  --email=EMAIL            Admin login email (default: admin@example.com)
  --password=PASSWORD      Admin login password (default: preview1337#)
  --test-filter=FILTER     Cargo test filter for the live suite
  --test-threads=N         Test thread count (default: 1)
  --help                   Show this help

Environment variables:
  FLEET_URL
  FLEET_EMAIL
  FLEET_PASSWORD
  FLEET_PREVIEW_LICENSE_KEY
  FLEET_PREVIEW_TAG
  FLEET_TEST_FILTER
  FLEET_TEST_THREADS
EOF
}

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    err "$1 not found in PATH."
    exit 1
  fi
}

detect_checksum_command() {
  if command -v shasum >/dev/null 2>&1; then
    CHECKSUM_CMD=(shasum -a 256)
    return
  fi
  if command -v sha256sum >/dev/null 2>&1; then
    CHECKSUM_CMD=(sha256sum)
    return
  fi

  err "Neither shasum nor sha256sum is available for checksum verification."
  exit 1
}

latest_release_tag() {
  local effective_url
  effective_url=$(curl -fsSLI -o /dev/null -w '%{url_effective}' \
    "https://github.com/fleetdm/fleet/releases/latest")
  basename "$effective_url"
}

set_release_metadata_from_tag() {
  local raw_tag=${1:-}

  if [[ -n "$raw_tag" && ! "$raw_tag" =~ ^(fleet-)?v?[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    warn "Preview tag '$raw_tag' is not a released Fleet version. Using the latest released fleetctl."
  fi

  FLEETCTL_RELEASE_TAG=$(latest_release_tag)
  if [[ "$FLEETCTL_RELEASE_TAG" =~ ^fleet-(v[0-9]+\.[0-9]+\.[0-9]+)$ ]]; then
    FLEETCTL_ASSET_VERSION=${BASH_REMATCH[1]}
    return
  fi

  err "Unable to parse latest Fleet release tag: $FLEETCTL_RELEASE_TAG"
  exit 1
}

select_asset_name() {
  local os arch
  os=$(uname -s)
  arch=$(uname -m)

  case "$os" in
    Darwin)
      case "$arch" in
        arm64|x86_64) FLEETCTL_ASSET_NAME="fleetctl_${FLEETCTL_ASSET_VERSION}_macos.tar.gz" ;;
        *)
          err "Unsupported macOS architecture: $arch"
          exit 1
          ;;
      esac
      ;;
    Linux)
      case "$arch" in
        x86_64|amd64) FLEETCTL_ASSET_NAME="fleetctl_${FLEETCTL_ASSET_VERSION}_linux_amd64.tar.gz" ;;
        arm64|aarch64) FLEETCTL_ASSET_NAME="fleetctl_${FLEETCTL_ASSET_VERSION}_linux_arm64.tar.gz" ;;
        *)
          err "Unsupported Linux architecture: $arch"
          exit 1
          ;;
      esac
      ;;
    *)
      err "Unsupported operating system: $os"
      exit 1
      ;;
  esac
}

download_fleetctl() {
  local release_base archive_path checksums_path expected actual

  if [[ -z "$TMP_WORKDIR" ]]; then
    TMP_WORKDIR=$(mktemp -d "${TMPDIR:-/tmp}/fleet-live.XXXXXX")
    PREVIEW_LOG="$TMP_WORKDIR/fleet-preview.log"
  fi
  archive_path="$TMP_WORKDIR/$FLEETCTL_ASSET_NAME"
  checksums_path="$TMP_WORKDIR/checksums.txt"

  release_base="https://github.com/fleetdm/fleet/releases/download/${FLEETCTL_RELEASE_TAG}"
  log "Downloading fleetctl ${FLEETCTL_ASSET_VERSION} for ${FLEETCTL_ASSET_NAME}"
  curl -fsSL "${release_base}/checksums.txt" -o "$checksums_path"
  curl -fsSL "${release_base}/${FLEETCTL_ASSET_NAME}" -o "$archive_path"

  expected=$(awk -v file="$FLEETCTL_ASSET_NAME" '$2 == file { print $1 }' "$checksums_path")
  if [[ -z "$expected" ]]; then
    err "Did not find checksum entry for ${FLEETCTL_ASSET_NAME} in checksums.txt"
    exit 1
  fi

  actual=$("${CHECKSUM_CMD[@]}" "$archive_path" | awk '{print $1}')
  if [[ "$actual" != "$expected" ]]; then
    err "Checksum verification failed for ${FLEETCTL_ASSET_NAME}"
    exit 1
  fi

  tar -xzf "$archive_path" -C "$TMP_WORKDIR"
  FLEETCTL_BIN=$(find "$TMP_WORKDIR" -type f -name fleetctl -print -quit)
  if [[ ! -f "$FLEETCTL_BIN" ]]; then
    err "Downloaded archive did not contain fleetctl"
    exit 1
  fi
  chmod +x "$FLEETCTL_BIN"
}

FLEET_URL=${FLEET_URL:-"http://localhost:1337"}
FLEET_EMAIL=${FLEET_EMAIL:-"admin@example.com"}
FLEET_PASSWORD=${FLEET_PASSWORD:-"preview1337#"}
FLEET_PREVIEW_LICENSE_KEY=${FLEET_PREVIEW_LICENSE_KEY:-""}
FLEET_PREVIEW_TAG=${FLEET_PREVIEW_TAG:-""}
FLEET_TEST_FILTER=${FLEET_TEST_FILTER:-""}
FLEET_TEST_THREADS=${FLEET_TEST_THREADS:-"1"}
KEEP_RUNNING=0
RESET_FIRST=0
EXTERNAL=0
ALLOW_DESTRUCTIVE=0
SMOKE=0
CHECKSUM_CMD=()
TMP_WORKDIR=""
PREVIEW_LOG=""
PREVIEW_PID=""
FLEETCTL_BIN=""
FLEETCTL_RELEASE_TAG=""
FLEETCTL_ASSET_VERSION=""
FLEETCTL_ASSET_NAME=""
FLEET_FIXTURE_FILE=""
FLEET_TOKEN_FILE=""
FIXTURE_SEEDED=0

for arg in "$@"; do
  case "$arg" in
    --keep-running) KEEP_RUNNING=1 ;;
    --reset) RESET_FIRST=1 ;;
    --external) EXTERNAL=1 ;;
    --allow-destructive) ALLOW_DESTRUCTIVE=1 ;;
    --smoke) SMOKE=1 ;;
    --license-key=*) FLEET_PREVIEW_LICENSE_KEY=${arg#*=} ;;
    --tag=*) FLEET_PREVIEW_TAG=${arg#*=} ;;
    --fleet-url=*) FLEET_URL=${arg#*=} ;;
    --email=*) FLEET_EMAIL=${arg#*=} ;;
    --password=*) FLEET_PASSWORD=${arg#*=} ;;
    --test-filter=*) FLEET_TEST_FILTER=${arg#*=} ;;
    --test-threads=*) FLEET_TEST_THREADS=${arg#*=} ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      err "Unknown arg: $arg"
      usage
      exit 1
      ;;
  esac
done

require_command curl
require_command mktemp

if [[ $EXTERNAL -eq 0 ]]; then
  require_command tar
  detect_checksum_command
fi

if [[ $SMOKE -eq 1 && -n "$FLEET_TEST_FILTER" ]]; then
  err "--smoke and --test-filter cannot be used together"
  exit 1
fi

if [[ $EXTERNAL -eq 1 ]]; then
  if [[ -z "$FLEET_URL" || -z "$FLEET_EMAIL" || -z "$FLEET_PASSWORD" ]]; then
    err "--external requires FLEET_URL, FLEET_EMAIL, and FLEET_PASSWORD"
    exit 1
  fi
  case "$FLEET_URL" in
    http://localhost|http://localhost/*|http://localhost:*|\
    http://127.0.0.1|http://127.0.0.1/*|http://127.0.0.1:*|\
    http://\[::1\]|http://\[::1\]/*|http://\[::1\]:*) ;;
    http://*)
      err "Refusing to send Fleet credentials to a non-loopback HTTP URL. Use HTTPS."
      exit 1
      ;;
  esac
  if [[ $ALLOW_DESTRUCTIVE -eq 0 && $SMOKE -eq 0 ]]; then
    err "Refusing to run the mutating live suite against an external server without --allow-destructive. Use --smoke for read-only checks."
    exit 1
  fi
fi

if [[ -z "$TMP_WORKDIR" ]]; then
  TMP_WORKDIR=$(mktemp -d "${TMPDIR:-/tmp}/fleet-live.XXXXXX")
  PREVIEW_LOG="$TMP_WORKDIR/fleet-preview.log"
fi

cleanup() {
  local exit_code=$?

  if [[ $FIXTURE_SEEDED -eq 1 && -f "$FLEET_FIXTURE_FILE" ]]; then
    log "Cleaning deterministic live fixtures"
    if ! FLEET_URL="$FLEET_URL" \
      FLEET_EMAIL="$FLEET_EMAIL" \
      FLEET_PASSWORD="$FLEET_PASSWORD" \
      cargo run --quiet --features live-tests --example live-fixture -- \
        cleanup "$FLEET_FIXTURE_FILE"; then
      warn "Deterministic fixture cleanup failed"
    fi
  fi

  if [[ $EXTERNAL -eq 0 && $KEEP_RUNNING -eq 0 && -n "$FLEETCTL_BIN" ]]; then
    log "Stopping Fleet preview"
    "$FLEETCTL_BIN" preview stop >/dev/null 2>&1 || true
  elif [[ $EXTERNAL -eq 0 && $KEEP_RUNNING -eq 1 ]]; then
    log "Leaving Fleet preview running (--keep-running set)"
  fi

  if [[ $exit_code -ne 0 && -n "$PREVIEW_LOG" ]]; then
    err "Preview/test run failed. Recent preview log output:"
    tail -n 80 "$PREVIEW_LOG" >&2 || true
  fi

  if [[ -n "$TMP_WORKDIR" ]]; then
    rm -rf "$TMP_WORKDIR"
  fi

  exit "$exit_code"
}

trap cleanup EXIT

if [[ $EXTERNAL -eq 0 ]]; then
  set_release_metadata_from_tag "$FLEET_PREVIEW_TAG"
  select_asset_name
  download_fleetctl

  if [[ $RESET_FIRST -eq 1 ]]; then
    log "Resetting Fleet preview state"
    "$FLEETCTL_BIN" preview reset >/dev/null 2>&1 || true
  fi

  preview_cmd=("$FLEETCTL_BIN" preview)
  if [[ -n "$FLEET_PREVIEW_LICENSE_KEY" ]]; then
    preview_cmd+=(--license-key "$FLEET_PREVIEW_LICENSE_KEY")
  fi
  if [[ -n "$FLEET_PREVIEW_TAG" ]]; then
    preview_cmd+=(--tag "$FLEET_PREVIEW_TAG")
  fi

  log "Starting Fleet preview"
  "${preview_cmd[@]}" >"$PREVIEW_LOG" 2>&1 &
  PREVIEW_PID=$!
else
  log "Using external Fleet server at $FLEET_URL"
fi

log "Waiting for Fleet to become ready at $FLEET_URL"
READY=0
ATTEMPTS=120
SLEEP_SECONDS=5
VERSION_URL="${FLEET_URL%/}/api/v1/fleet/version"

for _ in $(seq 1 "$ATTEMPTS"); do
  http_code=$(curl -sS -o /dev/null -w '%{http_code}' "$VERSION_URL" || true)
  if [[ "$http_code" =~ ^[1-4][0-9][0-9]$ ]]; then
    READY=1
    break
  fi

  if [[ -n "$PREVIEW_PID" ]] && ! kill -0 "$PREVIEW_PID" >/dev/null 2>&1; then
    if wait "$PREVIEW_PID"; then
      PREVIEW_PID=""
    else
      err "fleetctl preview exited before Fleet became ready"
      exit 1
    fi
  fi

  sleep "$SLEEP_SECONDS"
done

if [[ $READY -ne 1 ]]; then
  err "Fleet did not become ready after $((ATTEMPTS * SLEEP_SECONDS)) seconds"
  exit 1
fi

if [[ $EXTERNAL -eq 0 && -n "$PREVIEW_PID" ]]; then
  log "Waiting for Fleet preview population to complete"
  if wait "$PREVIEW_PID"; then
    PREVIEW_PID=""
  else
    err "fleetctl preview failed while populating the test instance"
    exit 1
  fi
fi

if [[ $EXTERNAL -eq 0 || $ALLOW_DESTRUCTIVE -eq 1 ]]; then
  FLEET_FIXTURE_FILE="$TMP_WORKDIR/live-fixture.json"
  log "Seeding deterministic live fixtures"
  FLEET_URL="$FLEET_URL" \
  FLEET_EMAIL="$FLEET_EMAIL" \
  FLEET_PASSWORD="$FLEET_PASSWORD" \
    cargo run --quiet --features live-tests --example live-fixture -- \
      setup "$FLEET_FIXTURE_FILE"
  FIXTURE_SEEDED=1
else
  FLEET_TOKEN_FILE="$TMP_WORKDIR/live-token.json"
  log "Creating shared token for read-only smoke tests"
  FLEET_URL="$FLEET_URL" \
  FLEET_EMAIL="$FLEET_EMAIL" \
  FLEET_PASSWORD="$FLEET_PASSWORD" \
    cargo run --quiet --features live-tests --example live-fixture -- \
      authenticate "$FLEET_TOKEN_FILE"
fi

run_live_test() {
  local filter=${1:-}
  local exact=${2:-0}
  local test_cmd=(cargo test --features live-tests --test live)
  if [[ -n "$filter" ]]; then
    test_cmd+=("$filter")
  fi
  test_cmd+=(-- --test-threads="$FLEET_TEST_THREADS" --nocapture)
  if [[ $exact -eq 1 ]]; then
    test_cmd+=(--exact)
  fi

  FLEET_LIVE=1 \
  FLEET_URL="$FLEET_URL" \
  FLEET_EMAIL="$FLEET_EMAIL" \
  FLEET_PASSWORD="$FLEET_PASSWORD" \
  FLEET_FIXTURE_FILE="$FLEET_FIXTURE_FILE" \
  FLEET_TOKEN_FILE="$FLEET_TOKEN_FILE" \
    "${test_cmd[@]}"
}

if [[ $SMOKE -eq 1 ]]; then
  log "Running live smoke tests"
  smoke_filters=(
    live::version::version_smoke
    live::authentication::login_with_credentials
    live::authentication::get_me
    live::users::list_users
    live::hosts::list_hosts_smoke
    live::labels::list_labels
    live::software::list_software
    live::queries::list_reports_smoke
    live::teams::list_teams
    live::targets::search_targets
  )
  if [[ $FIXTURE_SEEDED -eq 1 ]]; then
    smoke_filters+=(live::fixture::seeded_fixture_contract)
  fi
  for filter in "${smoke_filters[@]}"; do
    run_live_test "$filter" 1
  done
else
  log "Running live tests"
  run_live_test "$FLEET_TEST_FILTER"
fi

log "Live test run completed"
