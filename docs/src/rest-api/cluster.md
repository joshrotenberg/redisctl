# Cluster REST API Reference

## Get Cluster Information
- **Method**: `GET`
- **Path**: `/v1/cluster`
- **Authentication**: Basic Auth
- **Response**: `200 OK`

```json
{
  "name": "string",
  "nodes": ["array of node objects"],
  "alert_settings": {},
  "email_alerts": true,
  "email_from": "string",
  "smtp_host": "string",
  "rack_aware": false,
  "persistent_path": "string",
  "register_dns_suffix": "string",
  "sentinel_service_port": 8001,
  "created_time": "ISO8601",
  "license": {},
  "state": "string"
}
```

## Update Cluster Configuration
- **Method**: `PUT`
- **Path**: `/v1/cluster`
- **Authentication**: Basic Auth
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "name": "string",
  "alert_settings": {
    "cluster_alert_when_down": true,
    "node_alert_when_down": true,
    "alert_threshold_ram": 80
  },
  "email_alerts": true,
  "email_from": "alerts@example.com",
  "smtp_host": "smtp.example.com"
}
```
- **Response**: `200 OK` - Returns updated cluster object

## Bootstrap Cluster
- **Method**: `POST`
- **Path**: `/v1/bootstrap`
- **Authentication**: Basic Auth (uses credentials from body)
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "action": "create_cluster",
  "cluster": {
    "name": "string"
  },
  "node": {
    "paths": {
      "persistent_path": "/var/opt/redislabs/persist",
      "ephemeral_path": "/var/opt/redislabs/tmp"
    }
  },
  "credentials": {
    "username": "admin@example.com",
    "password": "secure-password"
  },
  "license_file": "string (optional)"
}
```
- **Response**: `200 OK`

## Join Node to Cluster
- **Method**: `POST`
- **Path**: `/v1/bootstrap/join`
- **Authentication**: Basic Auth
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "action": "join_cluster",
  "cluster": {
    "nodes": ["192.168.1.10"]
  },
  "credentials": {
    "username": "admin@example.com",
    "password": "secure-password"
  }
}
```
- **Response**: `200 OK`

## Get Cluster Statistics
- **Method**: `GET`
- **Path**: `/v1/cluster/stats`
- **Authentication**: Basic Auth
- **Query Parameters**:
  - `interval`: `1sec`, `10sec`, `1min`, `5min`, `15min`, `1hour`, `12hour`, `1day`, `1week`, `1month`, `1year`
  - `stime`: Start time (Unix timestamp)
  - `etime`: End time (Unix timestamp)
- **Response**: `200 OK`

```json
{
  "intervals": [
    {
      "interval": "1min",
      "timestamps": [1234567890, 1234567950],
      "values": {
        "cpu_system": [12.5, 13.2],
        "cpu_user": [45.3, 46.1],
        "total_req": [1000, 1100],
        "total_connections": [50, 52]
      }
    }
  ]
}
```

## Get License Information
- **Method**: `GET`
- **Path**: `/v1/license`
- **Authentication**: Basic Auth
- **Response**: `200 OK`

```json
{
  "license_key": "string",
  "type": "enterprise",
  "expired": false,
  "expiration_date": "2025-12-31",
  "shards_limit": 100,
  "node_limit": 10,
  "features": ["crdt", "modules"],
  "owner": "Example Corp"
}
```

## Update License
- **Method**: `PUT`
- **Path**: `/v1/license`
- **Authentication**: Basic Auth
- **Content-Type**: `application/json`
- **Request Body**:

```json
{
  "license": "-----BEGIN LICENSE-----\n...\n-----END LICENSE-----"
}
```
- **Response**: `200 OK` - Returns updated license object

## Common Response Codes
- `200 OK` - Success
- `400 Bad Request` - Invalid parameters
- `401 Unauthorized` - Authentication failed
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server error
- `507 Insufficient Storage` - Not enough resources

## Error Response Format
```json
{
  "error_code": "string",
  "description": "string",
  "detail": "string"
}
```
