# Redis Enterprise CLI Development Makefile

.PHONY: help build test clean docker-up docker-down docker-logs docker-cli docker-test docker-examples docker-monitor docker-showcase docker-integration

# Default target
help:
	@echo "Redis Enterprise CLI Development Commands"
	@echo ""
	@echo "Build Commands:"
	@echo "  make build         - Build the CLI in release mode"
	@echo "  make build-debug   - Build the CLI in debug mode"
	@echo "  make test          - Run all tests"
	@echo "  make clippy        - Run clippy linter"
	@echo "  make fmt           - Format code"
	@echo "  make clean         - Clean build artifacts"
	@echo ""
	@echo "Docker Commands:"
	@echo "  make docker-up     - Start Redis Enterprise and initialize cluster"
	@echo "  make docker-down   - Stop and remove all containers"
	@echo "  make docker-logs   - Show container logs"
	@echo "  make docker-cli    - Start interactive CLI container"
	@echo "  make docker-test   - Run comprehensive tests against cluster"
	@echo "  make docker-showcase - Demonstrate all CLI features"
	@echo "  make docker-integration - Run full integration test suite"
	@echo "  make docker-examples - Create example databases"
	@echo "  make docker-monitor - Start monitoring service"
	@echo "  make docker-all-dbs - Create all database types"
	@echo "  make docker-perf   - Run performance tests"
	@echo "  make docker-debug  - Start debug container with verbose logging"
	@echo "  make docker-cleanup - Remove all test databases"
	@echo "  make docker-rebuild - Rebuild and restart containers"
	@echo ""
	@echo "Development Workflow:"
	@echo "  make dev           - Build, test, and start Docker environment"
	@echo "  make dev-clean     - Stop Docker and clean build artifacts"

# Build Commands
build:
	cargo build --release --bin redis-enterprise

build-debug:
	cargo build --bin redis-enterprise

test:
	cargo test --all-features

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all -- --check

fmt-fix:
	cargo fmt --all

clean:
	cargo clean
	rm -rf target/

# Docker Commands
docker-up:
	docker compose up -d
	@echo "Waiting for services to be ready..."
	@sleep 5
	docker compose ps

docker-down:
	docker compose down -v

docker-logs:
	docker compose logs -f

docker-cli:
	docker compose --profile cli up -d cli
	docker attach redis-enterprise-cli-interactive

docker-test:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile test up test-runner

docker-examples:
	docker compose --profile examples up enterprise-db-examples

docker-monitor:
	docker compose --profile monitor up monitor

docker-all-dbs:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile all-dbs up create-all-db-types

docker-perf:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile perf up perf-test

docker-debug:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile debug up -d debug
	docker exec -it redis-enterprise-debug sh

docker-cleanup:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile cleanup up cleanup

docker-showcase:
	docker compose --profile showcase up showcase

docker-integration:
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile integration up integration

docker-rebuild:
	docker compose down
	docker compose build --no-cache
	docker compose up -d

docker-ps:
	docker compose ps

# Development Workflow Commands
dev: build test docker-up
	@echo "Development environment ready!"
	@echo "Redis Enterprise: https://localhost:9443"
	@echo "Web UI: https://localhost:8443"
	@echo "Use 'make docker-cli' for interactive testing"

dev-clean: docker-down clean
	@echo "Development environment cleaned"

# Quick test against running cluster
quick-test:
	REDIS_ENTERPRISE_URL=https://localhost:9443 \
	REDIS_ENTERPRISE_USER=admin@redis.local \
	REDIS_ENTERPRISE_PASSWORD=Redis123! \
	./target/release/redis-enterprise cluster info --insecure

# Run all Docker test profiles
docker-test-all: docker-up
	@echo "Running all test profiles..."
	docker compose --profile showcase up showcase
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile test up test-runner
	docker compose -f docker-compose.yml -f docker-compose.dev.yml --profile integration up integration
	@echo "All tests completed!"

# Watch for changes and rebuild
watch:
	cargo watch -x "build --bin redis-enterprise" -x "test"

# Documentation
docs:
	cargo doc --no-deps --open

docs-all:
	cargo doc --open

# Check everything before committing
pre-commit: fmt test clippy
	@echo "All checks passed!"

# Install development dependencies
install-dev-deps:
	cargo install cargo-watch
	cargo install cargo-edit
	cargo install cargo-outdated

# Update dependencies
update-deps:
	cargo update
	cargo outdated