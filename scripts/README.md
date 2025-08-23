# Development Scripts

## install-hooks.sh

Sets up pre-commit hooks to automatically run formatting, linting, and tests before each commit.

```bash
./scripts/install-hooks.sh
```

This installs hooks that will run:
- `cargo fmt --all --check` (formatting check)
- `cargo clippy --all-targets --all-features -- -D warnings` (linting)
- `cargo test --all-features` (all tests)

Prevents commits that would fail CI.