# Releasing

This workspace supports a manual, safety-first publish flow. It does not
publish automatically on normal pushes.

## crates.io token setup

1. Create or reuse a crates.io API token with publish access for `use-measure`.
2. Add the token to the GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Do not print the token in logs or local shell history.

## GitHub Actions secret

- Secret name: `CARGO_REGISTRY_TOKEN`

## Dry-run publish

Use the `Publish` workflow with:

- `crate = all` or `crate = use-measure`
- `dry_run = true`

This is the default mode and the safest way to validate publish packaging
before a real release.

## Manual publish

Use the `Publish` workflow with:

- `crate = all` or `crate = use-measure`
- `dry_run = false`

The workflow runs formatting, linting, tests, and `cargo check` before it
attempts any publish step.

## Post-initial-release automation

After the first manual crates.io release for `use-measure` exists, the
repository can use the `release-plz` workflows for follow-up releases.

### Release PR automation

- Workflow: `Release PR Automation`
- Trigger: pushes to `main` or manual dispatch
- Purpose: opens or updates a release pull request based on
  `release-plz.toml` and the current changelog rules

### Release publish automation

- Workflow: `Release Publish Automation`
- Trigger: manual dispatch only
- Required input: `post-initial-release = true`
- Purpose: confirms `use-measure` already exists on crates.io, then runs
  `release-plz release`

Real release-plz publishes still require `CARGO_REGISTRY_TOKEN` unless the
repository later moves to trusted publishing.

## Local dry-run example

```sh
cargo publish -p use-measure --dry-run
```

## Semver notes

- Patch bumps are for compatible fixes and small additive maintenance changes.
- Minor bumps are for additive API changes during `0.x` development.
- Major bumps are for breaking changes once the crate stabilizes at `1.0.0`.
- Pre-release identifiers should remain intentional and explicit.

## Permanent version warning

Published crates.io versions are permanent. You cannot replace an already
published version with new contents, so verify crate metadata, README examples,
and changelog entries before any real publish.
