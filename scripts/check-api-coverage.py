#!/usr/bin/env python3
"""
API Coverage Checker for Redis Enterprise and Redis Cloud

Enhanced to parse the local Redis Enterprise REST routing table (HTML) and
compare code endpoints against documented ones. By default, only /v1 and /v2
endpoints are considered for coverage (legacy /crdbs and /crdb_tasks are
excluded). Use this to identify missing endpoints and track progress toward
100% API coverage.

Optional dependencies: none (HTML parsed via regex). BeautifulSoup/lxml are not
required.
"""

import os
import re
import json
from pathlib import Path
from typing import Dict, List, Set, Tuple, Iterable

import argparse
import sys

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
        
        # Find all client.get/post/put/delete/patch calls
        patterns = [
            r'\.get\(&format!\("([^"]+)"',
            r'\.post\(&format!\("([^"]+)"',
            r'\.put\(&format!\("([^"]+)"',
            r'\.delete\(&format!\("([^"]+)"',
            r'\.patch\(&format!\("([^"]+)"',
            r'\.get\("([^"]+)"\)',
            r'\.post\("([^"]+)"',
            r'\.put\("([^"]+)"',
            r'\.delete\("([^"]+)"',
            r'\.patch\("([^"]+)"',
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
                elif 'patch' in pattern:
                    method = 'PATCH'
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


def normalize_path(path: str, drop_query: bool = True) -> str:
    """Normalize endpoint path placeholders to a canonical form for comparison."""
    # Replace Rust format placeholders and doc placeholders with {}
    path = re.sub(r"\{[^\}]+\}", "{}", path)
    path = re.sub(r"\((?:int:)?[^\)]+\)", "{}", path)
    # Normalize duplicate slashes
    path = re.sub(r"//+", "/", path)
    if drop_query:
        path = path.split("?")[0]
    return path


def parse_routing_table_html(html_path: Path) -> Set[Tuple[str, str]]:
    """Parse documented endpoints from the local routing table HTML using regex only.

    Returns a set of (METHOD, PATH) tuples.
    """
    if not html_path.exists():
        return set()
    html = html_path.read_text(errors="ignore")
    # Match <code class="xref">GET /v1/...</code>
    pattern = re.compile(r'<code class="xref">\s*([A-Z]+)\s+([^<]+?)\s*</code>')
    endpoints: Set[Tuple[str, str]] = set()
    for method, path in pattern.findall(html):
        endpoints.add((method, normalize_path(path)))
    return endpoints


def filter_scope(endpoints: Iterable[Tuple[str, str]], include_v2: bool = True) -> Set[Tuple[str, str]]:
    """Keep only /v1 and (optionally) /v2 endpoints. Excludes legacy /crdbs, /crdb_tasks."""
    scoped: Set[Tuple[str, str]] = set()
    for method, path in endpoints:
        if path.startswith("/v1/"):
            scoped.add((method, path))
        elif include_v2 and path.startswith("/v2/"):
            scoped.add((method, path))
        # else: skip legacy or other roots
    return scoped


def group_by_category(endpoints: Iterable[Tuple[str, str]]) -> Dict[str, List[Tuple[str, str]]]:
    """Group endpoints by category (the first segment after /v1/ or /v2/)."""
    grouped: Dict[str, List[Tuple[str, str]]] = {}
    for method, path in endpoints:
        root = "unknown"
        if path.startswith("/v1/") or path.startswith("/v2/"):
            root = path.split("/")[2]
        grouped.setdefault(root, []).append((method, path))
    return grouped

def main():
    """Main function to analyze API coverage."""
    parser = argparse.ArgumentParser(description="API coverage analyzer")
    parser.add_argument("--fail-on-gaps", action="store_true", help="Exit non-zero if gaps exist in scoped endpoints")
    parser.add_argument("--routing-table", default="tmp/rest-html/http-routingtable.html", help="Path to routing table HTML")
    parser.add_argument("--scope", choices=["v1", "v1v2"], default="v1v2", help="Endpoint scope to require coverage for")
    args = parser.parse_args()
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
        print(f"Total unique API endpoints (from code): {total_endpoints}")
        
        # Check for dual interface violations
        violations = check_dual_interface(typed_coverage)
        if violations:
            print("\n### Dual Interface Violations:")
            for handler, issues in violations.items():
                print(f"  {handler}:")
                for issue in issues:
                    print(f"    - {issue}")
        
        # List all endpoints
        print("\n### API Endpoints by Handler (from code):")
        for handler, endpoints in sorted(api_calls.items()):
            if endpoints:
                print(f"\n  {handler}:")
                for method, endpoint in sorted(endpoints):
                    print(f"    {method:6} {endpoint}")

        # Compare with documented endpoints (scoped)
        print("\n---\n")
        print("### Documentation Diff (scoped)")
        rt_path = Path(args.routing_table)
        doc_endpoints_all = parse_routing_table_html(rt_path)
        if not doc_endpoints_all:
            print(f"Routing table not found at {rt_path}; skipping doc diff.")
        else:
            include_v2 = args.scope == "v1v2"
            doc_scoped = filter_scope(doc_endpoints_all, include_v2=include_v2)

            # Normalize code endpoints similarly and scope to v1/v2
            code_eps: Set[Tuple[str, str]] = set()
            for handler, eps in api_calls.items():
                for method, endpoint in eps:
                    norm = normalize_path(endpoint)
                    # Only keep v1/v2
                    if norm.startswith("/v1/") or (include_v2 and norm.startswith("/v2/")):
                        code_eps.add((method, norm))

            missing = sorted(doc_scoped - code_eps)
            extra = sorted(code_eps - doc_scoped)

            print(f"Scoped documented endpoints: {len(doc_scoped)}")
            print(f"Scoped code endpoints:       {len(code_eps)}")
            print(f"Missing in code:             {len(missing)}")
            print(f"Extra in code:               {len(extra)}")

            # Group missing by category
            if missing:
                print("\n#### Missing by Category:")
                by_cat = group_by_category(missing)
                for cat in sorted(by_cat.keys()):
                    print(f"- {cat}: {len(by_cat[cat])}")
                    for m, p in by_cat[cat][:6]:
                        print(f"    {m:6} {p}")
                    if len(by_cat[cat]) > 6:
                        print("    ...")

            if args.fail_on_gaps and missing:
                print("\nCoverage gaps detected.", file=sys.stderr)
                sys.exit(1)
    
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
