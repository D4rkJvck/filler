#!/bin/bash

source "$(dirname $0)/utils.sh"

# Remove unecessary files and folders
remove "filler.zip" "docker_image" "solution"

# Clean build artifacts
log "Cleaning build artifacts"
cargo clean -v --target-dir solution
cargo clean -v --target target
