#!/bin/bash

# Install pre-commit hooks for Rust development

set -e

echo "Setting up pre-commit hooks..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null; then
    echo "pre-commit not found. Installing via pip..."
    pip install pre-commit
fi

# Install the hooks
echo "Installing pre-commit and pre-push hooks..."
pre-commit install --hook-type pre-commit --hook-type pre-push

echo "Pre-commit and pre-push hooks installed successfully!"
echo ""
echo "The following hooks will run on commit and push:"
echo "  - cargo fmt --all --check"
echo "  - cargo clippy --all-targets --all-features -- -D warnings"
echo "  - cargo test --workspace --all-features"
echo ""
echo "To run hooks manually: pre-commit run --all-files"
