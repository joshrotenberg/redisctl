#!/bin/bash

set -euo pipefail

echo "Testing cargo-workspaces release process..."

# Check if cargo-workspaces is installed
if ! command -v cargo-workspaces &> /dev/null; then
    echo "Installing cargo-workspaces..."
    cargo install cargo-workspaces
else
    echo "✓ cargo-workspaces already installed"
fi

echo ""
echo "Current workspace structure:"
cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]'

echo ""
echo "Current versions:"
for pkg in redis-cloud redis-enterprise redisctl; do
    version=$(cargo metadata --no-deps --format-version 1 | jq -r ".packages[] | select(.name == \"$pkg\") | .version")
    echo "  $pkg: $version"
done

echo ""
echo "Testing release process (dry-run)..."
cargo workspaces publish --dry-run patch

echo ""
echo "✓ All tests passed! cargo-workspaces is ready to use."
echo ""
echo "To perform an actual release:"
echo "  1. Commit all changes"
echo "  2. Run: cargo workspaces publish [patch|minor|major] --yes --no-verify --allow-branch main --force '*'"
echo "  3. Or use the GitHub Actions workflow for automated releases"
echo ""
echo "The single publish command will:"
echo "  - Bump versions according to the specified type"
echo "  - Update cross-dependencies between crates"
echo "  - Commit the changes to git"
echo "  - Create and push a git tag"
echo "  - Publish all crates to crates.io in dependency order"
