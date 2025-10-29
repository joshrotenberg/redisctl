# 9. Next Steps

## Try It Now

### Installation

```bash
# macOS/Linux
brew install joshrotenberg/brew/redisctl

# Or download from releases
# https://github.com/joshrotenberg/redisctl/releases
```

### Quick Start

```bash
# Set up profile
redisctl profile set myprofile \
  --deployment-type enterprise \
  --url https://cluster:9443 \
  --username admin@cluster.local \
  --use-keyring

# Try a command
redisctl enterprise cluster get
```

### Hands-On Practice

```bash
# Start Docker environment
docker compose up -d

# Run demo scripts
cd examples/presentation
./03-demo-workflow.sh
```

## Learn More

### Documentation
- **[Quick Start](../getting-started/quickstart.md)** - Get started in 5 minutes
- **[Cookbook](../cookbook/README.md)** - Task-oriented recipes
- **[Cloud Reference](../cloud/overview.md)** - All Cloud commands
- **[Enterprise Reference](../enterprise/overview.md)** - All Enterprise commands

### Resources
- **GitHub:** [github.com/joshrotenberg/redisctl](https://github.com/joshrotenberg/redisctl)
- **API Docs:** [docs.rs/redisctl](https://docs.rs/redisctl)
- **Releases:** [GitHub Releases](https://github.com/joshrotenberg/redisctl/releases/latest)

## Get Involved

### Report Issues
Found a bug? Have a feature request?  
[Open an issue](https://github.com/joshrotenberg/redisctl/issues/new)

### Contribute
- Check out [good first issues](https://github.com/joshrotenberg/redisctl/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
- Submit PRs for bug fixes or features
- Improve documentation

### Share Feedback

**What would help you?**
- What workflows would be useful?
- What features are missing?
- How can we improve the UX?

Leave feedback in [GitHub Discussions](https://github.com/joshrotenberg/redisctl/discussions)

## Roadmap

### Near Term
- Additional workflows (migration, Active-Active, etc.)
- Enhanced streaming (metrics, events)
- More human-friendly commands

### Long Term
- Terraform provider (using our libraries)
- Interactive REPL mode
- Additional monitoring integrations

See [open issues](https://github.com/joshrotenberg/redisctl/issues) for details.

## Key Takeaways

1. **FIRST CLI tool** for Redis Cloud and Enterprise
2. **Eliminates fragile scripts** - One command vs 50 lines of bash
3. **Four-layer architecture** - Raw API Human Workflows Tools
4. **Production ready** - 85%+ coverage, v0.6.5, cross-platform
5. **Library-first** - Foundation for Redis Rust ecosystem
6. **Automation-friendly** - JSON output, JMESPath, profiles
7. **Support tools** - 30-second support packages vs 10+ minutes

## Thank You!

Questions? Comments? Feedback?

[Open an issue](https://github.com/joshrotenberg/redisctl/issues)  
[Start a discussion](https://github.com/joshrotenberg/redisctl/discussions)

---

**Previous:** [8. Library Architecture](./08-libraries.md)  
**Appendix →** [rladmin Comparison](./rladmin-comparison.md)

**[Back to Start](./README.md)**
