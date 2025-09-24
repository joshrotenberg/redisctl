#!/bin/bash

# Generate all VHS demo recordings for redisctl

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🎬 Generating redisctl demo recordings..."
echo

# Check if VHS is installed
if ! command -v vhs &> /dev/null; then
    echo -e "${RED}Error: VHS is not installed.${NC}"
    echo "Install it with: brew install vhs (macOS) or see https://github.com/charmbracelet/vhs"
    exit 1
fi

# Check if ffmpeg is installed
if ! command -v ffmpeg &> /dev/null; then
    echo -e "${YELLOW}Warning: ffmpeg is not installed. Some features may not work.${NC}"
    echo "Install it with: brew install ffmpeg (macOS)"
fi

# Change to the vhs directory
cd "$(dirname "$0")"

# Build redisctl if needed
if [ ! -f "../target/release/redisctl" ]; then
    echo -e "${YELLOW}Building redisctl...${NC}"
    cd ..
    cargo build --release
    cd vhs
fi

# Make sure redisctl is in PATH
export PATH="../target/release:$PATH"

# Array of tape files to process
TAPES=(
    "quick-start.tape"
    "profile-management.tape"
    "enterprise-demo.tape"
    "cloud-demo.tape"
    "async-operations.tape"
)

# Process each tape file
for tape in "${TAPES[@]}"; do
    if [ -f "$tape" ]; then
        echo -e "${GREEN}Processing:${NC} $tape"
        vhs "$tape" || {
            echo -e "${RED}Failed to process $tape${NC}"
            continue
        }

        # Extract output filename from tape file
        output=$(grep "^Output" "$tape" | awk '{print $2}')
        if [ -f "$output" ]; then
            size=$(du -h "$output" | cut -f1)
            echo -e "  ✓ Generated $output (${size})"
        fi
    else
        echo -e "${YELLOW}Skipping:${NC} $tape (not found)"
    fi
done

echo
echo -e "${GREEN}✨ Demo generation complete!${NC}"
echo
echo "Generated files:"
ls -lh img/*.gif 2>/dev/null || echo "No GIF files generated"

echo
echo "To view a demo:"
echo "  open vhs/img/quick-start.gif"
echo
echo "To create a new demo:"
echo "  1. Create a new .tape file"
echo "  2. Run: vhs your-demo.tape"
