# Redis CLI Docker Image
# Builds from source for reliability and multi-arch support

# Build stage
FROM rust:1.89-bookworm AS builder

WORKDIR /usr/src/redisctl

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build the release binary
RUN cargo build --release --bin redisctl

# Runtime stage
FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/redisctl/target/release/redisctl /usr/local/bin/redisctl

# Create non-root user
RUN useradd -m redis && \
    mkdir -p /home/redis/.config/redisctl && \
    chown -R redis:redis /home/redis

USER redis
WORKDIR /home/redis

# Default environment variables
ENV REDIS_ENTERPRISE_URL=""
ENV REDIS_ENTERPRISE_USER=""
ENV REDIS_ENTERPRISE_PASSWORD=""
ENV REDIS_CLOUD_API_KEY=""
ENV REDIS_CLOUD_API_SECRET=""

ENTRYPOINT ["redisctl"]
CMD ["--help"]