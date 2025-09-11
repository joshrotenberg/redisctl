#!/bin/bash
# Initialize Redis Enterprise cluster after starting Docker Compose

echo "Waiting for Redis Enterprise to be healthy..."
while ! docker compose ps redis-enterprise | grep -q "healthy"; do
  sleep 2
done

echo "Initializing cluster with workflow..."
REDIS_ENTERPRISE_URL="https://localhost:9443" \
REDIS_ENTERPRISE_INSECURE=true \
cargo run --release -- enterprise workflow init-cluster \
  --name docker-cluster \
  --username admin@redis.local \
  --password Redis123!

echo "Cluster initialized! Access at:"
echo "  Web UI: https://localhost:8443"
echo "  API: https://localhost:9443"
echo "  Credentials: admin@redis.local / Redis123!"
