#!/usr/bin/env bash
#
# Debug script for release process
# Helps diagnose issues with the release pipeline

set -euo pipefail

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_command() {
    if command -v "$1" &> /dev/null; then
        log_info "✓ $1 is installed: $(command -v "$1")"
        return 0
    else
        log_error "✗ $1 is not installed"
        return 1
    fi
}

check_version() {
    local cmd="$1"
    local version_flag="${2:---version}"
    if command -v "$cmd" &> /dev/null; then
        local version=$($cmd $version_flag 2>&1 | head -n1)
        log_info "  Version: $version"
    fi
}

main() {
    log_info "Release System Debug Report"
    log_info "============================"
    echo

    # Check working directory
    log_info "Working directory: $PROJECT_ROOT"
    cd "$PROJECT_ROOT"
    echo

    # Check Git status
    log_info "Git Information:"
    if git rev-parse --git-dir > /dev/null 2>&1; then
        log_info "  Current branch: $(git branch --show-current)"
        log_info "  Latest commit: $(git rev-parse --short HEAD)"
        log_info "  Latest tag: $(git describe --tags --abbrev=0 2>/dev/null || echo 'No tags found')"

        if [[ -n $(git status --porcelain) ]]; then
            log_warn "  Working directory has uncommitted changes"
        else
            log_info "  Working directory is clean"
        fi
    else
        log_error "Not in a git repository"
    fi
    echo

    # Check required tools
    log_info "Checking required tools:"
    check_command "cargo" && check_version "cargo"
    check_command "rustc" && check_version "rustc"
    check_command "git-cliff" && check_version "git-cliff"
    check_command "cargo-dist" && check_version "cargo-dist" "dist --version"
    check_command "docker" && check_version "docker"
    check_command "gh" && check_version "gh"
    echo

    # Check Cargo.toml versions
    log_info "Checking Cargo.toml versions:"
    for toml in Cargo.toml crates/*/Cargo.toml; do
        if [[ -f "$toml" ]]; then
            version=$(grep '^version = ' "$toml" | head -1 | cut -d'"' -f2)
            log_info "  $toml: v$version"
        fi
    done
    echo

    # Check configuration files
    log_info "Checking configuration files:"
    for config in cliff.toml dist-workspace.toml .github/workflows/prepare-release.yml .github/workflows/tag-on-merge.yml .github/workflows/v-release.yml .github/workflows/docker.yml; do
        if [[ -f "$config" ]]; then
            log_info "  ✓ $config exists"
        else
            log_error "  ✗ $config missing"
        fi
    done
    echo

    # Check GitHub secrets (if gh is available)
    if command -v gh &> /dev/null && gh auth status &> /dev/null; then
        log_info "Checking GitHub configuration:"
        log_info "  Repository: $(gh repo view --json nameWithOwner -q .nameWithOwner)"

        # Check if secrets are set (we can't see values, just existence)
        log_info "  Checking for Docker Hub secrets..."
        if gh secret list | grep -q DOCKER_USERNAME; then
            log_info "    ✓ DOCKER_USERNAME is set"
        else
            log_warn "    ✗ DOCKER_USERNAME not found"
        fi

        if gh secret list | grep -q DOCKER_PASSWORD; then
            log_info "    ✓ DOCKER_PASSWORD is set"
        else
            log_warn "    ✗ DOCKER_PASSWORD not found"
        fi
    else
        log_warn "GitHub CLI not authenticated, skipping GitHub checks"
    fi
    echo

    # Test changelog generation
    log_info "Testing changelog generation:"
    if command -v git-cliff &> /dev/null; then
        log_info "  Generating test changelog..."
        if git-cliff --config cliff.toml --unreleased --strip all > /dev/null 2>&1; then
            log_info "  ✓ Changelog generation works"
        else
            log_error "  ✗ Changelog generation failed"
        fi
    fi
    echo

    # Test cargo dist
    log_info "Testing cargo-dist:"
    if command -v cargo &> /dev/null && cargo dist --help &> /dev/null 2>&1; then
        log_info "  Running cargo dist plan..."
        if cargo dist plan &> /dev/null 2>&1; then
            log_info "  ✓ cargo-dist plan succeeds"
        else
            log_error "  ✗ cargo-dist plan failed"
        fi
    fi
    echo

    # Summary
    log_info "Summary:"
    log_info "========"
    log_info "Use './scripts/debug-release.sh' to diagnose release issues"
    log_info "To trigger a release:"
    log_info "  1. Run: gh workflow run prepare-release.yml"
    log_info "  2. Select version bump type (patch/minor/major)"
    log_info "  3. Review and merge the created PR"
    log_info "  4. Release will be automatically created and published"
}

main "$@"
