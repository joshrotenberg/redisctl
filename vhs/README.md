# VHS Demo Recordings

This directory contains VHS tape files for creating animated terminal demonstrations of redisctl.

## Prerequisites

Install VHS and its dependencies:

```bash
# macOS
brew install vhs ffmpeg

# Linux
# Install from https://github.com/charmbracelet/vhs/releases
# Also need: ffmpeg and ttyd
```

## Available Demos

- **quick-start.tape** - Basic introduction to redisctl
- **profile-management.tape** - Managing authentication profiles
- **enterprise-demo.tape** - Working with Redis Enterprise clusters
- **cloud-demo.tape** - Managing Redis Cloud resources
- **async-operations.tape** - Demonstration of async operation tracking

## Building the Demos

Build all demos:
```bash
./generate-demos.sh
```

Build a specific demo:
```bash
vhs vhs/quick-start.tape
```

## Recording Your Own Demo

Create a new tape file or record a session:

```bash
# Record a new session
vhs record > my-demo.tape

# Edit the tape file
vim my-demo.tape

# Generate the GIF
vhs my-demo.tape
```

## Customization

Each tape file can be customized with:
- Terminal dimensions (`Set Width`, `Set Height`)
- Color themes (`Set Theme`)
- Font size (`Set FontSize`)
- Typing speed (`Set TypingSpeed`)
- Output format (GIF, MP4, WebM)

See the [VHS documentation](https://github.com/charmbracelet/vhs) for all available options.

## Notes

- The demos use placeholder credentials and mock data
- For real demonstrations, ensure sensitive data is not exposed
- GIFs are not tracked in git (add to .gitignore)
- Consider hosting GIFs on a CDN or GitHub releases for documentation