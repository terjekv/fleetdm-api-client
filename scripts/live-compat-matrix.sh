#!/usr/bin/env bash
set -euo pipefail

log() { printf "[live-compat] %s\n" "$*"; }
err() { printf "[live-compat][error] %s\n" "$*" >&2; }

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"
VERSIONS=${FLEET_COMPAT_VERSIONS:-"v4.87.0 latest"}
MODE=${FLEET_COMPAT_MODE:-"smoke"}

case "$MODE" in
  smoke) mode_args=(--smoke) ;;
  full) mode_args=() ;;
  *)
    err "FLEET_COMPAT_MODE must be 'smoke' or 'full'"
    exit 1
    ;;
esac

read -r -a versions <<<"$VERSIONS"
if [[ ${#versions[@]} -eq 0 ]]; then
  err "FLEET_COMPAT_VERSIONS must contain at least one version"
  exit 1
fi

for version in "${versions[@]}"; do
  log "Testing Fleet ${version} (${MODE})"
  if [[ "$version" == "latest" ]]; then
    "$ROOT_DIR/scripts/live-test.sh" --reset "${mode_args[@]}"
  else
    "$ROOT_DIR/scripts/live-test.sh" --reset --tag="$version" "${mode_args[@]}"
  fi
done

log "Compatibility matrix completed"
