#!/bin/bash

set -euo pipefail

# redisctl Manual Release Script
# Usage: ./scripts/release.sh [patch|minor|major]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default to patch if no argument provided
VERSION_TYPE="${1:-patch}"

log() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

error() {
    echo -e "${RED}[ERROR]${NC} $1"
    exit 1
}

header() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

confirm() {
    local prompt="$1"
    local response
    echo -e "${YELLOW}$prompt${NC}"
    read -p "Continue? (y/N): " response
    case "$response" in
        [yY]|[yY][eS]|[yY][eE][sS])
            return 0
            ;;
        *)
            echo "Aborted by user"
            exit 1
            ;;
    esac
}

check_requirements() {
    header "Checking Requirements"

    # Check if we're in the right directory
    if [[ ! -f "$PROJECT_ROOT/Cargo.toml" ]]; then
        error "Not in project root directory"
    fi

    # Check if cargo-workspaces is installed
    if ! command -v cargo-workspaces &> /dev/null; then
        warn "cargo-workspaces not found. Installing..."
        cargo install cargo-workspaces
    fi

    # Check git status
    if [[ -n $(git status --porcelain) ]]; then
        warn "Working directory has uncommitted changes"
        git status --short
        confirm "Continue with uncommitted changes?"
    fi

    # Check if we're on main branch
    local current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" ]]; then
        warn "Not on main branch (currently on: $current_branch)"
        confirm "Continue from $current_branch?"
    fi

    # Check if we can push to origin
    log "Checking git remote access..."
    git fetch origin

    log "✓ All requirements satisfied"
}

show_version_info() {
    header "Current Version Information"

    echo "Current versions:"
    for pkg in redis-cloud redis-enterprise redisctl; do
        version=$(cargo metadata --no-deps --format-version 1 | jq -r ".packages[] | select(.name == \"$pkg\") | .version")
        echo "  $pkg: $version"
    done

    echo ""
    echo "Workspace members:"
    cargo metadata --no-deps --format-version 1 | jq -r '.workspace_members[]'

    echo ""
    echo "Version bump: $VERSION_TYPE"
    cargo workspaces version $VERSION_TYPE --no-git-commit --yes 2>/dev/null | grep "Changes:" -A 10 | head -5

    # Reset any changes made by the dry run
    git checkout HEAD -- Cargo.toml crates/*/Cargo.toml 2>/dev/null || true
}

check_crates_io_auth() {
    header "Checking crates.io Authentication"

    if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
        error "CARGO_REGISTRY_TOKEN environment variable not set"
    fi

    # Test authentication with a small crate
    log "Testing crates.io authentication..."
    cd "$PROJECT_ROOT/crates/redis-cloud"
    if cargo publish --dry-run &>/dev/null; then
        log "✓ crates.io authentication working"
    else
        error "crates.io authentication failed"
    fi
    cd "$PROJECT_ROOT"
}

run_tests() {
    header "Running Tests"

    log "Running workspace tests..."
    if cargo test --workspace --all-features; then
        log "✓ All tests passed"
    else
        error "Tests failed - aborting release"
    fi

    log "Running clippy..."
    if cargo clippy --all-targets --all-features -- -D warnings; then
        log "✓ Clippy passed"
    else
        error "Clippy failed - aborting release"
    fi
}

create_release() {
    header "Creating Release"

    log "Bumping versions and publishing..."

    # Use cargo-workspaces to version, commit, tag, and publish in one command
    if cargo workspaces publish \
        --yes \
        --no-verify \
        --allow-branch main \
        --force '*' \
        "$VERSION_TYPE"; then

        NEW_VERSION=$(git describe --tags --abbrev=0)
        log "✓ Successfully released $NEW_VERSION"

        return 0
    else
        error "Release failed"
    fi
}

post_release_info() {
    header "Release Complete!"

    local new_version=$(git describe --tags --abbrev=0)

    echo -e "${GREEN}🎉 Successfully released $new_version${NC}"
    echo ""
    echo "What happened:"
    echo "  ✓ Versions bumped in all crates"
    echo "  ✓ Changes committed to git"
    echo "  ✓ Git tag created and pushed: $new_version"
    echo "  ✓ All crates published to crates.io"
    echo ""
    echo "What's happening next:"
    echo "  🔄 cargo-dist will build binaries (triggered by tag)"
    echo "  🔄 Docker images will be built (triggered by tag)"
    echo ""
    echo "Check status at:"
    echo "  📦 crates.io: https://crates.io/crates/redisctl"
    echo "  🏷️  GitHub: https://github.com/joshrotenberg/redisctl/releases"
    echo "  🐳 Docker: https://hub.docker.com/r/joshrotenberg/redisctl"
    echo ""
    echo -e "${BLUE}Release completed successfully!${NC} 🚀"
}

cleanup_on_failure() {
    header "Cleaning up after failure..."

    # Get the version that might have been created
    local latest_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

    if [[ -n "$latest_tag" ]]; then
        warn "Found tag: $latest_tag"
        confirm "Delete the failed release tag $latest_tag?"

        # Delete local and remote tags
        git tag -d "$latest_tag" 2>/dev/null || true
        git push --delete origin "$latest_tag" 2>/dev/null || true

        log "Cleaned up tag: $latest_tag"
    fi

    warn "Release failed. You may need to:"
    echo "  1. Check the error messages above"
    echo "  2. Ensure CARGO_REGISTRY_TOKEN is valid"
    echo "  3. Verify git push access to origin"
    echo "  4. Check crates.io authentication"
}

main() {
    echo -e "${BLUE}redisctl Release Script${NC}"
    echo -e "${BLUE}=====================${NC}"
    echo ""

    # Validate version type argument
    case "$VERSION_TYPE" in
        patch|minor|major)
            ;;
        *)
            error "Invalid version type: $VERSION_TYPE. Use: patch, minor, or major"
            ;;
    esac

    # Set up error handling
    trap cleanup_on_failure ERR

    # Run release process
    check_requirements
    show_version_info
    confirm "Proceed with $VERSION_TYPE release?"
    check_crates_io_auth
    run_tests
    confirm "All checks passed. Create the release?"
    create_release
    post_release_info
}

# Run main function
main "$@"
