#!/bin/sh

install_dir="$HOME/.local/share"
# Move binary to install directory
mkdir -p "$install_dir/leash-cpu-limit"
# Binary located at target/release/leash-cpu-limit
release_binary="$(pwd)/target/release/leash-cpu-limit"
# Desktop file leash-cpu-limit.desktop
desktop_file="$(pwd)/src/leash-cpu-limit.desktop"
# Verify that the binary exists
if [ ! -f "$release_binary" ]; then
    echo "Binary not found at $release_binary, run 'cargo build --release' first"
    exit 1
fi
# Copy binary to install directory
cp "$release_binary" "$install_dir/leash-cpu-limit"
# Copy desktop file to ~/.local/share/applications/
cp "$desktop_file" "$HOME/.local/share/applications/"