#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
checker="${script_dir}/check-release-provenance.sh"
test_root="$(mktemp -d)"
repository="${test_root}/repository"

cleanup() {
  rm -rf "$test_root"
}
trap cleanup EXIT

fail() {
  echo "error: $*" >&2
  exit 1
}

git init --quiet --initial-branch=main "$repository"
git -C "$repository" config user.email release-test@example.invalid
git -C "$repository" config user.name "Release provenance test"
git -C "$repository" config commit.gpgsign false
git -C "$repository" commit --quiet --allow-empty -m "main release commit"

main_sha="$(git -C "$repository" rev-parse HEAD)"
git -C "$repository" update-ref refs/remotes/origin/main "$main_sha"

(
  cd "$repository"
  "$checker" "$main_sha" origin/main >/dev/null
)

git -C "$repository" switch --quiet -c non-main
git -C "$repository" commit --quiet --allow-empty -m "non-main commit"
non_main_sha="$(git -C "$repository" rev-parse HEAD)"
failure_output="${test_root}/non-main-error"

if (
  cd "$repository"
  "$checker" "$non_main_sha" origin/main >"$failure_output" 2>&1
); then
  fail "provenance guard accepted a non-main release commit"
fi

grep -Fq "does not match protected main head" "$failure_output" ||
  fail "provenance guard returned an unexpected non-main error"

echo "Release provenance guard accepts main and rejects a non-main commit"
