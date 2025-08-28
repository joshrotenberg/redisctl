# Redis CLI Docker Image
# Multi-stage build for optimal size

FROM rust:1.89 AS builder

WORKDIR /build

# Copy workspace files
COPY Cargo.toml ./
COPY crates/ ./crates/

# Build release binary
RUN cargo build --release --bin redisctl

# Runtime stage - Debian for compatibility
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/redisctl /usr/local/bin/redisctl

# Create non-root user
RUN useradd -m -u 1000 redis && \
    mkdir -p /home/redis/.config/redisctl && \
    chown -R redis:redis /home/redis

USER redis
WORKDIR /home/redis

# Default environment variables
ENV REDIS_ENTERPRISE_URL=""
ENV REDIS_ENTERPRISE_USER=""
ENV REDIS_ENTERPRISE_PASSWORD=""
ENV REDIS_CLOUD_API_KEY=""
ENV REDIS_CLOUD_SECRET_KEY=""

ENTRYPOINT ["redisctl"]
CMD ["--help"]