# Issue Migration Audit

## Summary
- **Total Issues**: 96
- **Open Issues**: ~80
- **Closed Issues**: ~16

## Issue Categories

### By Type:
- **Features**: 45 issues - Implementation of new commands/functionality
- **Tests**: 21 issues - Test coverage improvements
- **Chores**: 8 issues - Maintenance and refactoring
- **Documentation**: 4 issues - Documentation improvements

### By Platform:
- **Cloud API**: ~40% of issues
- **Enterprise API**: ~40% of issues
- **CLI/General**: ~20% of issues

## Migration Strategy

### ✅ **KEEP** - Issues to Migrate (High Value)

#### Architecture & Design Issues:
- #109: Generate Protocol Buffer Definitions (NEW - already in personal repo)
- #87: CLI Architecture Redesign - Tracking Issue
- #85: CLI Phase 4: Advanced Workflow Features
- #84: CLI Phase 3: Implement Core Workflow Commands
- #83: CLI Phase 2: Human-Friendly Commands (likely closed)
- #82: CLI Phase 1: Raw API Access (likely closed)

#### High-Priority Feature Implementations:
- Cloud API command implementations (database, backup, ACL, etc.)
- Enterprise API command implementations (cluster, node, user, etc.)
- Workflow commands for common operations

### ⚠️ **REVIEW** - Selectively Migrate

#### Test Coverage Issues (#93-108):
These are valuable but numerous. Options:
1. Create a single "Improve Test Coverage" meta-issue
2. Track in a project board instead
3. Keep as-is but lower priority

**Recommendation**: Create one meta-issue: "Achieve 80% test coverage across all handlers"

### ❌ **SKIP** - Don't Migrate

#### Internal/Completed Issues:
- Issues specific to Redis internal workflows
- Already completed Phase 1 & 2 issues
- Issues referencing internal systems or people

## Recommended Migration Approach

### Option 1: Fresh Start (Recommended)
Create new, consolidated issues in personal repo:
1. **"Roadmap: Complete API Coverage"** - Track remaining unimplemented endpoints
2. **"Roadmap: Test Coverage"** - Achieve 80% coverage target
3. **"Roadmap: Workflow Commands"** - High-level operations
4. **"Roadmap: Documentation"** - Comprehensive docs
5. **"Feature: Protocol Buffer Definitions"** - Already created

### Option 2: Selective Migration
Use GitHub's issue transfer feature for ~10-15 high-value issues:
- Major architecture issues
- Roadmap tracking issues
- High-priority features

### Option 3: Full Migration
Transfer all open issues (not recommended - too much noise)

## New Issues to Create

After migration, create these fresh issues for public repo:

1. **"Help Wanted: First Good Issues"** - Label easy starter tasks
2. **"Feature Request: Your Ideas"** - Community input
3. **"Documentation: Examples Needed"** - Real-world usage examples
4. **"Testing: Platform Coverage"** - Windows/Mac/Linux testing

## Action Items

1. [ ] Close completed issues in old repo
2. [ ] Create roadmap issues in new repo
3. [ ] Add README section pointing to new repo
4. [ ] Archive old repository after migration

## Issue Templates for New Repo

### Bug Report Template
```yaml
name: Bug Report
about: Report a bug in redisctl
labels: bug
body:
  - type: input
    label: Version
    placeholder: "0.1.0"
  - type: dropdown
    label: Platform
    options: [Cloud, Enterprise, Both]
  - type: textarea
    label: Description
```

### Feature Request Template
```yaml
name: Feature Request
about: Suggest a new feature
labels: enhancement
body:
  - type: dropdown
    label: Category
    options: [CLI, Cloud API, Enterprise API, Documentation]
  - type: textarea
    label: Description
```

## Decision Required

Which migration approach do you prefer?
1. **Fresh Start** - Clean slate, new focused issues
2. **Selective** - Transfer ~10-15 key issues
3. **Full** - Transfer everything

**My Recommendation**: Fresh Start with consolidated roadmap issues. It's cleaner for a public repo and less overwhelming for potential contributors.