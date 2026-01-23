# Release Process

This document describes the end-to-end release process for redisctl and its associated crates.

## Overview

The release process produces artifacts for multiple destinations:

| Artifact | Destination | Workflow |
|----------|-------------|----------|
| `redis-cloud` crate | crates.io | `release-plz.yml` |
| `redis-enterprise` crate | crates.io | `release-plz.yml` |
| `redisctl-config` crate | crates.io | `release-plz.yml` |
| `redisctl` crate | crates.io | `release-plz.yml` |
| CLI binaries | GitHub Releases | `release.yml` |
| Homebrew formula | redis-developer/homebrew-tap | `release.yml` |
| Docker images | ghcr.io | `docker.yml` |
| Python wheels | PyPI | `python-publish.yml` |

## Release Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              MERGE TO MAIN                                   │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           release-plz.yml                                    │
│  Trigger: push to main                                                       │
│  Actions:                                                                    │
│    1. Analyzes commits since last release                                    │
│    2. Updates CHANGELOGs                                                     │
│    3. Bumps versions in Cargo.toml                                           │
│    4. Creates/updates Release PR                                             │
│    5. When PR merges: publishes to crates.io + creates git tags              │
└─────────────────────────────────────────────────────────────────────────────┘
                                     │
                                     │ (creates tags like redisctl-v0.7.6)
                                     ▼
         ┌───────────────────────────┼───────────────────────────┐
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐     ┌─────────────────────┐     ┌─────────────────────┐
│  release.yml    │     │    docker.yml       │     │ python-publish.yml  │
│                 │     │                     │     │                     │
│ Trigger: tag    │     │ Trigger: tag        │     │ Trigger: release    │
│ **[0-9]+.*      │     │ redisctl-v* or v*   │     │ published event     │
│                 │     │                     │     │ (only *-python tags)│
│ Actions:        │     │ Actions:            │     │                     │
│ 1. cargo-dist   │     │ 1. Build multi-arch │     │ Actions:            │
│    plan         │     │    images           │     │ 1. Build wheels for │
│ 2. Build bins   │     │ 2. Push to ghcr.io  │     │    linux/mac/win    │
│    for all      │     │    with version     │     │ 2. Build sdist      │
│    platforms    │     │    tags             │     │ 3. Publish to PyPI  │
│ 3. Create       │     │                     │     │                     │
│    GitHub       │     │                     │     │                     │
│    Release      │     │                     │     │                     │
│ 4. Update       │     │                     │     │                     │
│    Homebrew     │     │                     │     │                     │
└─────────────────┘     └─────────────────────┘     └─────────────────────┘
```

## Workflow Details

### 1. release-plz.yml

**Trigger:** Push to `main` branch

**What it does:**
1. Runs `release-plz` which analyzes conventional commits
2. Determines which crates have changes
3. Calculates version bumps based on commit types:
   - `fix:` → patch bump
   - `feat:` → minor bump  
   - `BREAKING CHANGE:` → major bump
4. Updates `CHANGELOG.md` files
5. Creates or updates a Release PR

**When the Release PR is merged:**
1. Publishes crates to crates.io (uses OIDC trusted publishing)
2. Creates git tags:
   - `redisctl-v{version}` for the CLI
   - `redis-cloud-v{version}`, etc. for libraries (if `git_tag_enable = true`)

**Configuration:** `release-plz.toml`
```toml
[workspace]
changelog_update = true
git_release_enable = false  # Let cargo-dist handle GitHub releases
git_tag_enable = true

[[package]]
name = "redisctl"
git_tag_name = "redisctl-v{{ version }}"
```

### 2. release.yml (cargo-dist)

**Trigger:** Push of tags matching `**[0-9]+.[0-9]+.[0-9]+*`

**What it does:**
1. **Plan phase:** Determines what to build based on the tag
2. **Build local artifacts:** Builds binaries for each platform:
   - `aarch64-apple-darwin` (macOS ARM)
   - `x86_64-apple-darwin` (macOS Intel)
   - `x86_64-unknown-linux-gnu` (Linux)
   - `x86_64-pc-windows-msvc` (Windows)
3. **Build global artifacts:** Creates installers, checksums
4. **Host phase:** Uploads artifacts, creates GitHub Release
5. **Update Homebrew:** Updates formula in `redis-developer/homebrew-tap`

**Configuration:** `Cargo.toml` under `[workspace.metadata.dist]`

**Important:** This workflow is auto-generated by `cargo dist generate`. The Homebrew job is manually added and must be re-added after regeneration.

### 3. docker.yml

**Trigger:** Push of tags matching `v*` or `redisctl-v*`

**What it does:**
1. Extracts version from tag
2. Builds multi-arch images (linux/amd64, linux/arm64)
3. Pushes to `ghcr.io/redis-developer/redisctl` with tags:
   - `latest`
   - `{version}` (e.g., `0.7.6`)
   - `{major}.{minor}` (e.g., `0.7`)
   - `{major}` (e.g., `0`)

### 4. python-publish.yml

**Trigger:** GitHub Release `published` event (only for `*-python` tags)

**What it does:**
1. Checks if this is a Python release (tag contains `-python` or `python-`)
2. Builds wheels for:
   - Linux x86_64 and aarch64 (using maturin)
   - macOS x86_64 and aarch64
   - Windows x86_64
3. Builds source distribution (sdist)
4. Publishes to PyPI (uses OIDC trusted publishing)

**Note:** Python releases use a separate tag format (e.g., `v0.1.0-python`) to decouple from CLI releases.

## Tag Formats

| Tag Format | Triggers | Example |
|------------|----------|---------|
| `redisctl-v{version}` | release.yml, docker.yml | `redisctl-v0.7.6` |
| `redis-cloud-v{version}` | release.yml (plan only) | `redis-cloud-v0.7.6` |
| `v{version}-python` | python-publish.yml | `v0.1.0-python` |
| `v{version}` | docker.yml | `v0.7.6` |

## Dependencies Between Workflows

```
release-plz.yml (creates tags)
       │
       ├──► release.yml (builds binaries, creates GitHub Release)
       │           │
       │           └──► python-publish.yml (on release published event)
       │
       └──► docker.yml (builds container images)
```

**Critical dependency:** `python-publish.yml` triggers on the `release: published` event, which means it runs AFTER `release.yml` creates the GitHub Release.

## Failure Modes

### Crates.io publish fails
- **Symptom:** Release PR merged but crates not published
- **Check:** Look at release-plz workflow logs
- **Recovery:** Can manually publish with `cargo publish` if needed
- **Common causes:** 
  - OIDC token issues
  - Version already exists
  - Dependency not published first

### Binary build fails
- **Symptom:** Tag exists but no GitHub Release or partial artifacts
- **Check:** `release.yml` workflow logs
- **Recovery:** 
  1. Delete the failed release (if partial)
  2. Delete and recreate the tag, OR
  3. Fix the issue and create a patch release
- **Common causes:**
  - Platform-specific build failures
  - Missing dependencies in CI

### Homebrew update fails
- **Symptom:** GitHub Release exists but formula not updated
- **Check:** `update-homebrew` job in release.yml
- **Recovery:** Manually update the formula in `redis-developer/homebrew-tap`
- **Common causes:**
  - `COMMITTER_TOKEN` secret expired/invalid
  - Download URL incorrect

### Docker build fails
- **Symptom:** Tag exists but no new images on ghcr.io
- **Check:** `docker.yml` workflow logs
- **Recovery:** Manually trigger with `workflow_dispatch`
- **Common causes:**
  - Dockerfile issues
  - GHCR authentication

### Python publish fails
- **Symptom:** CLI released but no Python package on PyPI
- **Check:** `python-publish.yml` workflow logs
- **Recovery:** Manually trigger with `workflow_dispatch`
- **Common causes:**
  - pyo3/maturin build issues on specific platforms
  - PyPI trusted publisher not configured

## Verification Checklist

After a release, verify:

- [ ] **crates.io:** All crates published with correct versions
  ```bash
  cargo search redisctl
  cargo search redis-cloud
  cargo search redis-enterprise
  ```

- [ ] **GitHub Release:** Release exists with all binary artifacts
  ```bash
  gh release view redisctl-v{version}
  ```

- [ ] **Homebrew:** Formula updated
  ```bash
  brew update && brew info redisctl
  ```

- [ ] **Docker:** Images available
  ```bash
  docker pull ghcr.io/redis-developer/redisctl:{version}
  ```

- [ ] **PyPI:** Package available (if Python release)
  ```bash
  pip index versions redisctl
  ```

## Manual Release Steps

If automation fails, here's how to manually release:

### Publish to crates.io
```bash
# In dependency order
cargo publish -p redis-cloud
cargo publish -p redis-enterprise  
cargo publish -p redisctl-config
cargo publish -p redisctl
```

### Create GitHub Release
```bash
# Build with cargo-dist
cargo dist build --artifacts=all

# Create release
gh release create redisctl-v{version} \
  --title "redisctl v{version}" \
  --notes-file CHANGELOG.md \
  target/distrib/*
```

### Update Homebrew
```bash
# In the homebrew-tap repo
# Update Formula/redisctl.rb with new version and SHA256
```

### Build and Push Docker
```bash
docker buildx build --platform linux/amd64,linux/arm64 \
  -t ghcr.io/redis-developer/redisctl:{version} \
  -t ghcr.io/redis-developer/redisctl:latest \
  --push .
```

### Publish Python
```bash
cd crates/redisctl-python
maturin build --release
maturin publish
```

## Secrets Required

| Secret | Used By | Purpose |
|--------|---------|---------|
| `RELEASE_TOKEN` | release-plz.yml | GitHub token for creating PRs/tags |
| `COMMITTER_TOKEN` | release.yml | Push to homebrew-tap repo |
| `GITHUB_TOKEN` | All workflows | Default GitHub Actions token |

**Note:** crates.io and PyPI use OIDC trusted publishing (no API tokens needed).

## Troubleshooting

### "Tag exists but no release"
The `release.yml` workflow failed after `release-plz` created the tag. Check the workflow logs and potentially re-run or create a patch release.

### "Release exists but missing artifacts"
Some platform builds failed. Check which platforms failed in the `build-local-artifacts` job matrix.

### "Homebrew formula not updated"
The `update-homebrew` job only runs for `redisctl-v*` tags. Check if the `COMMITTER_TOKEN` is valid.

### "Python wheels missing for a platform"
Check the specific platform's build job. Common issues:
- macOS x86_64: pyo3 linking issues (need Python headers)
- Linux: manylinux compatibility

## Future Improvements

- [ ] Add release verification/smoke tests
- [ ] Consider decoupling Python releases from CLI releases
- [ ] Add Slack/Discord notifications on release
- [ ] Create release dashboard/status page
