# Repository Guidelines

Use this guide to contribute effectively to this repository. Keep changes focused, tested, and consistent with the existing style.

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
- Docs: `cd docs && mdbook build` (serve locally: `mdbook serve`)
- Local Enterprise env: `docker compose up -d` (cleanup: `docker compose down -v`)

## Coding Style & Naming Conventions
- Rust 2024 edition; MSRV: 1.89. Run `cargo fmt --all` before pushing.
- Names: types/traits = CamelCase; functions/modules/files = snake_case; feature flags = kebab-case (`cloud`, `enterprise`, `full`).
- Errors: prefer explicit types via `thiserror`; add context with `anyhow::Context`.
- Tracing: binaries should initialize logging with `tracing_subscriber::fmt::init()`.

## Testing Guidelines
- Unit tests live next to code; integration tests in `tests/integration`.
- Prefer async tests with `#[tokio::test]`; mock HTTP using `wiremock`.
- Add tests for new commands and API paths; cover success and error cases.
- Before a PR, run: `cargo test --workspace --all-features` and clippy/fmt commands above.

## Commit & Pull Request Guidelines
- Use Conventional Commits (e.g., `feat: add cluster status command`, `fix: handle 401 in cloud client`, `docs: update install steps`).
- PRs must include: clear description, linked issues, updated tests, and docs/help text for user-facing changes. Keep changes scoped; ensure CI is green.

## Security & Configuration Tips
- Never commit secrets. Use env vars like `REDIS_CLOUD_API_KEY` and `REDIS_ENTERPRISE_PASSWORD`.
- Install pre-commit hooks: `scripts/install-hooks.sh` (runs fmt, clippy, tests).
