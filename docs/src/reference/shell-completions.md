# Shell Completions

redisctl supports tab completion for all major shells. This guide shows how to install and configure completions for your shell.

## Generating Completions

First, generate the completion script for your shell:

```bash
# Bash
redisctl completions bash > redisctl.bash

# Zsh
redisctl completions zsh > _redisctl

# Fish
redisctl completions fish > redisctl.fish

# PowerShell
redisctl completions powershell > redisctl.ps1

# Elvish
redisctl completions elvish > redisctl.elv
```

## Installing Completions

### Bash

```bash
# Linux - User-specific
redisctl completions bash > ~/.local/share/bash-completion/completions/redisctl

# Linux - System-wide (requires sudo)
sudo redisctl completions bash > /usr/share/bash-completion/completions/redisctl

# macOS with Homebrew
redisctl completions bash > $(brew --prefix)/etc/bash_completion.d/redisctl

# Reload your shell
source ~/.bashrc
# or start a new terminal
```

### Zsh

```bash
# Add to your fpath (usually in ~/.zshrc)
echo 'fpath=(~/.zsh/completions $fpath)' >> ~/.zshrc

# Create directory if needed
mkdir -p ~/.zsh/completions

# Generate completion file
redisctl completions zsh > ~/.zsh/completions/_redisctl

# Reload your shell
source ~/.zshrc
# or start a new terminal
```

### Fish

```bash
# Generate completion file
redisctl completions fish > ~/.config/fish/completions/redisctl.fish

# Completions are loaded automatically in new shells
# or reload current shell:
source ~/.config/fish/config.fish
```

### PowerShell

```powershell
# Add to your PowerShell profile
redisctl completions powershell >> $PROFILE

# Or save to a file and source it
redisctl completions powershell > redisctl.ps1
Add-Content $PROFILE ". $PWD\redisctl.ps1"

# Reload profile
. $PROFILE
```

### Elvish

```bash
# Generate completion file
redisctl completions elvish > ~/.elvish/lib/redisctl.elv

# Add to rc.elv
echo "use redisctl" >> ~/.elvish/rc.elv

# Reload shell
exec elvish
```

## Testing Completions

After installation, test that completions work:

```bash
# Type and press Tab
redisctl <Tab>
# Should show: api, auth, cloud, enterprise, profile, etc.

# Try sub-commands
redisctl cloud <Tab>
# Should show: database, subscription, user, etc.

# Try options
redisctl --<Tab>
# Should show: --help, --version, --profile, --output, etc.
```

## Troubleshooting

### Completions Not Working

1. **Check shell configuration:**
   ```bash
   # Bash - verify completion is enabled
   echo $BASH_COMPLETION_COMPAT_DIR
   
   # Zsh - check fpath
   echo $fpath
   
   # Fish - check completion directory
   ls ~/.config/fish/completions/
   ```

2. **Reload your shell:**
   ```bash
   # Option 1: Source config file
   source ~/.bashrc  # or ~/.zshrc, etc.
   
   # Option 2: Start new shell
   exec $SHELL
   
   # Option 3: Open new terminal
   ```

3. **Verify file permissions:**
   ```bash
   # Check completion file exists and is readable
   ls -la ~/.local/share/bash-completion/completions/redisctl
   # or your shell's completion directory
   ```

### Updating Completions

When updating redisctl, regenerate completions to get new commands:

```bash
# Example for Bash
redisctl completions bash > ~/.local/share/bash-completion/completions/redisctl
source ~/.bashrc
```

### Custom Completion Directories

If using non-standard directories:

```bash
# Bash - add to .bashrc
source /path/to/redisctl.bash

# Zsh - add to .zshrc  
fpath=(/path/to/completions $fpath)
autoload -U compinit && compinit

# Fish - add to config.fish
source /path/to/redisctl.fish
```

## Tips

- **Auto-update completions:** Add completion generation to your dotfiles setup
- **Multiple shells:** Generate completions for all shells you use
- **Container usage:** Mount completion files when using Docker:
  ```bash
  docker run -v ~/.local/share/bash-completion:/etc/bash_completion.d:ro ...
  ```
- **CI/CD:** Include completion generation in your deployment scripts

## See Also

- [Installation Guide](./installation.md) - Installing redisctl
- [Configuration](./configuration.md) - Setting up profiles
- [Quick Start](./quickstart.md) - Getting started with redisctl