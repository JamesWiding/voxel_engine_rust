#!/bin/bash

# Define your source and destination paths
SRC_DIR="/home/james/freeminer"
DEST_DIR="/media/james/James USB 3/freeminer"

echo "Scanning for symbolic links in $SRC_DIR..."

find "$SRC_DIR" -type l | while read -r src_link; do
    # Get the relative path from the source root
    rel_path="${src_link#$SRC_DIR/}"
    dest_link="$DEST_DIR/$rel_path"
    
    # Ensure the target parent directory exists on the USB
    mkdir -p "$(dirname "$dest_link")"
    
    # If a broken or old file exists at the destination, remove it
    if [ -e "$dest_link" ] || [ -L "$dest_link" ]; then
        rm -f "$dest_link"
    fi
    
    # Copy the symlink itself (-P / --no-dereference ensures we copy the link, not the target content)
    cp --no-dereference "$src_link" "$dest_link"
    echo "Copied symlink: $rel_path"
done

echo "Symlink recovery complete!"
