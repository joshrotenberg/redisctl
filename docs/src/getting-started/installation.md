# Installation

## Docker

The quickest way to try redisctl with no installation:

```bash
# Run commands directly
docker run --rm ghcr.io/redis-developer/redisctl --help

# With environment variables
docker run --rm \
  -e REDIS_CLOUD_API_KEY="your-key" \
  -e REDIS_CLOUD_SECRET_KEY="your-secret" \
  ghcr.io/redis-developer/redisctl cloud database list
```

## Homebrew (macOS/Linux)

The easiest way to install on macOS or Linux:

```bash
# Install directly (automatically taps the repository)
brew install redis-developer/homebrew-tap/redisctl

# Or tap first, then install
brew tap redis-developer/homebrew-tap
brew install redisctl
```

This will:
- Install the latest stable version
- Set up the binary in your PATH
- Enable automatic updates via `brew upgrade`

To upgrade to the latest version:
```bash
brew upgrade redisctl
```

## Binary Releases

Download the latest release for your platform from the [GitHub releases page](https://github.com/redis-developer/redisctl/releases).

### Linux/macOS
```bash
# Download the binary (replace VERSION and PLATFORM)
curl -L https://github.com/redis-developer/redisctl/releases/download/vVERSION/redisctl-PLATFORM.tar.gz | tar xz

# Move to PATH
sudo mv redisctl /usr/local/bin/

# Make executable
chmod +x /usr/local/bin/redisctl
```

### Windows
Download the `.zip` file from the releases page and extract to a directory in your PATH.

## From Cargo

If you have Rust installed:

```bash
# Basic installation
cargo install redisctl

# With secure credential storage support (recommended)
cargo install redisctl --features secure-storage
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `secure-storage` | Enables OS keyring support for secure credential storage (recommended) |
| `cloud-only` | Builds only Cloud functionality (smaller binary) |
| `enterprise-only` | Builds only Enterprise functionality (smaller binary) |

## From Source

```bash
git clone https://github.com/redis-developer/redisctl.git
cd redisctl

# Basic installation
cargo install --path crates/redisctl

# With secure storage support (recommended)
cargo install --path crates/redisctl --features secure-storage

# Development build with all features
cargo build --release --all-features
```

## Shell Completions

`redisctl` can generate shell completions for better command-line experience.

### Bash
```bash
# Generate completion
redisctl completions bash > ~/.local/share/bash-completion/completions/redisctl

# Or system-wide (requires sudo)
redisctl completions bash | sudo tee /usr/share/bash-completion/completions/redisctl

# Reload your shell or source the completion
source ~/.local/share/bash-completion/completions/redisctl
```

### Zsh
```bash
# Add to your fpath (usually in ~/.zshrc)
redisctl completions zsh > ~/.zsh/completions/_redisctl

# Or use oh-my-zsh custom completions
redisctl completions zsh > ~/.oh-my-zsh/custom/completions/_redisctl

# Reload shell
exec zsh
```

### Fish
```bash
# Generate completion
redisctl completions fish > ~/.config/fish/completions/redisctl.fish

# Completions are loaded automatically
```

### PowerShell
```powershell
# Generate completion
redisctl completions powershell | Out-String | Invoke-Expression

# To make permanent, add to your PowerShell profile
redisctl completions powershell >> $PROFILE
```

### Elvish
```bash
# Generate completion
redisctl completions elvish > ~/.config/elvish/lib/redisctl.elv

# Add to rc.elv
echo "use redisctl" >> ~/.config/elvish/rc.elv
```

## Verify Installation

```bash
redisctl --version
```

## Platform-Specific Binaries

For specific deployment scenarios, you can build platform-specific binaries:

```bash
# Cloud-only binary (smaller size)
cargo build --release --features cloud-only --bin redis-cloud

# Enterprise-only binary
cargo build --release --features enterprise-only --bin redis-enterprise
```

## Next Steps

- Configuration - Set up your credentials
- Quick Start - Run your first commands