#!/bin/bash

# Setup script for VHS demos
# This script backs up the existing config and sets up a clean demo environment

CONFIG_DIR="$HOME/.config/redisctl"
CONFIG_FILE="$CONFIG_DIR/config.toml"
BACKUP_FILE="$CONFIG_DIR/config.toml.backup"
DEMO_CONFIG="$(dirname "$0")/demo-config.toml"

# Create config directory if it doesn't exist
mkdir -p "$CONFIG_DIR"

# Backup existing config if it exists
if [ -f "$CONFIG_FILE" ]; then
    echo "Backing up existing config to $BACKUP_FILE"
    cp "$CONFIG_FILE" "$BACKUP_FILE"
fi

# Copy demo config
echo "Setting up demo config"
cp "$DEMO_CONFIG" "$CONFIG_FILE"

echo "Demo environment ready. Run your VHS demos now."
echo "To restore: mv $BACKUP_FILE $CONFIG_FILE"
