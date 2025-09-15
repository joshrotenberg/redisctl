# Multi-stage build for minimal final image
FROM rust:1.89-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

# Create app directory
WORKDIR /app

# Copy all source files
COPY . .

# Build static binary with musl
RUN cargo build --release --bin redisctl

# Final minimal image
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy binary from builder
COPY --from=builder /app/target/release/redisctl /usr/local/bin/redisctl

# Create non-root user
RUN adduser -D -u 1001 redisctl
USER redisctl

# Set entrypoint
ENTRYPOINT ["redisctl"]
