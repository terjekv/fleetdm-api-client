#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
test_root="$(mktemp -d)"
trap 'rm -rf "$test_root"' EXIT

fail() { echo "error: $*" >&2; exit 1; }

mkdir -p "$test_root/bin" "$test_root/archive"
cat >"$test_root/archive/fleetctl" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"$LIVE_TEST_ROOT/fleetctl-calls"
EOF
chmod +x "$test_root/archive/fleetctl"
tar -czf "$test_root/fleetctl.tar.gz" -C "$test_root/archive" fleetctl
if command -v shasum >/dev/null 2>&1; then
  checksum=$(shasum -a 256 "$test_root/fleetctl.tar.gz" | awk '{print $1}')
else
  checksum=$(sha256sum "$test_root/fleetctl.tar.gz" | awk '{print $1}')
fi

# Exercise the real script, including archive verification and preview arguments,
# without network access, Docker, or a Fleet server.
cat >"$test_root/bin/curl" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
output=""
url=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -o) output=$2; shift ;;
    https://*|http://*) url=$1 ;;
  esac
  shift
done
printf '%s\n' "$url" >>"$LIVE_TEST_ROOT/curl-calls"
case "$url" in
  https://github.com/fleetdm/fleet/releases/latest)
    printf 'https://github.com/fleetdm/fleet/releases/tag/fleet-v4.93.0'
    ;;
  https://github.com/fleetdm/fleet/releases/download/*/checksums.txt)
    for version in v4.87.0 v4.93.0; do
      for platform in macos linux_amd64 linux_arm64; do
        printf '%s  fleetctl_%s_%s.tar.gz\n' "$LIVE_TEST_CHECKSUM" "$version" "$platform"
      done
    done >"$output"
    ;;
  https://github.com/fleetdm/fleet/releases/download/*/*.tar.gz)
    cp "$LIVE_TEST_ROOT/fleetctl.tar.gz" "$output"
    ;;
  http://localhost:1337/api/v1/fleet/version) printf '200' ;;
  *) echo "unexpected URL: $url" >&2; exit 1 ;;
esac
EOF
cat >"$test_root/bin/cargo" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
chmod +x "$test_root/bin/curl" "$test_root/bin/cargo"

run_preview() {
  : >"$test_root/curl-calls"
  : >"$test_root/fleetctl-calls"
  PATH="$test_root/bin:$PATH" \
    LIVE_TEST_ROOT="$test_root" LIVE_TEST_CHECKSUM="$checksum" \
    FLEET_URL="http://localhost:1337" FLEET_EMAIL="admin@example.com" \
    FLEET_PASSWORD="test-password" FLEET_PREVIEW_TAG="" \
    FLEET_PREVIEW_LICENSE_KEY="" FLEET_TEST_FILTER="" \
    "$script_dir/live-test.sh" --smoke "$@" >"$test_root/output" 2>&1
}

for tag in v4.87.0 4.87.0 fleet-v4.87.0; do
  run_preview "--tag=$tag"
  grep -Fq '/download/fleet-v4.87.0/fleetctl_v4.87.0_' "$test_root/curl-calls" ||
    fail "$tag did not download matching fleetctl"
  if grep -Fq '/releases/latest' "$test_root/curl-calls"; then
    fail "$tag unexpectedly resolved the latest release"
  fi
  grep -Fxq 'preview --preview-config fleet-v4.87.0 --tag v4.87.0' "$test_root/fleetctl-calls" ||
    fail "$tag did not pin the server and preview configuration"
done

for tag in '' latest main; do
  if [[ -n "$tag" ]]; then
    run_preview "--tag=$tag"
  else
    run_preview
  fi
  grep -Fq '/download/fleet-v4.93.0/fleetctl_v4.93.0_' "$test_root/curl-calls" ||
    fail "${tag:-default} did not download latest fleetctl"
  if [[ "$tag" == main ]]; then
    expected='preview --preview-config main --tag main'
  else
    expected='preview --preview-config fleet-v4.93.0 --tag v4.93.0'
  fi
  grep -Fxq "$expected" "$test_root/fleetctl-calls" ||
    fail "${tag:-default} used unexpected preview arguments"
done

checksum=invalid
if run_preview --tag=v4.87.0; then
  fail 'accepted an archive with an incorrect checksum'
fi
grep -Fq 'Checksum verification failed' "$test_root/output" ||
  fail 'unexpected error for an incorrect checksum'
[[ ! -s "$test_root/fleetctl-calls" ]] || fail 'executed an unverified archive'

echo 'Live preview release selection and checksum checks passed'
