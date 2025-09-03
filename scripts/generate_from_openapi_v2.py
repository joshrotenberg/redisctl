#!/usr/bin/env python3
"""
Generate Rust models and handlers from Redis Cloud OpenAPI spec - V2.

Usage: python generate_from_openapi_v2.py <openapi.json> <output_dir>
"""

import json
import sys
import os
from pathlib import Path
from typing import Dict, List, Any, Optional, Set, Tuple
import re

def to_snake_case(name: str) -> str:
    """Convert camelCase/PascalCase to snake_case."""
    # Handle acronyms
    name = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    # Insert underscore before capitals
    name = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', name)
    return name.lower()

def to_pascal_case(name: str) -> str:
    """Convert snake_case to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('_'))

def clean_method_name(operation_id: str, method: str) -> str:
    """Clean up method names from operationId."""
    # Remove HTTP method prefix
    name = operation_id
    for prefix in ['get', 'post', 'put', 'delete', 'patch']:
        if name.lower().startswith(prefix):
            name = name[len(prefix):]
    
    # Convert to snake_case
    name = to_snake_case(name)
    
    # Clean up common patterns
    replacements = {
        'get_all_': 'list_',
        'get_': '',
        '_by_id': '',
        'create_': 'create_',
        'update_': 'update_',
        'delete_': 'delete_',
    }
    
    for old, new in replacements.items():
        if name.startswith(old):
            name = new + name[len(old):]
    
    # Special cases
    if method == 'get' and not name:
        name = 'get'
    elif method == 'get' and not name.startswith(('list', 'get')):
        name = 'get_' + name
    elif method == 'post' and not name.startswith('create'):
        name = 'create_' + name if 'create' in operation_id.lower() else name
    elif method == 'delete' and not name.startswith('delete'):
        name = 'delete_' + name if name else 'delete'
    
    return name

def rust_type_from_openapi(schema: Dict[str, Any], required: bool = True, schemas: Dict = None) -> str:
    """Convert OpenAPI type to Rust type."""
    if not schema:
        return "Value"
    
    # Handle refs
    if '$ref' in schema:
        ref_name = schema['$ref'].split('/')[-1]
        # Capitalize if it starts with lowercase
        if ref_name and ref_name[0].islower():
            ref_name = ref_name.capitalize()
        rust_type = ref_name
    else:
        type_str = schema.get('type', 'object')
        format_str = schema.get('format', '')
        
        # Handle enums
        if 'enum' in schema:
            # For simple string enums, just use String
            return 'String' if required else 'Option<String>'
        
        # Basic type mapping
        type_map = {
            ('string', 'uuid'): 'String',
            ('string', 'date-time'): 'String',
            ('string', 'date'): 'String',
            ('string', 'binary'): 'Vec<u8>',
            ('string', ''): 'String',
            ('integer', 'int32'): 'i32',
            ('integer', 'int64'): 'i64',
            ('integer', ''): 'i32',
            ('number', 'float'): 'f32',
            ('number', 'double'): 'f64',
            ('number', ''): 'f64',
            ('boolean', ''): 'bool',
            ('object', ''): 'HashMap<String, Value>',
        }
        
        rust_type = type_map.get((type_str, format_str), 'Value')
        
        # Handle arrays
        if type_str == 'array' and 'items' in schema:
            item_type = rust_type_from_openapi(schema['items'], True, schemas)
            rust_type = f'Vec<{item_type}>'
    
    # Wrap in Option if not required
    if not required:
        rust_type = f'Option<{rust_type}>'
    
    return rust_type

def collect_nested_schemas(schema_name: str, schemas: Dict[str, Any], collected: Set[str]) -> None:
    """Recursively collect nested schema dependencies."""
    if schema_name in collected or schema_name not in schemas:
        return
    
    collected.add(schema_name)
    schema = schemas[schema_name]
    
    # Check properties for refs
    for prop in schema.get('properties', {}).values():
        if '$ref' in prop:
            nested_name = prop['$ref'].split('/')[-1]
            collect_nested_schemas(nested_name, schemas, collected)
        elif prop.get('type') == 'array' and 'items' in prop:
            if '$ref' in prop['items']:
                nested_name = prop['items']['$ref'].split('/')[-1]
                collect_nested_schemas(nested_name, schemas, collected)
    
    # Check allOf, oneOf, anyOf
    for key in ['allOf', 'oneOf', 'anyOf']:
        if key in schema:
            for item in schema[key]:
                if '$ref' in item:
                    nested_name = item['$ref'].split('/')[-1]
                    collect_nested_schemas(nested_name, schemas, collected)

def generate_struct(name: str, schema: Dict[str, Any], schemas: Dict[str, Any]) -> str:
    """Generate a Rust struct from OpenAPI schema."""
    lines = []
    
    # Fix lowercase struct names
    if name and name[0].islower():
        name = name.capitalize()
    
    # Add doc comment
    description = schema.get('description', name)
    lines.append(f"/// {description}")
    lines.append("#[derive(Debug, Clone, Serialize, Deserialize)]")
    
    # Determine if we need rename_all
    properties = schema.get('properties', {})
    has_camel = any(k[0].islower() and any(c.isupper() for c in k) for k in properties.keys())
    if has_camel:
        lines.append('#[serde(rename_all = "camelCase")]')
    
    lines.append(f"pub struct {name} {{")
    
    required_fields = set(schema.get('required', []))
    
    for prop_name, prop_schema in properties.items():
        field_name = to_snake_case(prop_name)
        
        # Skip 'links' field as we'll handle it generically
        if prop_name == 'links':
            continue
        
        # Handle reserved keywords
        if field_name in ['type', 'ref', 'use', 'mod', 'match', 'move']:
            field_name = f'r#{field_name}'
        
        # Get type
        is_required = prop_name in required_fields
        rust_type = rust_type_from_openapi(prop_schema, is_required, schemas)
        
        # Add field doc if description exists
        if 'description' in prop_schema:
            lines.append(f"    /// {prop_schema['description']}")
        
        # Add serde attributes
        needs_rename = field_name != prop_name and not has_camel
        
        if not is_required or rust_type.startswith('Option<'):
            lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
            if needs_rename:
                lines.append(f'    #[serde(rename = "{prop_name}")]')
        elif needs_rename:
            lines.append(f'    #[serde(rename = "{prop_name}")]')
        
        lines.append(f"    pub {field_name}: {rust_type},")
        lines.append("")
    
    # Always add links field if present in original
    if 'links' in properties:
        lines.append("    /// HATEOAS links")
        lines.append('    #[serde(skip_serializing_if = "Option::is_none")]')
        lines.append("    pub links: Option<Vec<HashMap<String, Value>>>,")
        lines.append("")
    
    # Add catch-all for unknown fields
    lines.append("    /// Additional fields from the API")
    lines.append("    #[serde(flatten)]")
    lines.append("    pub extra: Value,")
    
    lines.append("}")
    
    return '\n'.join(lines)

def generate_handler_method(path: str, method: str, operation: Dict, schemas: Dict) -> Optional[str]:
    """Generate a handler method from an OpenAPI operation."""
    lines = []
    
    # Extract operation details
    operation_id = operation.get('operationId', '')
    summary = operation.get('summary', '')
    description = operation.get('description', '')
    parameters = operation.get('parameters', [])
    request_body = operation.get('requestBody', {})
    responses = operation.get('responses', {})
    
    # Generate clean method name
    method_name = clean_method_name(operation_id, method) if operation_id else f"{method}_{path.replace('/', '_')}"
    
    # Add documentation
    if summary:
        lines.append(f"    /// {summary}")
    if description and description != summary:
        for line in description.split('\n'):
            lines.append(f"    /// {line}")
    lines.append(f"    ///")
    lines.append(f"    /// {method.upper()} {path}")
    
    # Start method signature
    lines.append(f"    pub async fn {method_name}(")
    lines.append("        &self,")
    
    # Add path parameters
    path_params = [p for p in parameters if p.get('in') == 'path']
    for param in path_params:
        param_name = to_snake_case(param['name'])
        param_type = rust_type_from_openapi(param.get('schema', {}), True, schemas)
        lines.append(f"        {param_name}: {param_type},")
    
    # Add query parameters
    query_params = [p for p in parameters if p.get('in') == 'query']
    for param in query_params:
        param_name = to_snake_case(param['name'])
        param_type = rust_type_from_openapi(param.get('schema', {}), param.get('required', False), schemas)
        lines.append(f"        {param_name}: {param_type},")
    
    # Add request body
    if request_body:
        content = request_body.get('content', {})
        if 'application/json' in content:
            schema = content['application/json'].get('schema', {})
            if '$ref' in schema:
                type_name = schema['$ref'].split('/')[-1]
                lines.append(f"        request: &{type_name},")
            else:
                lines.append(f"        request: &Value,")
    
    # Determine return type
    return_type = "()"
    success_response = responses.get('200') or responses.get('201') or responses.get('202')
    if success_response:
        content = success_response.get('content', {})
        if 'application/json' in content:
            schema = content['application/json'].get('schema', {})
            if '$ref' in schema:
                return_type = schema['$ref'].split('/')[-1]
            else:
                return_type = rust_type_from_openapi(schema, True, schemas)
        else:
            # No content in response (empty body)
            return_type = "()"
    
    lines.append(f"    ) -> Result<{return_type}> {{")
    
    # Generate method body
    if query_params:
        lines.append("        let mut query = Vec::new();")
        for param in query_params:
            param_name = to_snake_case(param['name'])
            original_name = param['name']
            if param.get('required', False):
                lines.append(f'        query.push(format!("{original_name}={{}}", {param_name}));')
            else:
                lines.append(f'        if let Some(v) = {param_name} {{')
                lines.append(f'            query.push(format!("{original_name}={{}}", v));')
                lines.append('        }')
        lines.append('        let query_string = if query.is_empty() {')
        lines.append('            String::new()')
        lines.append('        } else {')
        lines.append('            format!("?{}", query.join("&"))')
        lines.append('        };')
    
    # Build the path
    path_formatted = path
    for param in path_params:
        param_name = param['name']
        path_formatted = path_formatted.replace(f'{{{param_name}}}', '{}')
    
    if path_params:
        format_args = ', '.join(to_snake_case(p['name']) for p in path_params)
        if query_params:
            url = f'&format!("{path_formatted}{{}}", {format_args}, query_string)'
        else:
            url = f'&format!("{path_formatted}", {format_args})'
    elif query_params:
        url = f'&format!("{path}{{}}", query_string)'
    else:
        url = f'"{path}"'
    
    # Call appropriate client method
    if method == 'get':
        lines.append(f"        self.client.get({url}).await")
    elif method == 'post':
        if request_body:
            lines.append(f"        self.client.post({url}, request).await")
        else:
            lines.append(f"        self.client.post({url}, &serde_json::json!({{}})).await")
    elif method == 'put':
        if request_body:
            lines.append(f"        self.client.put({url}, request).await")
        else:
            lines.append(f"        self.client.put({url}, &serde_json::json!({{}})).await")
    elif method == 'patch':
        if request_body:
            lines.append(f"        self.client.patch({url}, request).await")
        else:
            lines.append(f"        self.client.patch({url}, &serde_json::json!({{}})).await")
    elif method == 'delete':
        if request_body:
            # DELETE with body - currently not supported by client, so we ignore the body
            # TODO: Add delete_with_body to client
            lines.append(f"        // TODO: DELETE with body not yet supported by client")
            lines.append(f"        let _ = request; // Suppress unused variable warning")
        if return_type == "()":
            lines.append(f"        self.client.delete({url}).await")
        else:
            # For DELETE operations that return data, we need to deserialize the response
            lines.append(f"        let response = self.client.delete_raw({url}).await?;")
            lines.append(f"        serde_json::from_value(response).map_err(Into::into)")
    
    lines.append("    }")
    
    return '\n'.join(lines)

def generate_module(tag: str, paths: Dict[str, Dict], schemas: Dict[str, Any], spec: Dict) -> str:
    """Generate a complete module for a tag."""
    lines = []
    
    handler_name = extract_handler_name(tag)
    
    # File header
    lines.append(f"//! {tag} operations and models")
    lines.append("")
    lines.append("use crate::{CloudClient, Result};")
    lines.append("use serde::{Deserialize, Serialize};")
    lines.append("use serde_json::Value;")
    lines.append("use std::collections::HashMap;")
    lines.append("")
    
    # Collect all schemas used by this tag's operations
    used_schemas = set()
    tag_paths = {}
    
    for path, methods in paths.items():
        for method, operation in methods.items():
            if method in ['get', 'post', 'put', 'delete', 'patch']:
                tags = operation.get('tags', [])
                if tag in tags:
                    tag_paths[path] = tag_paths.get(path, {})
                    tag_paths[path][method] = operation
                    
                    # Collect schemas from parameters
                    for param in operation.get('parameters', []):
                        if 'schema' in param and '$ref' in param['schema']:
                            schema_name = param['schema']['$ref'].split('/')[-1]
                            collect_nested_schemas(schema_name, schemas, used_schemas)
                    
                    # Collect from request body
                    request_body = operation.get('requestBody', {})
                    if request_body:
                        content = request_body.get('content', {})
                        if 'application/json' in content:
                            schema = content['application/json'].get('schema', {})
                            if '$ref' in schema:
                                schema_name = schema['$ref'].split('/')[-1]
                                collect_nested_schemas(schema_name, schemas, used_schemas)
                    
                    # Collect from responses
                    for response in operation.get('responses', {}).values():
                        content = response.get('content', {})
                        if 'application/json' in content:
                            schema = content['application/json'].get('schema', {})
                            if '$ref' in schema:
                                schema_name = schema['$ref'].split('/')[-1]
                                collect_nested_schemas(schema_name, schemas, used_schemas)
    
    # Generate models section
    if used_schemas:
        lines.append("// " + "=" * 76)
        lines.append("// Models")
        lines.append("// " + "=" * 76)
        lines.append("")
        
        # Sort schemas to ensure dependencies come first
        sorted_schemas = []
        remaining = used_schemas.copy()
        while remaining:
            for schema_name in list(remaining):
                schema = schemas.get(schema_name, {})
                deps = set()
                for prop in schema.get('properties', {}).values():
                    if '$ref' in prop:
                        dep_name = prop['$ref'].split('/')[-1]
                        if dep_name in remaining and dep_name != schema_name:
                            deps.add(dep_name)
                
                if not deps:
                    sorted_schemas.append(schema_name)
                    remaining.remove(schema_name)
        
        for schema_name in sorted_schemas:
            if schema_name in schemas:
                schema = schemas[schema_name]
                if 'enum' in schema:
                    # Skip enums for now, just use String
                    continue
                else:
                    lines.append(generate_struct(schema_name, schema, schemas))
                lines.append("")
                lines.append("")
    
    # Generate handler
    lines.append("// " + "=" * 76)
    lines.append("// Handler")
    lines.append("// " + "=" * 76)
    lines.append("")
    
    handler_class = to_pascal_case(handler_name) + "Handler"
    lines.append(f"/// {tag} operations handler")
    lines.append(f"pub struct {handler_class} {{")
    lines.append("    client: CloudClient,")
    lines.append("}")
    lines.append("")
    
    lines.append(f"impl {handler_class} {{")
    lines.append("    /// Create a new handler")
    lines.append("    pub fn new(client: CloudClient) -> Self {")
    lines.append("        Self { client }")
    lines.append("    }")
    lines.append("")
    
    # Generate methods
    for path, methods in sorted(tag_paths.items()):
        for method, operation in sorted(methods.items()):
            method_code = generate_handler_method(path, method, operation, schemas)
            if method_code:
                lines.append(method_code)
                lines.append("")
    
    lines.append("}")
    
    return '\n'.join(lines)

def extract_handler_name(tag: str) -> str:
    """Extract handler name from OpenAPI tag."""
    # Map tags to handler names
    tag_map = {
        'Account': 'account',
        'Users': 'users',
        'Subscriptions - Pro': 'subscriptions',
        'Databases - Pro': 'databases', 
        'Subscriptions - Pro - Connectivity': 'connectivity',
        'Subscriptions - Essentials': 'fixed_subscriptions',
        'Databases - Essentials': 'fixed_databases',
        'Role-based Access Control (RBAC)': 'acl',
        'Cloud Accounts': 'cloud_accounts',
        'Tasks': 'tasks',
    }
    return tag_map.get(tag, to_snake_case(tag.replace(' - ', '_').replace(' ', '_').replace('(', '').replace(')', '')))

def main():
    if len(sys.argv) != 3:
        print(__doc__)
        sys.exit(1)
    
    spec_file = Path(sys.argv[1])
    output_dir = Path(sys.argv[2])
    
    if not spec_file.exists():
        print(f"Error: {spec_file} does not exist")
        sys.exit(1)
    
    # Load OpenAPI spec
    with open(spec_file) as f:
        spec = json.load(f)
    
    paths = spec.get('paths', {})
    schemas = spec.get('components', {}).get('schemas', {})
    tags = spec.get('tags', [])
    
    # Create output directory
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate module for each tag
    for tag in tags:
        tag_name = tag['name']
        print(f"Generating module for {tag_name}...")
        
        module_code = generate_module(tag_name, paths, schemas, spec)
        
        # Write to file
        module_name = extract_handler_name(tag_name)
        output_file = output_dir / f"{module_name}.rs"
        output_file.write_text(module_code)
        print(f"  Written to {output_file}")
    
    print("\nGeneration complete!")

if __name__ == '__main__':
    main()