# Repository Guidelines

## Project Structure & Module Organization
- `crates/redisctl/`: CLI binary (`src/main.rs`) and command handlers.
- `crates/redis-cloud/`: Rust client for Redis Cloud REST API.
- `crates/redis-enterprise/`: Rust client for Redis Enterprise REST API.
- `tests/integration/`: cross-crate integration tests.
- `docs/`: mdBook sources and generated CLI reference.
- `scripts/`: helper scripts (pre-commit, docs generation).

## Build, Test, and Development Commands
- Build workspace: `cargo build --workspace`
- Run tests: `cargo test --workspace --all-features`
- Lint (deny warnings): `cargo clippy --all-targets --all-features -- -D warnings`
- Format check: `cargo fmt --all -- --check`
- Install CLI from source: `cargo install --path crates/redisctl`
- Docs (mdBook): `cd docs && mdbook build` (serve locally: `mdbook serve`)
- Local Enterprise env: `docker compose up -d` (cleanup: `docker compose down -v`)

## Coding Style & Naming Conventions
- Rust 2024 edition, MSRV: 1.89.
- Use `rustfmt` defaults; run `cargo fmt --all` before pushing.
- Naming: types/traits = `CamelCase`; functions/modules/files = `snake_case`; feature flags = `kebab-case` (`cloud`, `enterprise`, `full`).
- Prefer explicit errors via `thiserror` and `anyhow::Context`.
- Tracing: initialize via `tracing_subscriber::fmt::init()` in binaries.

## Testing Guidelines
- Unit tests live next to code; integration tests under `tests/integration`.
- Mock HTTP with `wiremock`; prefer async tests using `tokio::test`.
- Add tests for new commands and API paths; cover both success and error cases.
- Run full suite and clippy locally before opening a PR.

## Commit & Pull Request Guidelines
- Conventional commits (used by tooling and changelogs):
  - Examples: `feat: add cluster status command`, `fix: handle 401 in cloud client`, `docs: update install steps`.
- PRs should include: clear description, linked issues, test updates, and docs/help text when user-facing.
- Keep changes focused; avoid unrelated refactors. Ensure CI green.

## Security & Configuration Tips
- Never commit secrets. Use env vars (e.g., `REDIS_CLOUD_API_KEY`, `REDIS_ENTERPRISE_PASSWORD`).
- Use `scripts/install-hooks.sh` to install pre-commit hooks (fmt, clippy, tests).
