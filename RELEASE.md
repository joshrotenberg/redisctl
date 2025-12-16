# Release Process for redisctl

This document outlines the complete release process to ensure all components are published correctly.

## Overview

A complete release includes:
1. **GitHub Release** with binaries (handled by cargo-dist)
2. **Docker Hub** image publication
3. **crates.io** package publication

## Release Steps

### 1. Prepare the Release

```bash
# Ensure you're on main and up to date
git checkout main
git pull origin main

# Update version in Cargo.toml files
# Edit these files to update version (e.g., 0.6.1 -> 0.6.2):
# - Cargo.toml (workspace.package.version)
# - crates/redis-cloud/Cargo.toml
# - crates/redis-enterprise/Cargo.toml
# - crates/redisctl/Cargo.toml
# Also update the dependency versions in crates/redisctl/Cargo.toml

# Update CHANGELOG.md with release notes

# Commit the changes
git add -A
git commit -m "chore: release vX.Y.Z"

# Create a PR and merge it
```

### 2. Create and Push the Release Tag

**IMPORTANT**: Use the `v` prefix format (not `redisctl-v`) for consistency:

```bash
# After PR is merged, pull latest main
git checkout main
git pull origin main

# Create the release tag
git tag -a vX.Y.Z -m "Release vX.Y.Z"

# Push the tag - this triggers all workflows
git push origin vX.Y.Z
```

### 3. Verify All Workflows Triggered

The tag push should automatically trigger:

1. **Release workflow** (cargo-dist): https://github.com/redis-developer/redisctl/actions/workflows/release.yml
   - Creates GitHub release with binaries
   - Takes ~10-15 minutes

2. **Docker Build**: https://github.com/redis-developer/redisctl/actions/workflows/docker.yml
   - Publishes to Docker Hub
   - Takes ~5-10 minutes

3. **Publish to crates.io**: https://github.com/redis-developer/redisctl/actions/workflows/publish-crates.yml
   - Publishes all three crates
   - Takes ~2-5 minutes

### 4. Verify Release Components

After workflows complete, verify:

- [ ] GitHub Release: https://github.com/redis-developer/redisctl/releases
  - Should have binaries for all platforms
- [ ] Docker Hub: https://ghcr.io/redis-developer/redisctl/tags
  - Should have new version tag
- [ ] crates.io: https://crates.io/crates/redisctl
  - Should show new version

## Troubleshooting

### If Docker or crates.io workflows don't trigger:

The workflows now support both `v*` and `redisctl-v*` tag formats. If they still don't trigger:

1. **Manual trigger via GitHub UI**:
   - Go to Actions tab
   - Select the workflow (Docker Build or Publish to crates.io)
   - Click "Run workflow"
   - Select the tag or branch

2. **Manual trigger via CLI**:
   ```bash
   gh workflow run "Docker Build" --ref vX.Y.Z
   gh workflow run "Publish to crates.io" --ref vX.Y.Z
   ```

### If crates.io publish fails:

Publish manually in order:
```bash
cd crates/redis-cloud && cargo publish
cd ../redis-enterprise && cargo publish  
cd ../redisctl && cargo publish
```

## Tag Format Standards

Going forward, use the simpler `v` prefix format:
- ✅ `v0.6.2`
- ❌ `redisctl-v0.6.2` (avoid this)

The workflows have been updated to support both formats for backward compatibility, but `v*` is preferred.

## Automation Opportunities

Consider adding:
1. **release-plz** for automated version bumping and changelog generation
2. **Unified release workflow** that ensures all three components are published
3. **Release checklist GitHub Action** that validates all components were published