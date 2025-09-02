# Repository Guidelines

## Project Structure & Module Organization
- `crates/redisctl/`: CLI binary (`src/main.rs`) and command handlers.
- `crates/redis-cloud/`: Redis Cloud REST client.
- `crates/redis-enterprise/`: Redis Enterprise REST client.
- `tests/integration/`: cross-crate integration tests.
- `docs/`: mdBook sources and generated CLI reference.
- `scripts/`: helper scripts (pre-commit, docs generation).

## Build, Test, and Development Commands
- Build workspace: `cargo build --workspace` — compiles all crates.
- Run tests: `cargo test --workspace --all-features` — runs unit + integration tests.
- Lint (deny warnings): `cargo clippy --all-targets --all-features -- -D warnings`.
- Format check: `cargo fmt --all -- --check` (format with `cargo fmt --all`).
- Install CLI: `cargo install --path crates/redisctl`.
- Docs: `cd docs && mdbook build` (serve locally: `mdbook serve`).
- Local Enterprise env: `docker compose up -d` (cleanup: `docker compose down -v`).

## Coding Style & Naming Conventions
- Rust 2024 edition; MSRV: 1.89. Run `cargo fmt --all` before pushing.
- Names: types/traits = CamelCase; functions/modules/files = snake_case; features = kebab-case (`cloud`, `enterprise`, `full`).
- Errors: use `thiserror` for explicit types; add context with `anyhow::Context`.
- Tracing: binaries initialize logging with `tracing_subscriber::fmt::init()`.

## Testing Guidelines
- Unit tests live next to code; integration tests in `tests/integration`.
- Prefer async tests with `#[tokio::test]`; mock HTTP using `wiremock`.
- Add tests for new commands and API paths (success and error).
- Run the full suite locally: `cargo test --workspace --all-features`.

## Commit & Pull Request Guidelines
- Use Conventional Commits (e.g., `feat: add cluster status command`, `fix: handle 401 in cloud client`, `docs: update install steps`).
- PRs must include: clear description, linked issues, updated tests, and docs/help text for user-facing changes.
- Keep changes scoped and consistent with existing style; ensure CI is green.

## Security & Configuration Tips
- Never commit secrets. Use env vars like `REDIS_CLOUD_API_KEY` and `REDIS_ENTERPRISE_PASSWORD`.
- Install pre-commit hooks: `scripts/install-hooks.sh` (runs fmt, clippy, tests).
- Prefer configuration via env vars and `.env` in local only; do not commit `.env`.

