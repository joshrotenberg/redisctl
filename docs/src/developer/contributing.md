# Contributing

Guidelines for contributing to redisctl.

## Development Setup

```bash
# Clone repository
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl

# Build
cargo build

# Run tests
cargo test --workspace --all-features

# Run lints
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Branch Naming

- `feat/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` - Test improvements

## Commit Messages

Use conventional commits:

```
feat: add database streaming support
fix: handle empty response in cluster stats
docs: update installation instructions
refactor: extract async operation handling
test: add wiremock tests for VPC peering
```

## Pull Requests

1. Create feature branch from `main`
2. Make changes with tests
3. Run `cargo fmt` and `cargo clippy`
4. Push and create PR
5. Wait for CI to pass
6. Request review

## Testing

### Unit Tests
```bash
cargo test --lib --all-features
```

### Integration Tests
```bash
cargo test --test '*' --all-features
```

### With Docker Environment
```bash
docker compose up -d
cargo test --package redis-enterprise
```

## Code Style

- Use `anyhow` for CLI errors, `thiserror` for library errors
- All public APIs must have doc comments
- Follow Rust 2024 edition idioms
- No emojis in code, commits, or docs

## Adding Commands

1. Add CLI enum variant in `crates/redisctl/src/cli/`
2. Implement handler in `crates/redisctl/src/commands/`
3. Add client method in library crate if needed
4. Add tests
5. Update documentation

## Questions?

Open an issue at [github.com/joshrotenberg/redisctl/issues](https://github.com/joshrotenberg/redisctl/issues)
