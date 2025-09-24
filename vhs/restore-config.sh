#!/bin/bash

# Restore original config after demos

CONFIG_DIR="$HOME/.config/redisctl"
CONFIG_FILE="$CONFIG_DIR/config.toml"
BACKUP_FILE="$CONFIG_DIR/config.toml.backup"

if [ -f "$BACKUP_FILE" ]; then
    echo "Restoring original config from $BACKUP_FILE"
    mv "$BACKUP_FILE" "$CONFIG_FILE"
    echo "Original config restored"
else
    echo "No backup found at $BACKUP_FILE"
fi
