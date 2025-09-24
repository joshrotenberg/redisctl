#!/bin/bash
set -e

echo "🎬 Generating clean redisctl demos..."

# Check for VHS
if ! command -v vhs &> /dev/null; then
    echo "❌ Error: VHS is not installed"
    echo "Install with: brew install vhs"
    exit 1
fi

# Check for redisctl
if ! command -v redisctl &> /dev/null; then
    echo "❌ Error: redisctl is not installed"
    echo "Install with: cargo install redisctl"
    exit 1
fi

# Setup clean demo environment
echo "📝 Setting up demo environment..."
./setup-demo-env.sh

echo "🎥 Generating demos..."

# Generate each demo
for tape in *.tape; do
    if [ -f "$tape" ]; then
        echo "  Recording: $tape"
        vhs "$tape" || echo "    ⚠️  Failed: $tape"
    fi
done

echo "🔄 Restoring original config..."
./restore-config.sh

echo "✨ Done! Check vhs/img/ for generated demos."
echo ""
echo "Generated files:"
ls -lh img/*.gif 2>/dev/null || echo "  No GIF files found"
