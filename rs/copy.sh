#!/bin/bash

set -e

# Usage: ./copy.sh /dev/sdb2

DEVICE="$1"
MOUNT_POINT="/mnt/sdcard"

if [ -z "$DEVICE" ]; then
  echo "Usage: $0 /dev/sdXn"
  exit 1
fi

# Create mount point
sudo mkdir -p "$MOUNT_POINT"

# Mount
echo "Mounting $DEVICE..."
sudo mount "$DEVICE" "$MOUNT_POINT"

# Files to copy
FILES=(
  "blink.img"
  "config.txt"
)

# Copy files
for FILE in "${FILES[@]}"; do
  echo "Copying $FILE..."
  sudo cp "$FILE" "$MOUNT_POINT/"
done

# Sync
echo "Syncing..."
sync

# Unmount
echo "Unmounting..."
sudo umount "$MOUNT_POINT"

echo "Done."
