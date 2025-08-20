# Redis Enterprise CLI Docker Image
# Multi-stage build for optimal size

FROM rust:1.82 AS builder

WORKDIR /build

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build release binary
RUN cargo build --release --bin redis-enterprise

# Runtime stage - minimal debian image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/redis-enterprise /usr/local/bin/redis-enterprise

# Create non-root user
RUN useradd -m -u 1000 redis && \
    mkdir -p /home/redis/.config/redis-enterprise && \
    chown -R redis:redis /home/redis

USER redis
WORKDIR /home/redis

# Default environment variables
ENV REDIS_ENTERPRISE_URL=""
ENV REDIS_ENTERPRISE_USER=""
ENV REDIS_ENTERPRISE_PASSWORD=""

ENTRYPOINT ["redis-enterprise"]
CMD ["--help"]