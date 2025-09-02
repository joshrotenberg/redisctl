#!/usr/bin/env python3
"""
API Coverage Checker for Redis Enterprise and Redis Cloud

This script analyzes the handler implementations to determine API coverage
by comparing implemented methods against known API endpoints.
"""

import os
import re
import json
from pathlib import Path
from typing import Dict, List, Set, Tuple

def find_handler_methods(handler_path: Path) -> Dict[str, List[str]]:
    """Find all public async functions in handler files."""
    handlers = {}
    
    for handler_file in handler_path.glob("*.rs"):
        if handler_file.name == "mod.rs":
            continue
            
        with open(handler_file, 'r') as f:
            content = f.read()
            
        # Find handler struct name
        handler_match = re.search(r'pub struct (\w+Handler)', content)
        if not handler_match:
            continue
            
        handler_name = handler_match.group(1)
        
        # Find all public async functions
        methods = re.findall(r'pub async fn (\w+)', content)
        handlers[handler_name] = methods
        
    return handlers

def find_api_calls(handler_path: Path) -> Dict[str, Set[Tuple[str, str]]]:
    """Find all API endpoint calls in handler files."""
    api_calls = {}
    
    for handler_file in handler_path.glob("*.rs"):
        if handler_file.name == "mod.rs":
            continue
            
        with open(handler_file, 'r') as f:
            content = f.read()
            
        handler_name = handler_file.stem
        endpoints = set()
        
        # Find all client.get/post/put/delete calls
        patterns = [
            r'\.get\(&format!\("([^"]+)"',
            r'\.post\(&format!\("([^"]+)"',
            r'\.put\(&format!\("([^"]+)"',
            r'\.delete\(&format!\("([^"]+)"',
            r'\.get\("([^"]+)"\)',
            r'\.post\("([^"]+)"',
            r'\.put\("([^"]+)"',
            r'\.delete\("([^"]+)"',
        ]
        
        for pattern in patterns:
            matches = re.findall(pattern, content)
            for match in matches:
                # Extract HTTP method from pattern
                if 'get' in pattern:
                    method = 'GET'
                elif 'post' in pattern:
                    method = 'POST'
                elif 'put' in pattern:
                    method = 'PUT'
                elif 'delete' in pattern:
                    method = 'DELETE'
                endpoints.add((method, match))
                
        api_calls[handler_name] = endpoints
        
    return api_calls

def analyze_typed_coverage(handler_path: Path) -> Dict[str, Dict[str, bool]]:
    """Analyze which methods have typed vs raw versions."""
    typed_coverage = {}
    
    for handler_file in handler_path.glob("*.rs"):
        if handler_file.name == "mod.rs":
            continue
            
        with open(handler_file, 'r') as f:
            content = f.read()
            
        handler_name = handler_file.stem
        methods = re.findall(r'pub async fn (\w+).*?-> Result<([^>]+)>', content, re.DOTALL)
        
        method_types = {}
        for method_name, return_type in methods:
            is_typed = 'Value' not in return_type or method_name.endswith('_raw')
            method_types[method_name] = is_typed
            
        typed_coverage[handler_name] = method_types
        
    return typed_coverage

def check_dual_interface(typed_coverage: Dict[str, Dict[str, bool]]) -> Dict[str, List[str]]:
    """Check which methods have both typed and raw versions."""
    violations = {}
    
    for handler, methods in typed_coverage.items():
        handler_violations = []
        
        for method_name, is_typed in methods.items():
            if method_name.endswith('_raw'):
                continue
                
            # Check if there's a corresponding _raw version
            raw_version = f"{method_name}_raw"
            if is_typed and raw_version not in methods:
                handler_violations.append(f"{method_name} (missing _raw version)")
                
        if handler_violations:
            violations[handler] = handler_violations
            
    return violations

def main():
    """Main function to analyze API coverage."""
    # Paths to handler directories
    enterprise_handlers = Path("crates/redis-enterprise/src")
    cloud_handlers = Path("crates/redis-cloud/src/handlers")
    
    print("=" * 80)
    print("API COVERAGE ANALYSIS")
    print("=" * 80)
    
    # Analyze Enterprise
    if enterprise_handlers.exists():
        print("\n## Redis Enterprise\n")
        
        # Find all handler files
        handlers = list(enterprise_handlers.glob("*.rs"))
        handler_files = [h for h in handlers if h.name != "lib.rs" and h.name != "mod.rs" and h.name != "error.rs" and h.name != "client.rs"]
        
        print(f"Found {len(handler_files)} handler modules")
        
        # Get methods and API calls
        methods = find_handler_methods(enterprise_handlers)
        api_calls = find_api_calls(enterprise_handlers)
        typed_coverage = analyze_typed_coverage(enterprise_handlers)
        
        total_methods = sum(len(m) for m in methods.values())
        total_endpoints = sum(len(e) for e in api_calls.values())
        
        print(f"Total methods: {total_methods}")
        print(f"Total unique API endpoints: {total_endpoints}")
        
        # Check for dual interface violations
        violations = check_dual_interface(typed_coverage)
        if violations:
            print("\n### Dual Interface Violations:")
            for handler, issues in violations.items():
                print(f"  {handler}:")
                for issue in issues:
                    print(f"    - {issue}")
        
        # List all endpoints
        print("\n### API Endpoints by Handler:")
        for handler, endpoints in sorted(api_calls.items()):
            if endpoints:
                print(f"\n  {handler}:")
                for method, endpoint in sorted(endpoints):
                    print(f"    {method:6} {endpoint}")
    
    # Analyze Cloud
    if cloud_handlers.exists():
        print("\n" + "=" * 80)
        print("\n## Redis Cloud\n")
        
        handler_files = list(cloud_handlers.glob("*.rs"))
        handler_files = [h for h in handler_files if h.name != "mod.rs"]
        
        print(f"Found {len(handler_files)} handler modules")
        
        # Get methods and API calls
        methods = find_handler_methods(cloud_handlers)
        api_calls = find_api_calls(cloud_handlers)
        typed_coverage = analyze_typed_coverage(cloud_handlers)
        
        total_methods = sum(len(m) for m in methods.values())
        total_endpoints = sum(len(e) for e in api_calls.values())
        
        print(f"Total methods: {total_methods}")
        print(f"Total unique API endpoints: {total_endpoints}")
        
        # Check typed coverage
        total_typed = 0
        total_raw_only = 0
        for handler, methods in typed_coverage.items():
            for method_name, is_typed in methods.items():
                if not method_name.endswith('_raw'):
                    if is_typed:
                        total_typed += 1
                    else:
                        total_raw_only += 1
        
        print(f"Typed methods: {total_typed}")
        print(f"Raw-only methods: {total_raw_only}")
        print(f"Typed coverage: {total_typed / (total_typed + total_raw_only) * 100:.1f}%")
        
        # Check for dual interface violations
        violations = check_dual_interface(typed_coverage)
        if violations:
            print("\n### Dual Interface Violations:")
            for handler, issues in violations.items():
                print(f"  {handler}:")
                for issue in issues:
                    print(f"    - {issue}")
        
        # List handlers with no typed methods
        print("\n### Handlers Needing Typed Interfaces:")
        for handler, methods in typed_coverage.items():
            typed_count = sum(1 for m, t in methods.items() if t and not m.endswith('_raw'))
            if typed_count == 0:
                method_count = sum(1 for m in methods.keys() if not m.endswith('_raw'))
                print(f"  {handler}: 0/{method_count} methods typed")

if __name__ == "__main__":
    main()