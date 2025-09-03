#!/usr/bin/env python3
"""
Generate Rust models and handlers from Redis Cloud OpenAPI spec.

Usage: python generate_from_openapi.py <openapi.json> <output_dir>
"""

import json
import sys
import os
from pathlib import Path
from typing import Dict, List, Any, Optional
import re

def to_snake_case(name: str) -> str:
    """Convert camelCase/PascalCase to snake_case."""
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

def to_pascal_case(name: str) -> str:
    """Convert snake_case to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('_'))

def rust_type_from_openapi(schema: Dict[str, Any], required: bool = True) -> str:
    """Convert OpenAPI type to Rust type."""
    if not schema:
        return "Value"
    
    type_str = schema.get('type', 'object')
    format_str = schema.get('format', '')
    
    # Handle refs
    if '$ref' in schema:
        ref_name = schema['$ref'].split('/')[-1]
        return ref_name
    
    # Handle basic types
    type_map = {
        ('string', 'uuid'): 'String',  # We use String instead of Uuid
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
        ('array', ''): 'Vec<{}>',
        ('object', ''): 'HashMap<String, Value>',
    }
    
    rust_type = type_map.get((type_str, format_str), 'Value')
    
    # Handle arrays
    if type_str == 'array' and 'items' in schema:
        item_type = rust_type_from_openapi(schema['items'])
        rust_type = f'Vec<{item_type}>'
    
    # Wrap in Option if not required
    if not required:
        rust_type = f'Option<{rust_type}>'
    
    return rust_type

def generate_struct(name: str, schema: Dict[str, Any], spec: Dict) -> str:
    """Generate a Rust struct from OpenAPI schema."""
    lines = []
    
    # Add doc comment if description exists
    if 'description' in schema:
        lines.append(f"/// {schema['description']}")
    else:
        lines.append(f"/// {name}")
    
    lines.append("#[derive(Debug, Clone, Serialize, Deserialize)]")
    
    # Check if we need rename_all
    properties = schema.get('properties', {})
    if any('_' in k or k[0].isupper() for k in properties.keys()):
        # Determine the naming convention
        has_camel = any(k[0].islower() and any(c.isupper() for c in k) for k in properties.keys())
        if has_camel:
            lines.append('#[serde(rename_all = "camelCase")]')
    
    lines.append(f"pub struct {name} {{")
    
    required_fields = schema.get('required', [])
    
    for prop_name, prop_schema in properties.items():
        # Convert property name to Rust field name
        field_name = to_snake_case(prop_name)
        
        # Handle reserved keywords
        if field_name in ['type', 'ref', 'use', 'mod']:
            field_name = f'r#{field_name}'
        
        # Get type
        is_required = prop_name in required_fields
        rust_type = rust_type_from_openapi(prop_schema, is_required)
        
        # Add field attributes
        if not is_required or rust_type.startswith('Option<'):
            lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
            if prop_name != field_name and field_name != f'r#{prop_name}':
                lines.append(f'    #[serde(rename = "{prop_name}")]')
        elif prop_name != field_name and field_name != f'r#{prop_name}':
            lines.append(f'    #[serde(rename = "{prop_name}")]')
        
        # Add the field
        lines.append(f"    pub {field_name}: {rust_type},")
        lines.append("")
    
    # Add catch-all for unknown fields
    lines.append("    /// Additional fields from the API")
    lines.append("    #[serde(flatten)]")
    lines.append("    pub extra: Value,")
    
    lines.append("}")
    
    return '\n'.join(lines)

def generate_enum(name: str, values: List[str]) -> str:
    """Generate a Rust enum from OpenAPI enum values."""
    lines = []
    
    lines.append(f"/// {name}")
    lines.append("#[derive(Debug, Clone, Serialize, Deserialize)]")
    
    # Determine serialization format
    if any('-' in v for v in values):
        lines.append('#[serde(rename_all = "kebab-case")]')
    elif all(v.isupper() for v in values):
        lines.append('#[serde(rename_all = "UPPERCASE")]')
    elif all(v.islower() for v in values):
        lines.append('#[serde(rename_all = "lowercase")]')
    
    lines.append(f"pub enum {name} {{")
    
    for value in values:
        # Convert to PascalCase variant name
        variant = to_pascal_case(value.replace('-', '_').replace('.', '_'))
        if value != variant:
            lines.append(f'    #[serde(rename = "{value}")]')
        lines.append(f"    {variant},")
    
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
        'Subscriptions - Essentials': 'fixed_subscriptions',
        'Databases - Essentials': 'fixed_databases',
        'Role-based Access Control (RBAC)': 'acl',
        'Cloud Accounts': 'cloud_accounts',
        'Tasks': 'tasks',
    }
    return tag_map.get(tag, to_snake_case(tag.replace(' ', '_')))

def generate_handler_method(path: str, method: str, operation: Dict) -> Optional[str]:
    """Generate a handler method from an OpenAPI operation."""
    lines = []
    
    # Extract operation details
    operation_id = operation.get('operationId', '')
    summary = operation.get('summary', '')
    parameters = operation.get('parameters', [])
    request_body = operation.get('requestBody', {})
    responses = operation.get('responses', {})
    
    # Generate method name
    method_name = to_snake_case(operation_id) if operation_id else f"{method}_{path.replace('/', '_')}"
    method_name = method_name.replace('get_', '').replace('post_', 'create_').replace('put_', 'update_').replace('delete_', 'delete_')
    
    # Add doc comment
    if summary:
        lines.append(f"    /// {summary}")
    lines.append(f"    /// ")
    lines.append(f"    /// {method.upper()} {path}")
    
    # Start method signature
    lines.append(f"    pub async fn {method_name}(")
    lines.append("        &self,")
    
    # Add path parameters
    path_params = [p for p in parameters if p.get('in') == 'path']
    for param in path_params:
        param_name = to_snake_case(param['name'])
        param_type = rust_type_from_openapi(param.get('schema', {}))
        lines.append(f"        {param_name}: {param_type},")
    
    # Add query parameters
    query_params = [p for p in parameters if p.get('in') == 'query']
    for param in query_params:
        param_name = to_snake_case(param['name'])
        param_type = rust_type_from_openapi(param.get('schema', {}), required=param.get('required', False))
        lines.append(f"        {param_name}: {param_type},")
    
    # Add request body
    if request_body:
        content = request_body.get('content', {})
        if 'application/json' in content:
            schema = content['application/json'].get('schema', {})
            if '$ref' in schema:
                type_name = schema['$ref'].split('/')[-1]
                lines.append(f"        request: &{type_name},")
    
    # Determine return type from responses
    return_type = "Value"
    if '200' in responses or '201' in responses:
        response = responses.get('200', responses.get('201', {}))
        content = response.get('content', {})
        if 'application/json' in content:
            schema = content['application/json'].get('schema', {})
            if '$ref' in schema:
                return_type = schema['$ref'].split('/')[-1]
    
    lines.append(f"    ) -> Result<{return_type}> {{")
    
    # Generate method body
    path_with_params = path
    for param in path_params:
        param_name = param['name']
        path_with_params = path_with_params.replace(f'{{{param_name}}}', f'{{}}')
    
    if path_params:
        format_args = ', '.join(to_snake_case(p['name']) for p in path_params)
        url = f'&format!("{path_with_params}", {format_args})'
    else:
        url = f'"{path}"'
    
    # Add query parameters to URL
    if query_params:
        lines.append("        let mut query = Vec::new();")
        for param in query_params:
            param_name = to_snake_case(param['name'])
            if param.get('required', False):
                lines.append(f'        query.push(format!("{param["name"]}={{}}", {param_name}));')
            else:
                lines.append(f'        if let Some(v) = {param_name} {{')
                lines.append(f'            query.push(format!("{param["name"]}={{}}", v));')
                lines.append('        }')
        lines.append('        let query_string = if query.is_empty() {')
        lines.append('            String::new()')
        lines.append('        } else {')
        lines.append('            format!("?{}", query.join("&"))')
        lines.append('        };')
        url_with_query = f'&format!("{{}}{{}}", {url}, query_string)'
    else:
        url_with_query = url
    
    # Call appropriate client method
    if method == 'get':
        lines.append(f"        self.client.get({url_with_query}).await")
    elif method == 'post':
        if request_body:
            lines.append(f"        self.client.post({url_with_query}, request).await")
        else:
            lines.append(f"        self.client.post({url_with_query}, &serde_json::json!({{}})).await")
    elif method == 'put':
        if request_body:
            lines.append(f"        self.client.put({url_with_query}, request).await")
        else:
            lines.append(f"        self.client.put({url_with_query}, &serde_json::json!({{}})).await")
    elif method == 'delete':
        lines.append(f"        self.client.delete({url_with_query}).await?;")
        lines.append("        Ok(serde_json::json!({}))")
    
    lines.append("    }")
    
    return '\n'.join(lines)

def generate_module(tag: str, paths: Dict[str, Dict], schemas: Dict[str, Any], spec: Dict) -> str:
    """Generate a complete module for a tag."""
    lines = []
    
    module_name = extract_handler_name(tag)
    
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
                    
                    # Collect schemas from parameters and responses
                    for param in operation.get('parameters', []):
                        if 'schema' in param and '$ref' in param['schema']:
                            used_schemas.add(param['schema']['$ref'].split('/')[-1])
                    
                    request_body = operation.get('requestBody', {})
                    if request_body:
                        content = request_body.get('content', {})
                        if 'application/json' in content:
                            schema = content['application/json'].get('schema', {})
                            if '$ref' in schema:
                                used_schemas.add(schema['$ref'].split('/')[-1])
                    
                    for response in operation.get('responses', {}).values():
                        content = response.get('content', {})
                        if 'application/json' in content:
                            schema = content['application/json'].get('schema', {})
                            if '$ref' in schema:
                                used_schemas.add(schema['$ref'].split('/')[-1])
    
    # Generate models section
    if used_schemas:
        lines.append("// " + "=" * 76)
        lines.append("// Models")
        lines.append("// " + "=" * 76)
        lines.append("")
        
        for schema_name in sorted(used_schemas):
            if schema_name in schemas:
                schema = schemas[schema_name]
                if 'enum' in schema:
                    lines.append(generate_enum(schema_name, schema['enum']))
                else:
                    lines.append(generate_struct(schema_name, schema, spec))
                lines.append("")
                lines.append("")
    
    # Generate handler
    lines.append("// " + "=" * 76)
    lines.append("// Handler")
    lines.append("// " + "=" * 76)
    lines.append("")
    
    handler_name = to_pascal_case(module_name) + "Handler"
    lines.append(f"/// {tag} operations handler")
    lines.append(f"pub struct {handler_name} {{")
    lines.append("    client: CloudClient,")
    lines.append("}")
    lines.append("")
    
    lines.append(f"impl {handler_name} {{")
    lines.append("    /// Create a new handler")
    lines.append("    pub fn new(client: CloudClient) -> Self {")
    lines.append("        Self { client }")
    lines.append("    }")
    lines.append("")
    
    # Generate methods
    for path, methods in sorted(tag_paths.items()):
        for method, operation in sorted(methods.items()):
            method_code = generate_handler_method(path, method, operation)
            if method_code:
                lines.append(method_code)
                lines.append("")
    
    lines.append("}")
    
    return '\n'.join(lines)

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
    print("\nNext steps:")
    print("1. Review generated code")
    print("2. Add modules to lib.rs")
    print("3. Run cargo fmt")
    print("4. Write tests")

if __name__ == '__main__':
    main()