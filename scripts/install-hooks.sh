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
echo "Installing pre-commit hooks..."
pre-commit install

echo "Pre-commit hooks installed successfully!"
echo ""
echo "The following hooks will run on commit:"
echo "  - cargo fmt --all --check"
echo "  - cargo clippy --all-targets --all-features -- -D warnings"
echo "  - cargo test --all-features"
echo ""
echo "To run hooks manually: pre-commit run --all-files"