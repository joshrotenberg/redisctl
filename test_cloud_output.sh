#!/bin/bash
# Test script for cloud subscription list output

# Create a mock response file
cat > /tmp/mock_subscriptions.json << 'EOF'
[
  {
    "id": 12345,
    "name": "production-cache",
    "status": "active",
    "planId": "pro",
    "planName": "Pro",
    "paymentMethod": "credit-card",
    "created": "2024-01-15T10:30:00Z",
    "updated": "2024-11-20T15:45:00Z",
    "numberOfDatabases": 3,
    "memoryStorage": {
      "quantity": 4.0,
      "units": "GB"
    },
    "cloudProviders": [
      {
        "provider": "AWS",
        "regions": [
          {
            "region": "us-east-1",
            "memoryStorage": {
              "quantity": 4.0
            }
          }
        ]
      }
    ]
  },
  {
    "id": 67890,
    "name": "staging-db",
    "status": "pending",
    "planId": "fixed-50",
    "planName": "Standard",
    "created": "2025-09-01T08:00:00Z",
    "numberOfDatabases": 1,
    "memoryStorage": {
      "quantity": 1.0,
      "units": "GB"
    },
    "cloudProviders": [
      {
        "provider": "GCP",
        "regions": [
          {
            "region": "europe-west1",
            "memoryStorage": {
              "quantity": 1.0
            }
          }
        ]
      }
    ]
  },
  {
    "id": 11111,
    "name": "test-instance",
    "status": "error",
    "planId": "fixed-10",
    "planName": "Essentials",
    "created": "2025-09-03T14:00:00Z",
    "numberOfDatabases": 0,
    "memoryStorage": {
      "quantity": 0.25,
      "units": "GB"
    },
    "cloudProviders": [
      {
        "provider": "Azure",
        "regions": [
          {
            "region": "westus2",
            "memoryStorage": {
              "quantity": 0.25
            }
          }
        ]
      }
    ]
  }
]
EOF

echo "Mock subscription data created at /tmp/mock_subscriptions.json"

# Now you can test with:
# curl -X GET http://localhost:8080/subscriptions
# Then in another terminal:
# redisctl cloud subscription list