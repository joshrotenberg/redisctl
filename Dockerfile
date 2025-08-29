# Redis CLI Docker Image
# Uses pre-built binaries from GitHub releases for faster multi-arch builds

FROM ubuntu:24.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Set the version to install
ARG VERSION=0.1.1
ARG TARGETPLATFORM

# Download the appropriate binary based on platform
RUN case "$TARGETPLATFORM" in \
        "linux/amd64") \
            ARCH="x86_64-unknown-linux-gnu" \
            ;; \
        "linux/arm64") \
            ARCH="aarch64-unknown-linux-gnu" \
            ;; \
        *) \
            echo "Unsupported platform: $TARGETPLATFORM" && exit 1 \
            ;; \
    esac && \
    curl -L "https://github.com/joshrotenberg/redisctl/releases/download/redisctl-v${VERSION}/redisctl-${ARCH}.tar.xz" | \
    tar -xJ --strip-components=1 -C /tmp && \
    mv /tmp/redisctl /usr/local/bin/redisctl && \
    chmod +x /usr/local/bin/redisctl && \
    rm -rf /tmp/*

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