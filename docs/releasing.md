# Releasing

Releases are driven by version tags and published from GitHub Actions with
crates.io trusted publishing. The workflow does not store a long-lived
crates.io token.

## One-time bootstrap

Trusted publishing can only be configured after the crate exists on crates.io.
Version `0.0.1` therefore requires one manual bootstrap publication:

1. Confirm that `main` is clean and CI is green.
2. Run `scripts/check-release.sh v0.0.1` and
   `cargo publish --dry-run --locked`.
3. Publish `0.0.1` manually with a short-lived, crate-scoped crates.io token:
   `cargo publish --locked`.
4. Open the `fleetdm-api-client` crate settings on crates.io and add this
   trusted publisher:

   - GitHub owner: `terjekv`
   - repository: `fleetdm-api-client`
   - workflow: `release.yml`
   - environment: `release`

5. Enable trusted-publishing-only mode after verifying the first OIDC release.
   Remove any GitHub `CARGO_REGISTRY_TOKEN` secret; this workflow does not use
   one.

The repository's GitHub `release` environment must allow tags matching `v*`.
Required reviewers can be added if releases should require a manual approval;
leave them unset for fully automatic tag publishing.

After the manual `0.0.1` publication, pushing `v0.0.1` is safe: the workflow
recognizes the existing crate version, skips crates.io publication, and creates
the corresponding GitHub release. Later version tags publish through OIDC.

## Prepare a release

Before tagging:

1. Set the package version in `Cargo.toml` and update `Cargo.lock`.
2. Move user-visible changes from `Unreleased` into a dated
   `## [X.Y.Z] - YYYY-MM-DD` section in `CHANGELOG.md`.
3. Update the dependency version shown in `README.md`.
4. Run the release checks:

   ```bash
   scripts/check-release.sh vX.Y.Z
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features --locked -- -D warnings
   cargo test --all-targets --locked
   cargo test --doc --locked
   scripts/update-spec-api.py --check
   cargo audit
   cargo deny check advisories bans licenses sources
   cargo publish --dry-run --locked
   ```

5. Merge the release preparation to `main` and wait for CI to pass.

The release workflow requires the tagged commit to equal the current
`origin/main` head. Do not advance `main` between creating the tag and the
workflow's provenance check.

## Tag and publish

Create a signed tag from the verified `main` head and push only that tag:

```bash
git switch main
git pull --ff-only
git tag -s vX.Y.Z -m "fleetdm-api-client X.Y.Z"
git push origin vX.Y.Z
```

`.github/workflows/release.yml` then:

1. verifies strict `vX.Y.Z` syntax and exact `main` provenance;
2. checks that the tag, Cargo metadata, README, and changelog agree;
3. invokes the complete reusable CI workflow;
4. requests a short-lived crates.io token over GitHub OIDC;
5. publishes the crate with `Cargo.lock` enforced; and
6. creates or updates a GitHub release using the matching changelog section.

Only the publication job receives `id-token: write`. Only the final GitHub
release job receives `contents: write`. All external actions are pinned to full
commit hashes and tracked by Dependabot.

## Failure and recovery

The workflow is idempotent when a crate version is already present on
crates.io, which makes job reruns safe. A crates.io publication cannot be
overwritten or deleted. If a published version is defective, yank it through
crates.io and prepare a new patch release; do not move or reuse the old tag.

Do not add token-based publication as an OIDC fallback. Diagnose the trusted
publisher, environment, tag, or metadata mismatch instead.
