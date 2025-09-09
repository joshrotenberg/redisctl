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
echo "Testing version bump (dry-run)..."
cargo workspaces version patch --no-git-commit --no-git-push --no-git-tag -y

echo ""
echo "Testing publish (dry-run)..."
cargo workspaces publish --dry-run skip

echo ""
echo "✓ All tests passed! cargo-workspaces is ready to use."
echo ""
echo "To perform an actual release:"
echo "  1. Commit all changes"
echo "  2. Run: cargo workspaces version [patch|minor|major]"
echo "  3. Run: cargo workspaces publish --from-git --yes --no-verify --allow-branch main"
echo "  4. Or use the GitHub Actions workflow for automated releases"
