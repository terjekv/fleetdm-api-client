#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "error: $*" >&2
  exit 1
}

read_package_field() {
  local key="$1"

  awk -F= -v key="$key" '
    /^\[package\]$/ {
      in_package = 1
      next
    }
    /^\[/ && in_package {
      exit
    }
    in_package {
      name = $1
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", name)
      if (name == key) {
        value = substr($0, index($0, "=") + 1)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
        gsub(/^"|"$/, "", value)
        print value
        exit
      }
    }
  ' Cargo.toml
}

release_ref="${1:-${GITHUB_REF_NAME:-}}"
[ -n "$release_ref" ] || fail "pass a tag like v0.0.1 or set GITHUB_REF_NAME"
[[ "$release_ref" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] ||
  fail "release tag must look like vX.Y.Z"

for file in Cargo.toml Cargo.lock README.md CHANGELOG.md LICENSE; do
  [ -f "$file" ] || fail "missing $file"
done

release_version="${release_ref#v}"
package_name="$(read_package_field name)"
package_version="$(read_package_field version)"

[ "$package_name" = "fleetdm-api-client" ] ||
  fail "Cargo.toml package name must be fleetdm-api-client"
[ "$package_version" = "$release_version" ] ||
  fail "Cargo.toml version $package_version does not match $release_ref"

grep -Eq '^## \[Unreleased\]$' CHANGELOG.md ||
  fail "CHANGELOG.md must keep an [Unreleased] section"
scripts/release-notes.sh "$release_ref" >/dev/null ||
  fail "CHANGELOG.md must contain release notes for $release_version"
grep -Fq "fleetdm-api-client = \"$release_version\"" README.md ||
  fail "README.md must reference fleetdm-api-client = \"$release_version\""

for key in description license repository documentation readme rust-version; do
  [ -n "$(read_package_field "$key")" ] ||
    fail "Cargo.toml is missing [package].$key"
done

cargo metadata --format-version 1 --no-deps --locked >/dev/null

echo "Release metadata looks good for $release_ref"
