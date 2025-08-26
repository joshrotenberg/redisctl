# Migration & Publishing Plan for `redisctl`

## Overview
This document outlines the complete plan for migrating the `redisctl` repository from the Redis Field Engineering organization to a personal account and publishing it to crates.io.

## Phase 1: Pre-Migration Audit & Cleanup

### 1.1 Code Audit
- [ ] **Remove proprietary references**
  - Search for `redis-field-engineering` in all files
  - Check for internal Redis company references
  - Review commit history for any sensitive information
  - Update author emails to personal email

- [ ] **License verification**
  - Confirm dual MIT/Apache-2.0 license is appropriate
  - Ensure all dependencies are compatible
  - Add proper license headers to source files if needed

- [ ] **Documentation cleanup**
  - Remove internal development workflows
  - Clean up README for public audience
  - Remove any internal URLs or references
  - Update links to point to new repository location

### 1.2 Crate Structure Review
```
redisctl/
├── crates/
│   ├── redis-common/     # Core utilities
│   ├── redis-cloud/      # Cloud API client  
│   ├── redis-enterprise/ # Enterprise API client
│   └── redisctl/         # Main CLI binary
```

**Publishing strategy**: Publish as separate crates for modularity:
- `redis-common` or `redisctl-common` - Shared utilities
- `redis-cloud-api` - Cloud client library
- `redis-enterprise-api` - Enterprise client library  
- `redisctl` - Main CLI (depends on above)

## Phase 2: Repository Migration

### 2.1 Create New Personal Repository
```bash
# On your personal GitHub account
gh repo create redisctl --public --description "Unified CLI for Redis Cloud and Enterprise management"
```

### 2.2 Migration Steps
```bash
# 1. Clone current repo with full history
git clone --mirror https://github.com/redis-field-engineering/redisctl.git redisctl-migration

# 2. Update remote to your personal account
cd redisctl-migration
git remote set-url origin https://github.com/YOUR_USERNAME/redisctl.git

# 3. Push all branches and tags
git push --mirror

# 4. Clone the new repo normally for work
cd ..
rm -rf redisctl-migration
git clone https://github.com/YOUR_USERNAME/redisctl.git
cd redisctl
```

### 2.3 Update References
- [ ] Update all `Cargo.toml` files:
  - [ ] `repository` field
  - [ ] `homepage` field
  - [ ] `documentation` field
- [ ] Update README.md:
  - [ ] Repository links
  - [ ] CI/CD badges
  - [ ] Installation instructions
- [ ] Update GitHub Actions workflows
- [ ] Update issue templates if present
- [ ] Update CONTRIBUTING.md links

## Phase 3: Crates.io Preparation

### 3.1 Crate Naming Strategy
Check availability on crates.io:
```bash
# Check if names are available
cargo search redis-common --limit 1
cargo search redis-cloud-api --limit 1
cargo search redis-enterprise-api --limit 1
cargo search redisctl --limit 1
```

Alternative naming if taken:
- `redisctl-common`
- `redisctl-cloud`
- `redisctl-enterprise`
- `redisctl` (main CLI)

### 3.2 Version Strategy
Start with `0.1.0` for all crates:
- Follows semantic versioning
- Indicates pre-1.0 development phase
- Allows for breaking changes before 1.0

### 3.3 Cargo.toml Template
Each crate needs proper metadata:

```toml
[package]
name = "redisctl"
version = "0.1.0"
edition = "2024"
rust-version = "1.89"
authors = ["Your Name <your.email@example.com>"]
license = "MIT OR Apache-2.0"
description = "Unified CLI for Redis Cloud and Enterprise management"
homepage = "https://github.com/YOUR_USERNAME/redisctl"
repository = "https://github.com/YOUR_USERNAME/redisctl"
documentation = "https://docs.rs/redisctl"
readme = "README.md"
keywords = ["redis", "cli", "cloud", "enterprise", "database"]
categories = ["command-line-utilities", "api-bindings", "database"]

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Phase 4: Publishing Workflow

### 4.1 Publishing Order
Due to dependencies, publish in this order:
1. `redis-common` (no internal dependencies)
2. `redis-cloud-api` (depends on redis-common)
3. `redis-enterprise-api` (depends on redis-common)  
4. `redisctl` (depends on all above)

### 4.2 Pre-publish Checklist
For each crate, run:
```bash
# Navigate to crate
cd crates/redis-common

# Verify everything builds
cargo check --all-features

# Run all tests
cargo test --all-features

# Check for linting issues
cargo clippy --all-targets --all-features -- -D warnings

# Build documentation
cargo doc --no-deps --open

# Verify package contents
cargo package --list

# Do a dry run
cargo publish --dry-run
```

### 4.3 Manual Publishing Process
```bash
# 1. Publish redis-common
cd crates/redis-common
cargo publish

# 2. Wait for crates.io to index (usually 1-2 minutes)
sleep 120

# 3. Publish redis-cloud-api
cd ../redis-cloud
cargo publish

# 4. Wait again
sleep 120

# 5. Publish redis-enterprise-api
cd ../redis-enterprise
cargo publish

# 6. Wait again
sleep 120

# 7. Finally, publish the main CLI
cd ../redisctl
cargo publish
```

### 4.4 Automated Publishing (GitHub Actions)
Create `.github/workflows/publish.yml`:
```yaml
name: Publish to crates.io

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Login to crates.io
        run: cargo login ${{ secrets.CARGO_TOKEN }}
      
      # Publish in dependency order
      - name: Publish redis-common
        run: |
          cd crates/redis-common
          cargo publish --no-verify
        continue-on-error: true  # In case it's already published
        
      - name: Wait for indexing
        run: sleep 30
        
      - name: Publish redis-cloud-api
        run: |
          cd crates/redis-cloud
          cargo publish --no-verify
        continue-on-error: true
        
      - name: Wait for indexing
        run: sleep 30
        
      - name: Publish redis-enterprise-api
        run: |
          cd crates/redis-enterprise
          cargo publish --no-verify
        continue-on-error: true
        
      - name: Wait for indexing
        run: sleep 30
        
      - name: Publish redisctl
        run: |
          cd crates/redisctl
          cargo publish --no-verify
```

## Phase 5: Documentation & Community

### 5.1 Documentation Updates
- [ ] **README.md**: 
  - Clear installation instructions
  - Quick start guide
  - Feature overview
  - API coverage status
  - Contributing guidelines link
  
- [ ] **CHANGELOG.md**: 
  - Initial v0.1.0 release notes
  - Future version template
  
- [ ] **CONTRIBUTING.md**: 
  - How to contribute
  - Code of conduct
  - Development setup
  - Testing guidelines
  - Pull request process
  
- [ ] **docs/**: 
  - Update mdBook documentation
  - Publish to GitHub Pages
  
- [ ] **examples/**: 
  - Basic usage examples
  - Profile configuration
  - API access patterns
  - Output formatting examples

### 5.2 Release Notes Template
```markdown
# redisctl v0.1.0

## Overview
First public release of redisctl - a unified CLI for managing Redis Cloud and Redis Enterprise deployments.

## Features
- 🚀 Unified interface for both Redis Cloud and Enterprise
- 🔧 Smart command routing based on deployment type
- 📊 Multiple output formats (JSON, YAML, Table)
- 🔍 JMESPath query support
- 🔐 Secure credential management with profiles
- 📦 Available as separate libraries for custom integrations

## Installation
```bash
cargo install redisctl
```

## Quick Start
[Include basic usage examples]

## API Coverage
- Redis Cloud: 95%+ API coverage
- Redis Enterprise: 100% API coverage

## Known Limitations
- Some Cloud API endpoints pending implementation
- Workflow commands in development

## Contributing
Contributions welcome! See CONTRIBUTING.md for guidelines.
```

## Phase 6: Post-Publishing Tasks

### 6.1 Community Setup
- [ ] Enable GitHub Discussions for Q&A
- [ ] Create issue templates:
  - Bug report
  - Feature request
  - Documentation improvement
- [ ] Set up GitHub Projects for roadmap
- [ ] Configure branch protection rules
- [ ] Set up CODEOWNERS file

### 6.2 CI/CD Improvements
- [ ] Add code coverage reporting
- [ ] Set up dependency updates (Dependabot)
- [ ] Add security scanning
- [ ] Configure automatic releases

### 6.3 Promotion Strategy
- [ ] Submit to "This Week in Rust" newsletter
- [ ] Post on Reddit:
  - r/rust
  - r/redis
  - r/devops
- [ ] Share on social media:
  - Twitter/X
  - LinkedIn
  - Mastodon
- [ ] Write announcement blog post covering:
  - Problem it solves
  - Key features
  - Architecture decisions
  - Future roadmap

## Immediate Action Items

### Week 1: Preparation
1. **Audit current code** for proprietary references
2. **Run full test suite** to ensure stability
3. **Check crate name availability** on crates.io
4. **Create personal GitHub repo**
5. **Set up crates.io account** and generate API token

### Week 2: Migration
1. **Migrate repository** with full history
2. **Update all references** to new location
3. **Test build and CI/CD** in new location
4. **Update documentation** for public audience
5. **Create release notes**

### Week 3: Publishing
1. **Final review** of all crate metadata
2. **Publish to crates.io** in correct order
3. **Verify installation** works via `cargo install`
4. **Announce release** on appropriate channels
5. **Monitor for initial feedback**

## Decision Points

### Required Decisions
1. **GitHub username**: What's your personal GitHub account?
2. **Crate naming**: Final decision on crate names
3. **Author information**: Name and email for published crates
4. **Timeline**: Target date for publication
5. **Version number**: Confirm starting with 0.1.0

### Optional Decisions
1. **Documentation hosting**: GitHub Pages vs docs.rs only
2. **Discord/Slack**: Create community chat?
3. **Logo/Branding**: Create project logo?
4. **Website**: Create landing page?

## Success Metrics
- [ ] All crates successfully published to crates.io
- [ ] Installation via `cargo install redisctl` works
- [ ] CI/CD passes in new repository
- [ ] Documentation accessible on docs.rs
- [ ] At least one external user successfully uses the tool

## Risk Mitigation
1. **Naming conflicts**: Have backup names ready
2. **Dependency issues**: Test with fresh Cargo.lock
3. **Breaking changes**: Start with 0.x version
4. **Security concerns**: Run `cargo audit` before publishing
5. **Size concerns**: Check binary size, consider optimization

## Notes
- Keep the original repository archived for reference
- Consider transferring open issues to new repository
- Maintain a migration notice in the old repository
- Set up redirects if possible

---

*Last Updated: [Current Date]*
*Status: Planning Phase*