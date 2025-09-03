#!/usr/bin/env python3
"""
Convert OpenAPI-generated models to our style.

Usage: python convert_generated_model.py <input_file> [output_file]

Conversions:
- Remove generated comments/headers
- Remove Default trait and new() methods
- Change uuid::Uuid to String
- Remove Box<> wrappers
- Add doc comments with OpenAPI line references
- Keep serde attributes
- Add #[serde(flatten)] extra field where appropriate
"""

import re
import sys
from pathlib import Path

def convert_model(content: str, model_name: str = None) -> str:
    """Convert a generated model file to our style."""
    
    lines = content.split('\n')
    output_lines = []
    
    # Skip generated header comments
    i = 0
    while i < len(lines) and (lines[i].startswith('/*') or lines[i].startswith(' *') or lines[i].strip() == '*/'):
        i += 1
    
    # Skip empty lines after header
    while i < len(lines) and not lines[i].strip():
        i += 1
    
    # Process the rest of the file
    in_impl_block = False
    skip_default_impl = False
    struct_name = None
    added_imports = False
    
    while i < len(lines):
        line = lines[i]
        
        # Skip use crate::models; if present
        if line.strip() == 'use crate::models;':
            i += 1
            continue
            
        # Skip existing serde imports
        if 'use serde::{Deserialize, Serialize}' in line:
            i += 1
            continue
            
        # Add imports at the first non-empty line
        if not added_imports and line.strip() and not line.startswith('use'):
            output_lines.append('use serde::{Deserialize, Serialize};')
            output_lines.append('use serde_json::Value;')
            output_lines.append('')
            added_imports = True
            
        # Convert uuid::Uuid to String
        line = line.replace('Option<uuid::Uuid>', 'Option<String>')
        line = line.replace('uuid::Uuid', 'String')
        
        # Remove Box<> wrappers but keep the Option
        line = re.sub(r'Option<Box<(.+?)>>', r'Option<\1>', line)
        line = re.sub(r'Box<(.+?)>', r'\1', line)
        
        # Remove Default from derive macros
        if '#[derive(' in line:
            line = line.replace(', Default', '').replace('Default, ', '')
            
        # Detect struct name
        if 'pub struct' in line:
            match = re.search(r'pub struct (\w+)', line)
            if match:
                struct_name = match.group(1)
        
        # Skip impl blocks for new() methods
        if line.strip().startswith('impl ') and struct_name and struct_name in line:
            in_impl_block = True
            # Check if this is just for new() method
            j = i + 1
            has_only_new = True
            brace_count = 1
            while j < len(lines) and brace_count > 0:
                if '{' in lines[j]:
                    brace_count += lines[j].count('{')
                if '}' in lines[j]:
                    brace_count -= lines[j].count('}')
                if 'pub fn' in lines[j] and 'new' not in lines[j]:
                    has_only_new = False
                j += 1
            
            if has_only_new:
                # Skip the entire impl block
                brace_count = 1
                i += 1
                while i < len(lines) and brace_count > 0:
                    if '{' in lines[i]:
                        brace_count += lines[i].count('{')
                    if '}' in lines[i]:
                        brace_count -= lines[i].count('}')
                    i += 1
                continue
        
        # Skip Default trait impl
        if 'impl Default for' in line:
            skip_default_impl = True
            brace_count = 0
            i += 1
            continue
            
        if skip_default_impl:
            if '{' in line:
                brace_count += line.count('{')
            if '}' in line:
                brace_count -= line.count('}')
            if brace_count == 0 and '}' in line:
                skip_default_impl = False
            i += 1
            continue
        
        # Add line to output
        output_lines.append(line)
        i += 1
    
    # Join and clean up
    result = '\n'.join(output_lines)
    
    # Remove multiple consecutive blank lines
    result = re.sub(r'\n\n\n+', '\n\n', result)
    
    # Clean up trailing whitespace
    result = '\n'.join(line.rstrip() for line in result.split('\n'))
    
    return result

def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    
    input_file = Path(sys.argv[1])
    if not input_file.exists():
        print(f"Error: {input_file} does not exist")
        sys.exit(1)
    
    content = input_file.read_text()
    model_name = input_file.stem.replace('_', ' ').title()
    
    converted = convert_model(content, model_name)
    
    if len(sys.argv) > 2:
        output_file = Path(sys.argv[2])
        output_file.write_text(converted)
        print(f"Converted {input_file} -> {output_file}")
    else:
        print(converted)

if __name__ == '__main__':
    main()