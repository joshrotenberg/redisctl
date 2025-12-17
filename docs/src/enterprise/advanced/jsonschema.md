# JSON Schema

The JSON schema command provides access to the Redis Enterprise API schema definitions, useful for validation, documentation, and code generation.

## Available Commands

### Get JSON Schema

Retrieve the complete JSON schema for the Redis Enterprise API:

```bash
# Get full JSON schema
redisctl enterprise jsonschema get

# Get schema as YAML
redisctl enterprise jsonschema get -o yaml

# Extract specific schema definitions
redisctl enterprise jsonschema get -q 'definitions'

# Get schema for a specific resource
redisctl enterprise jsonschema get -q 'definitions.bdb'

# List all available definitions
redisctl enterprise jsonschema get -q 'definitions | keys(@)'
```

## Output Examples

### Schema Structure
```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Redis Enterprise API Schema",
  "version": "1.0.0",
  "definitions": {
    "bdb": {
      "type": "object",
      "properties": {
        "uid": {
          "type": "integer",
          "description": "Database unique ID"
        },
        "name": {
          "type": "string",
          "description": "Database name"
        },
        "memory_size": {
          "type": "integer",
          "description": "Memory limit in bytes"
        },
        "shards_count": {
          "type": "integer",
          "minimum": 1,
          "description": "Number of database shards"
        }
      },
      "required": ["name", "memory_size"]
    },
    "node": {
      "type": "object",
      "properties": {
        "uid": {
          "type": "integer",
          "description": "Node unique ID"
        },
        "addr": {
          "type": "string",
          "format": "ipv4",
          "description": "Node IP address"
        }
      }
    }
  },
  "paths": {
    "/v1/bdbs": {
      "post": {
        "requestBody": {
          "$ref": "#/definitions/bdb"
        }
      }
    }
  }
}
```

## Common Use Cases

### API Validation

Validate request payloads against the schema:

```bash
# Get schema for database creation
redisctl enterprise jsonschema get -q 'definitions.bdb'

# Extract required fields
redisctl enterprise jsonschema get -q 'definitions.bdb.required'

# Get property types
redisctl enterprise jsonschema get -q 'definitions.bdb.properties | to_entries[] | {property: .key, type: .value.type}'
```

### Code Generation

Generate TypeScript or other language definitions:

```bash
# Export schema for code generation
redisctl enterprise jsonschema get -o json > redis-enterprise-schema.json

# Extract definitions for specific resources
redisctl enterprise jsonschema get -q 'definitions.{database: bdb, cluster: cluster, node: node}' > resources.json

# Generate TypeScript interfaces (using external tool)
redisctl enterprise jsonschema get | npx json-schema-to-typescript > redis-enterprise.d.ts
```

### Documentation

Extract schema information for documentation:

```bash
# Get all resource definitions
redisctl enterprise jsonschema get -q 'definitions | keys(@)' -o json

# Get descriptions for properties
redisctl enterprise jsonschema get -q 'definitions.bdb.properties | to_entries[] | {property: .key, description: .value.description}'

# List all API paths
redisctl enterprise jsonschema get -q 'paths | keys(@)'

# Get operations for a path
redisctl enterprise jsonschema get -q 'paths."/v1/bdbs" | keys(@)'
```

### Schema Discovery

Explore available schemas and their structures:

```bash
# List all top-level schema properties
redisctl enterprise jsonschema get -q 'keys(@)'

# Find schemas with specific properties
redisctl enterprise jsonschema get -q 'definitions | to_entries[] | select(.value.properties.memory_size) | .key'

# Get enum values for properties
redisctl enterprise jsonschema get -q 'definitions.*.properties.* | select(.enum) | {property: @, values: .enum}'

# Find required properties across all schemas
redisctl enterprise jsonschema get -q 'definitions | to_entries[] | {schema: .key, required: .value.required}'
```

## Integration Examples

### Validation Script

Create a validation script using the schema:

```bash
#!/bin/bash
# validate-payload.sh

SCHEMA=$(redisctl enterprise jsonschema get -q 'definitions.bdb')
PAYLOAD=$1

echo "$PAYLOAD" | jq --argjson schema "$SCHEMA" '
  # Simple validation example
  if .name == null then
    error("name is required")
  elif .memory_size == null then
    error("memory_size is required")
  else
    .
  end
'
```

### OpenAPI Generation

Convert to OpenAPI specification:

```bash
# Extract and format for OpenAPI
redisctl enterprise jsonschema get -o json | jq '{
  openapi: "3.0.0",
  info: {
    title: "Redis Enterprise API",
    version: .version
  },
  components: {
    schemas: .definitions
  },
  paths: .paths
}' > openapi.json
```

### Schema Comparison

Compare schemas across versions:

```bash
# Save current schema
redisctl enterprise jsonschema get -o json > schema-current.json

# Later, compare with new version
redisctl enterprise jsonschema get -o json > schema-new.json
diff <(jq -S . schema-current.json) <(jq -S . schema-new.json)

# Find new properties
jq -r '.definitions | keys(@)' schema-new.json | \
  comm -13 <(jq -r '.definitions | keys(@)' schema-current.json | sort) -
```

## Best Practices

1. **Cache Schema**: The schema doesn't change frequently, so cache it locally
2. **Version Control**: Store schema snapshots in version control for tracking changes
3. **Validation**: Use the schema to validate payloads before API calls
4. **Code Generation**: Generate client code from schema for type safety
5. **Documentation**: Keep schema-based documentation up to date

## Troubleshooting

### Schema Retrieval Issues

If schema retrieval fails:

```bash
# Check API connectivity
redisctl enterprise cluster get -q 'name'

# Try raw API access
redisctl api enterprise get /v1/jsonschema

# Check with curl
curl -k -u "$REDIS_ENTERPRISE_USER:$REDIS_ENTERPRISE_PASSWORD" \
  https://$REDIS_ENTERPRISE_URL/v1/jsonschema
```

### Schema Validation

Validate that the schema is well-formed:

```bash
# Check if valid JSON (using jq for JSON validation)
redisctl enterprise jsonschema get | jq empty && echo "Valid JSON"

# Validate schema structure (jq for boolean expressions)
redisctl enterprise jsonschema get | jq 'has("definitions") and has("$schema")'

# Check for required sections
redisctl enterprise jsonschema get -q '[has("definitions"), has("properties"), has("paths")] | all'
```

## Related Commands

- `redisctl api enterprise` - Direct API access for testing
- `redisctl enterprise database create` - Use schema for creating resources
- `redisctl enterprise cluster` - Cluster configuration that follows schema